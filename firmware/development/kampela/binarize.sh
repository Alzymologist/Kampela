#!/bin/bash

set -e

cargo build --release
arm-none-eabi-objcopy -O binary target/thumbv8m.main-none-eabihf/release/app target/thumbv8m.main-none-eabihf/release/app.bin
commander flash target/thumbv8m.main-none-eabihf/release/app.bin --device EFM32PG23B200F512IM40 --address 0x08000000
