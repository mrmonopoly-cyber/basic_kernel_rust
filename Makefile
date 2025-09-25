ISO ?= damos.iso
ISOD ?= isodir
BIN ?= $(ISOD)/boot/damos.bin
TARGET ?= target/x86_64-unknown-none/release/basic_kernel
FEATURES ?= 
GRUB_FILE ?= --is-x86-multiboot2

all: update run

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
	rm -f $(BIN)
	rm -f $(ISO)
	cargo clean
