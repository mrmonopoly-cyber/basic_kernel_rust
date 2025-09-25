#User inputs
ARCH ?= $(shell uname -m)
BOOT ?= multiboot_v1

# Makefile parameters managed by the script
ISO ?= damos.iso
ISOD ?= isodir
BIN ?= $(ISOD)/boot/damos.bin
TARGET ?= target/x86_64-unknown-none/release/basic_kernel

ARCH_FEATURES =
BOOT_FEATURES =
FEATURES ?= $(ARCH_FEATURES)$(BOOT_FEATURES)

GRUB_FILE ?= --is-x86-multiboot2

all: check_arch check_boot update run

check_arch:
ifeq ($(ARCH) , x86)
	$(info supported arch: $(ARCH))
else ifeq ($(ARCH) , amd64)
	$(info supported arch: $(ARCH))
else
	$(error "unsopported arch: $(ARCH)")
endif

check_boot:
ifeq ($(BOOT), multiboot_v1)
	$(info supported boot: $(BOOT))
else ifeq ($(BOOT), multiboot_v2)
	$(info supported boot: $(BOOT))
else
	$(error "unsopported boot: $(BOOT)")
endif

ifeq ($(ARCH), x86)
ARCH_FEATURES =x86,
else ifeq ($(ARCH), amd64)
ARCH_FEATURES =amd64,
endif


ifeq ($(BOOT), multiboot_v1)
BOOT_FEATURES =multiboot_v1
GRUB_FILE = --is-x86-multiboot
ISOD = isodir
else ifeq ($(BOOT), multiboot_v2)
BOOT_FEATURES =multiboot_v2
GRUB_FILE = --is-x86-multiboot2
ISOD = isodir2
endif


update:
	rm -f $(ISO)
	rm -f $(BIN)

run: $(ISO) 
	qemu-system-x86_64 -cdrom $(ISO)

$(ISO): $(BIN)
	$(shell  if ! grub-file $(GRUB_FILE) $(BIN); then echo "damos.bin is not valid x86-multiboot format"; fi)
	grub-mkrescue -o $(ISO) $(ISOD)


$(BIN): 
	cargo b --release --features=$(FEATURES)
	cp $(TARGET) $(BIN)

clean:
	$(shell bash -c "rm -f ./isodir[0-9]/boot/*.bin")
	cargo clean
