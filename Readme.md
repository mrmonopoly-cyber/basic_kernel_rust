# Basic Rust Kernel

This is only a small project to study and understand how to make a kernel in rust.
At the current moment supports :
 - x86
And for booting options:
 - multiboot_v1 
 - multiboot_v2

# Building

To build and run the kernel in quemu you use make.
The makefile in the project has the following User parameters responsible to configure the kernel:

- ARCH: set the architecture. Possible values : x86, amd64
- BOOT: set the boot mode. Possible values: multiboot_v1, multiboot_v2

To build the kernel use following command:
```sh
    make ARCH=<YOUR ARCHITECTURE> BOOT=<YOUR BOOT SYSTEM>
```

# Run

The makefile will also run the kernel in an emulated environemnt with qemu

