ISO ?= damos.iso
BIN ?= ./target/x86_64-unknown-none/release/basic_uefi

all: run

run: $(ISO) 
	qemu-system-x86_64 -cdrom $(ISO)

$(ISO):  $(BIN)
	cp $(BIN) ./isodir/boot/damos.bin
	grub-mkrescue -o damos.iso ./isodir

$(BIN): 
	cargo b --release

clean:
	rm -f ./isodir/boot/damos.bin
	rm -f $(ISO)
	cargo clean
