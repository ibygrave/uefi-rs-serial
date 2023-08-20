# uefi-rs-serial

Example use of a serial port from a UEFI application.

## Description

Follows the exmaples in the
[Rust UEFI Book](https://rust-osdev.github.io/uefi-rs/HEAD/)
and adds code to locate, open and write to a serial device.

Tested in
[QEMU](https://www.qemu.org/)
with
[OVMF firmware](https://github.com/tianocore/tianocore.github.io/wiki/OVMF).

## Getting Started

See [the tutorial](https://rust-osdev.github.io/uefi-rs/HEAD/tutorial/introduction.html)
up to [Running in a VM](https://rust-osdev.github.io/uefi-rs/HEAD/tutorial/vm.html).

Add two serial ports to the VM:

```
qemu-system-x86_64 -drive if=pflash,format=raw,readonly=on,file=OVMF_CODE.fd -drive if=pflash,format=raw,file=OVMF_VARS.fd -drive format=raw,file=fat:rw:esp -serial vc -serial vc
```

The `Hello world!` text should be seen on the second serial port only.

On the first boot of the UEFI Application both serial ports will also show the boot console.
At the OVMF configuration menu the boot console configuation can be changed to disable
the console on the second serial port.
However I was unable to make this configuration persisitent.
This issue looks relevant:
[QEMU #745](https://gitlab.com/qemu-project/qemu/-/issues/745).
