# esp32p4-usb

USB 2.0 **device and host** stack for the ESP32-P4's two on-chip OTG controllers.

Both protocol layers — the device stack (bus trait, control pipe, device state
machine, CDC-ACM and HID classes) and the host stack (enumeration, control
transfers, bulk and interrupt pipes, HID) — live in the shared
[`usb-stack`](https://crates.io/crates/usb-stack) crate and are re-exported here.
What this crate adds is the Synopsys DesignWare OTG driver for both directions:

```text
  device mode                          host mode
  ──────────────────────────           ──────────────────────────
  class::cdc_acm, UsbClass             host::class::hid::Keyboard
  device::UsbDevice                    host::Device / host::Pipe
  bus::UsbBus                          host::HostController
  ──────────────────────────           ──────────────────────────
  (usb-stack, re-exported)             (usb-stack, re-exported)
  UsbFs / UsbHs                        UsbHostFs / UsbHostHs
  ──────────────────────────           ──────────────────────────
  (this crate)                         (this crate)
```

| Module | Contents |
|--------|----------|
| crate root | the four drivers (`UsbFs` / `UsbHs`, `UsbHostFs` / `UsbHostHs`) and the `FsResources` / `HsResources` bundles they take |
| `resources` | which core to drive, plus the clock, reset, PHY and pad set-up shared by both directions |
| `dwc` | DesignWare OTG core register definitions, shared by both directions |
| everything else | re-exported from `usb-stack` (`device`, `host`, `endpoint`, `descriptor`, `request`) |

Both directions are fully synchronous: no interrupts, no DMA, no `async`. The
device driver moves packets when `UsbDevice::poll` is called; the host driver
blocks in `Host::enumerate`, `Device::control` and `Pipe::read`/`write` until the
transaction completes.

A core belongs to one direction at a time: the device and host drivers share the
"core taken" flag, so initialising both panics.

## Adding it to a project

```toml
[dependencies]
esp32p4-usb = "0.1"
```

Device mode:

```rust,ignore
use esp32p4_hal::pac::Peripherals;
use esp32p4_usb::class::cdc_acm::CdcAcm;
use esp32p4_usb::{FsResources, UsbFs};
use esp32p4_usb::{UsbBusAllocator, UsbDeviceBuilder};

let allocator = UsbBusAllocator::new(UsbFs::new(FsResources {
    clkrst: &dp.HP_SYS_CLKRST,
    lp_aon_clkrst: &dp.LP_AON_CLKRST,
    usb_wrap: &dp.USB_WRAP,
    gpio: &dp.GPIO,
    io_mux: &dp.IO_MUX,
}));
let mut serial = CdcAcm::new(&allocator);
let mut device = UsbDeviceBuilder::new(&allocator, 0x303a, 0x1001)
    .product("ESP32-P4 serial")
    .build();
```

Host mode:

```rust,ignore
use esp32p4_hal::pac::Peripherals;
use esp32p4_usb::host::Host;
use esp32p4_usb::host::class::hid::Keyboard;
use esp32p4_usb::{FsResources, UsbHostFs};

let controller = UsbHostFs::new(FsResources {
    clkrst: &dp.HP_SYS_CLKRST,
    lp_aon_clkrst: &dp.LP_AON_CLKRST,
    usb_wrap: &dp.USB_WRAP,
    gpio: &dp.GPIO,
    io_mux: &dp.IO_MUX,
});
let host = Host::new(&controller);

let device = host.enumerate()?;               // resets the port and enumerates
device.set_configuration(1)?;
let keyboard = Keyboard::open(&device)?;
```

## Features

- Both controllers, both directions: full speed (GPIO26/27) and high speed
  (internal PHY)
- Device: standard device requests, string/configuration descriptors, composite
  devices with interface association descriptors, CDC-ACM, HID, GS_USB,
  i2c-tiny-usb and UAC 1.0 class drivers
- Host: root port reset and enumeration, control/bulk/interrupt transfers at low,
  full and high speed, descriptor parsing, string decoding, a HID boot
  keyboard driver and USB 2.0 hubs (FS/LS devices behind a high-speed hub need
  `new_fsls_only`, see below)
- `defmt` (off by default) — log through `defmt`

Host mode supports hubs, including cascades on a full-speed link. A full-speed
hub works on the full-speed core and on the high-speed core when it is
initialised with `UsbHostHs::new_fsls_only`; high-speed devices behind a
high-speed hub work with the ordinary `UsbHostHs::new`. A FS/LS device behind a
high-speed hub would need the hub's transaction translator, which this driver
does not implement, so it is rejected with `HostError::Unsupported`. Isochronous
transfers and SRP/HNP are not implemented either; the chip has no VBUS switch,
so the board has to power the port and the hub.

## License

Licensed under either of Apache-2.0 or MIT, at your option.
