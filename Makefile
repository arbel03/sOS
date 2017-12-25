rust_os := target/i386-sos/debug/libsos.a

assembly_source_files := $(wildcard src/boot/*.asm)
assembly_object_files := $(patsubst src/boot/%.asm, \
	build/boot/%.o, $(assembly_source_files))

iso := build/os.iso
kernel := build/kernel.bin

.PHONY: clean all run cargo

all: run

run: $(iso)
	@qemu-system-i386 -drive file=$<,format=raw

$(kernel): cargo $(rust_os) $(assembly_object_files)
	@ld -n --gc-sections -m elf_i386 -T linker.ld -o $@ $(assembly_object_files) $(rust_os)

cargo:
	@xargo build --target i386-sos

clean:
	@rm -rf build

build/boot/%.o: src/boot/%.asm
	@mkdir -p $(dir $@)
	@nasm -f elf32 -o $@ $<

include bootloader/Makefile
