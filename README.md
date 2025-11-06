# Poltergeist

A Sony DualSense controller driver implementation

```log
~/Projects/poltergeist> udevadm monitor
monitor will print the received events for:
UDEV - the event which udev sends out after rule processing
KERNEL - the kernel uevent

UDEV  [201743.351227] add      /devices/pci0000:00/0000:00:02.1/0000:03:00.0/0000:04:08.0/0000:06:00.0/0000:07:0c.0/0000:67:00.0/usb3/3-6/3-6:1.0/bluetooth/hci0/hci0:256 (bluetooth)
UDEV  [201743.540193] add      /devices/virtual/misc/uhid/0005:054C:0CE6.0014 (hid)
```

## Notes

Sony provides a `hid_playstation` kernel driver that we need in order to interact with the driver.

## Useful Links

- <https://github.com/nowrep/dualsensectl>
- <https://crates.io/crates/hidapi>
- <https://github.com/libusb/hidapi>
- <https://github.com/Smithay/udev-rs>
- <https://www.freedesktop.org/software/systemd/man/latest/sd-device.html>
- <https://docs.kernel.org/hid/uhid.html>
- <https://github.com/nix-rust/nix>
