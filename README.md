toy code that blinks LED on Raspberry Pi B with Rust

# How
## Prepare
* Rust Cross compiler to gnueabihf.
  `multirust add-target nightly arm-unknown-linux-gnueabihf`
* Bootloaders, `bootcode.bin` and `start.elf` from [here](https://github.com/raspberrypi/firmware/tree/master/boot)
* An SD card that is formatted with fat32

and, copy bootcode.bin and start.elf to SD card

## Build

```
make
```

then you will get build/arm/kernel.img

## Run

1. copy kernel.img to the SD card
2. mount the SD card to RSPi
3. power RSPi and LED (green) will blink

# LICENSE
BSD
