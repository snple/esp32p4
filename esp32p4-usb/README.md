# esp32p4-usb

USB 2.0 **device** stack for the ESP32-P4's two on-chip OTG controllers.

The MCU-independent part — bus trait, control pipe, device state machine,
CDC-ACM and HID classes — lives in the shared [`usb-stack`](https://crates.io/crates/usb-stack)
crate and is re-exported here. This crate adds the Synopsys DesignWare OTG
driver:

```text
  application / class          class::cdc_acm, your own UsbClass
  ────────────────────────     (usb-stack)
  device::UsbDevice            enumeration, standard requests, EP0
  ────────────────────────     (usb-stack)
  bus::UsbBus                  trait: allocate, poll, read, write
  ────────────────────────     (usb-stack)
  p4::UsbFs / p4::UsbHs        DWC OTG slave-mode driver (this crate)
```

| Module | Contents |
|--------|----------|
| `p4` | `UsbFs` / `UsbHs`, the full-speed (12 Mbit/s, GPIO26/27) and high-speed (480 Mbit/s, internal UTMI+ PHY) drivers |
| `dwc` | DesignWare OTG core register definitions |
| everything else | re-exported from `usb-stack` |

The driver is fully synchronous: no interrupts, no DMA, no `async`.
[`UsbDevice::poll`] moves packets in and out of the controller FIFOs and
dispatches them to the classes, so it has to be called from the main loop every
few milliseconds.

## Adding it to a project

```toml
[dependencies]
esp32p4-usb = "0.1"
```

```rust,ignore
use esp32p4_hal::pac::Peripherals;
use esp32p4_usb::class::cdc_acm::CdcAcm;
use esp32p4_usb::p4::{FsResources, UsbFs};
use esp32p4_usb::{UsbBusAllocator, UsbDeviceBuilder};
```

## Features

- Both controllers: full speed (GPIO26/27) and high speed (internal PHY)
- Standard device requests, string/configuration descriptors, composite
  devices with interface association descriptors
- `class::cdc_acm::CdcAcm` virtual serial port — 64-byte bulk packets on the FS
  core, 512-byte on the HS core
- Endpoint handles for custom classes (`bulk`, `interrupt`, `isochronous`)
- `defmt` (off by default) — log through `defmt`

## License

Licensed under either of Apache-2.0 or MIT, at your option.
