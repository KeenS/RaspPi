arch ?= arm
build_dir := build/$(arch)
kernel := build/$(arch)/kernel.img

target ?= $(arch)-unknown-linux-gnueabihf
rust_lib := target/$(target)/debug/rspi_gpio.o
xcc ?= arm-none-eabi-gcc
xobjcopy ?= arm-none-eabi-objcopy

.PHONY: cargo all clean run iso

all: $(kernel)

clean:
	@rm -r build


$(kernel): cargo $(rust_lib)
	@mkdir -p $(build_dir)
	@$(xcc) -O2 -mfpu=vfp -mfloat-abi=hard -march=armv6zk -mtune=arm1176jzf-s -nostartfiles $(rust_lib) -o $(build_dir)/kernel.elf
	@$(xobjcopy) $(build_dir)/kernel.elf -O binary $(kernel)

cargo:
	@cargo rustc --target $(target) -- --emit=obj -O
