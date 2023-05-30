#!/bin/bash

cargo build --release
mv target/thumbv8m.main-none-eabihf/release/app target/thumbv8m.main-none-eabihf/release/app.elf
arm-none-eabi-objcopy -O binary target/thumbv8m.main-none-eabihf/release/app.elf target/thumbv8m.main-none-eabihf/release/app.bin
commander flash target/thumbv8m.main-none-eabihf/release/app.bin --device EFM32PG23B200F512IM40 --address 0x08000000
