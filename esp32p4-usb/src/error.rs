//! USB stack error types.

/// Result alias used throughout the stack.
pub type UsbResult<T> = Result<T, UsbError>;

/// Errors returned by USB stack operations.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[non_exhaustive]
pub enum UsbError {
    /// The operation cannot make progress right now: no packet is available
    /// for `read`, or the previous packet has not left for `write`.
    WouldBlock,
    /// The caller's buffer is too small, or the payload exceeds the endpoint's
    /// maximum packet size.
    BufferOverflow,
    /// The endpoint address was not allocated on this bus.
    InvalidEndpoint,
    /// Every endpoint of the requested direction is already in use.
    EndpointOverflow,
    /// The controller has no FIFO space left for another endpoint.
    EndpointMemoryOverflow,
    /// The bus implementation does not support this operation.
    Unsupported,
    /// The operation is not valid in the current state.
    InvalidState,
    /// A SETUP packet or a descriptor could not be parsed.
    ParseError,
}
