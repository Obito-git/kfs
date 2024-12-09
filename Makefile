arch ?= x86-i386
kernel := build/kfs-kernel-$(arch).bin
iso := build/os-$(arch).iso
target ?= $(arch)-kfs
target_rust_config_file := arch/$(arch)/$(target).json
rust_os := target/$(target)/release/libkfs_1.a

linker_script := arch/$(arch)/linker.ld
grub_cfg := arch/$(arch)/grub.cfg
assembly_source_files := $(wildcard arch/$(arch)/*.asm)
assembly_object_files := $(patsubst arch/$(arch)/%.asm, \
	build/arch/$(arch)/%.o, $(assembly_source_files))

all: $(iso)

clean:
	@rm -rf build/arch
	@cargo clean

fclean: clean
	@rm -rf build

run: $(iso)
	@qemu-system-i386 -cdrom $(iso)

rerun: fclean run

iso: $(iso)

$(iso): $(kernel) $(grub_cfg)
	@grub-file --is-x86-multiboot2 $(kernel)
	@mkdir -p build/isofiles/boot/grub
	@cp $(kernel) build/isofiles/boot/kfs.bin
	@cp $(grub_cfg) build/isofiles/boot/grub
	@grub-mkrescue --compress xz -o $(iso) build/isofiles 2> /dev/null
	@rm -r build/isofiles

$(kernel): kernel $(rust_os) $(assembly_object_files) $(linker_script)
	@ld -m elf_i386 -n --gc-sections -T $(linker_script) -o $(kernel) \
    	build/arch/$(arch)/multiboot_header.o \
    	$(filter-out build/arch/$(arch)/multiboot_header.o, $(assembly_object_files)) \
    	$(rust_os)

kernel:
	@cargo build --release --target $(target_rust_config_file)

build/arch/$(arch)/%.o: arch/$(arch)/%.asm
	@mkdir -p $(shell dirname $@)
	@nasm -felf32 $< -o $@

re: clean all

.PHONY: all clean run iso kernel fclean rerun

