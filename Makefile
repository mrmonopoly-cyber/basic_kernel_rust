ISO ?= damos.iso
BIN ?= isodir/boot/damos.bin
TARGET ?= target/x86_64-unknown-none/release/basic_kernel

all: update run

update:
	rm -f $(ISO)
	rm -f ./isodir/boot/damos.bin

run: $(ISO) 
	qemu-system-x86_64 -cdrom $(ISO)

$(ISO):  $(BIN)
	$(shell  if ! grub-file --is-x86-multiboot $(BIN); then echo "damos.bin is not valid x86-multiboot format"; fi)
	grub-mkrescue -o damos.iso ./isodir


$(BIN): 
	cargo b --release
	cp $(TARGET) $(BIN)

clean:
	rm -f $(BIN)
	rm -f $(ISO)
	cargo clean
