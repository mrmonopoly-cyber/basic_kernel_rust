ISO ?= damos.iso
BIN ?= ./isodir/boot/damos.bin

all: update run

update:
	rm -f $(ISO)
	rm -f ./isodir/boot/damos.bin

run: $(ISO) 
	qemu-system-x86_64 -cdrom $(ISO)

$(ISO):  $(BIN)
	grub-mkrescue -o damos.iso ./isodir

$(BIN): 
	cargo b --release
	cp ./target/x86_64-unknown-none/release/basic_uefi $(BIN)

clean:
	rm -f $(BIN)
	rm -f $(ISO)
	cargo clean
