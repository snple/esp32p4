//! Control-transfer (endpoint 0) state machine.
//!
//! [`ControlPipe`] drives the EP0 handshake: it collects SETUP packets, moves
//! the data stage (multi-packet, with the zero-length-packet rule), and closes
//! the status stage. It is deliberately independent of the peripheral: all
//! hardware access goes through the [`UsbBus`] passed in on each call.
//!
//! [`ControlIn`] and [`ControlOut`] are short-lived handles given to
//! [`UsbClass`](crate::class::UsbClass) callbacks. A callback either accepts the
//! request (supplying or consuming the data) or rejects it, which stalls EP0.
//! Dropping the handle without doing either lets the next handler try.

pub mod request;

pub use request::{Recipient, Request, RequestType, feature, standard_request};

use core::cmp::min;

use crate::bus::UsbBus;
use crate::endpoint::{Direction, EndpointAddress};
use crate::error::{UsbError, UsbResult};

/// Staging buffer size for control transfers.
///
/// Large enough for a 256-byte configuration descriptor plus a little slack;
/// longer descriptors are truncated to `wLength` anyway.
pub const CONTROL_BUF_SIZE: usize = 256;

/// Where the control pipe currently is.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum State {
    /// No transfer in progress.
    Idle,
    /// IN data stage: `offset` of `len` bytes sent.
    DataIn { offset: usize, len: usize },
    /// The last packet exactly filled the endpoint, so a ZLP must follow.
    DataInZlp,
    /// All data (and any ZLP) sent; waiting for the status OUT.
    DataInLast,
    /// Waiting for the status OUT to complete.
    StatusOut,
    /// The request is parsed; the application has to accept or reject it.
    CompleteIn(Request),
    /// OUT data stage: the host is sending `len` bytes, `offset` arrived.
    DataOut {
        request: Request,
        offset: usize,
        len: usize,
    },
    /// The OUT data stage is done; the application has to accept or reject.
    CompleteOut { len: usize },
    /// The status IN (ZLP) was queued; wait for it to be collected.
    StatusIn,
    /// EP0 is stalled after a rejected or malformed request.
    Error,
}

/// Endpoint-0 state machine and staging buffer.
pub struct ControlPipe {
    state: State,
    buffer: [u8; CONTROL_BUF_SIZE],
    max_packet_size: u16,
}

impl ControlPipe {
    /// Create an idle pipe for an EP0 with `max_packet_size` bytes per packet.
    pub const fn new(max_packet_size: u16) -> Self {
        Self {
            state: State::Idle,
            buffer: [0; CONTROL_BUF_SIZE],
            max_packet_size,
        }
    }

    /// Drop any in-flight transfer (called on bus reset).
    pub fn reset(&mut self) {
        self.state = State::Idle;
    }

    /// Whether the application still has to accept or reject the request.
    pub fn waiting_for_response(&self) -> bool {
        matches!(self.state, State::CompleteIn(_) | State::CompleteOut { .. })
    }

    /// The OUT data received so far.
    pub fn data(&self) -> &[u8] {
        match self.state {
            State::CompleteOut { len } => &self.buffer[..len],
            State::DataOut { offset, .. } => &self.buffer[..offset],
            _ => &[],
        }
    }

    // ── Bus events ──────────────────────────────────────────────────────────

    /// Handle a SETUP packet on EP0.
    ///
    /// Returns the request when it has no OUT data stage and can be dispatched
    /// immediately. Requests with an OUT data stage are dispatched once the
    /// data arrived, from [`handle_out`](Self::handle_out).
    pub fn handle_setup<B: UsbBus>(&mut self, bus: &B) -> Option<Request> {
        let mut packet = [0u8; 8];
        match bus.read(EndpointAddress::EP0_OUT, &mut packet) {
            Ok(8) => {}
            _ => return None,
        }
        let request = Request::parse(&packet).ok()?;

        // A new SETUP aborts whatever was in flight.
        bus.set_stalled(EndpointAddress::EP0_IN, false);
        bus.set_stalled(EndpointAddress::EP0_OUT, false);

        if request.direction == Direction::Out {
            if request.length == 0 {
                self.state = State::CompleteOut { len: 0 };
                return Some(request);
            }
            if request.length as usize > self.buffer.len() {
                self.stall(bus);
                return None;
            }
            self.state = State::DataOut {
                request,
                offset: 0,
                len: request.length as usize,
            };
            None
        } else {
            self.state = State::CompleteIn(request);
            Some(request)
        }
    }

    /// Handle an OUT packet on EP0 during a control transfer.
    pub fn handle_out<B: UsbBus>(&mut self, bus: &B) -> UsbResult<Option<Request>> {
        match self.state {
            State::DataOut {
                request,
                ref mut offset,
                len,
            } => {
                let n = match bus.read(EndpointAddress::EP0_OUT, &mut self.buffer[*offset..]) {
                    Ok(n) => n,
                    Err(UsbError::WouldBlock) => return Ok(None),
                    Err(e) => {
                        self.stall(bus);
                        return Err(e);
                    }
                };
                *offset += n;
                if *offset >= len {
                    self.state = State::CompleteOut { len: *offset };
                    return Ok(Some(request));
                }
                Ok(None)
            }
            // The host ended the data stage early; consume the packet.
            State::DataIn { .. } | State::DataInZlp | State::DataInLast | State::StatusOut => {
                let mut scratch = [0u8; 64];
                let _ = bus.read(EndpointAddress::EP0_OUT, &mut scratch);
                if self.state == State::StatusOut {
                    self.state = State::Idle;
                }
                Ok(None)
            }
            _ => {
                let mut scratch = [0u8; 64];
                let _ = bus.read(EndpointAddress::EP0_OUT, &mut scratch);
                self.stall(bus);
                Ok(None)
            }
        }
    }

    /// Handle an IN-complete event on EP0.
    ///
    /// Returns `true` once the whole control transfer, including the status
    /// stage, has finished.
    pub fn handle_in_complete<B: UsbBus>(&mut self, bus: &B) -> UsbResult<bool> {
        match self.state {
            State::DataIn { offset, len } => {
                self.send_chunk(bus, offset, len)?;
                Ok(false)
            }
            State::DataInZlp => {
                bus.write(EndpointAddress::EP0_IN, &[])?;
                self.state = State::DataInLast;
                Ok(false)
            }
            State::DataInLast => {
                self.state = State::StatusOut;
                Ok(false)
            }
            State::StatusIn => {
                self.state = State::Idle;
                Ok(true)
            }
            _ => Ok(false),
        }
    }

    // ── Responses ───────────────────────────────────────────────────────────

    /// Accept an IN request, copying data out of the caller's closure.
    pub fn accept_in<B: UsbBus>(
        &mut self,
        bus: &B,
        fill: impl FnOnce(&mut [u8]) -> UsbResult<usize>,
    ) -> UsbResult<()> {
        let request = match self.state {
            State::CompleteIn(request) => request,
            _ => return Err(UsbError::InvalidState),
        };
        let len = fill(&mut self.buffer)?;
        if len > self.buffer.len() {
            self.stall(bus);
            return Err(UsbError::BufferOverflow);
        }
        let len = min(len, request.length as usize);
        self.state = State::DataIn { offset: 0, len };
        self.send_chunk(bus, 0, len)
    }

    /// Accept an IN request with pre-built data.
    pub fn accept_with<B: UsbBus>(&mut self, bus: &B, data: &[u8]) -> UsbResult<()> {
        self.accept_in(bus, |buffer| {
            if data.len() > buffer.len() {
                return Err(UsbError::BufferOverflow);
            }
            buffer[..data.len()].copy_from_slice(data);
            Ok(data.len())
        })
    }

    /// Accept an OUT request by sending the zero-length status IN.
    pub fn accept_out<B: UsbBus>(&mut self, bus: &B) -> UsbResult<()> {
        if !matches!(self.state, State::CompleteOut { .. }) {
            return Err(UsbError::InvalidState);
        }
        bus.write(EndpointAddress::EP0_IN, &[])?;
        self.state = State::StatusIn;
        Ok(())
    }

    /// Reject the pending request by stalling both EP0 directions.
    pub fn reject<B: UsbBus>(&mut self, bus: &B) -> UsbResult<()> {
        if !self.waiting_for_response() {
            return Err(UsbError::InvalidState);
        }
        self.stall(bus);
        Ok(())
    }

    // ── Internals ───────────────────────────────────────────────────────────

    fn send_chunk<B: UsbBus>(&mut self, bus: &B, offset: usize, len: usize) -> UsbResult<()> {
        let mps = self.max_packet_size as usize;
        let chunk = min(len - offset, mps);
        let written = bus.write(
            EndpointAddress::EP0_IN,
            &self.buffer[offset..offset + chunk],
        )?;
        let next = offset + written;
        self.state = if next >= len {
            if written == mps {
                State::DataInZlp
            } else {
                State::DataInLast
            }
        } else {
            State::DataIn { offset: next, len }
        };
        Ok(())
    }

    fn stall<B: UsbBus>(&mut self, bus: &B) {
        bus.set_stalled(EndpointAddress::EP0_IN, true);
        bus.set_stalled(EndpointAddress::EP0_OUT, true);
        self.state = State::Error;
    }
}

/// Handle for an incoming control IN request.
///
/// Dropping it without accepting or rejecting passes the request on to the next
/// class in the device's list.
pub struct ControlIn<'a, B: UsbBus> {
    pipe: &'a mut ControlPipe,
    bus: &'a B,
    request: &'a Request,
}

impl<'a, B: UsbBus> ControlIn<'a, B> {
    pub(crate) fn new(pipe: &'a mut ControlPipe, bus: &'a B, request: &'a Request) -> Self {
        Self { pipe, bus, request }
    }

    /// The SETUP packet that started this transfer.
    pub fn request(&self) -> &Request {
        self.request
    }

    /// Accept the request, letting `fill` write the response.
    pub fn accept(self, fill: impl FnOnce(&mut [u8]) -> UsbResult<usize>) -> UsbResult<()> {
        self.pipe.accept_in(self.bus, fill)
    }

    /// Accept the request with a pre-built response.
    pub fn accept_with(self, data: &[u8]) -> UsbResult<()> {
        self.pipe.accept_with(self.bus, data)
    }

    /// Reject the request (stalls EP0).
    pub fn reject(self) -> UsbResult<()> {
        self.pipe.reject(self.bus)
    }
}

/// Handle for an incoming control OUT request.
pub struct ControlOut<'a, B: UsbBus> {
    pipe: &'a mut ControlPipe,
    bus: &'a B,
    request: &'a Request,
}

impl<'a, B: UsbBus> ControlOut<'a, B> {
    pub(crate) fn new(pipe: &'a mut ControlPipe, bus: &'a B, request: &'a Request) -> Self {
        Self { pipe, bus, request }
    }

    /// The SETUP packet that started this transfer.
    pub fn request(&self) -> &Request {
        self.request
    }

    /// The data received in the OUT data stage (empty when `wLength` was 0).
    pub fn data(&self) -> &[u8] {
        self.pipe.data()
    }

    /// Accept the request, completing the status stage.
    pub fn accept(self) -> UsbResult<()> {
        self.pipe.accept_out(self.bus)
    }

    /// Reject the request (stalls EP0).
    pub fn reject(self) -> UsbResult<()> {
        self.pipe.reject(self.bus)
    }
}
