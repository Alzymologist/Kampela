#!/bin/bash

cargo build --release
mv target/thumbv8m.main-none-eabihf/release/app target/thumbv8m.main-none-eabihf/release/app.elf
arm-none-eabi-objcopy -O binary target/thumbv8m.main-none-eabihf/release/app.elf target/thumbv8m.main-none-eabihf/release/app.bin
