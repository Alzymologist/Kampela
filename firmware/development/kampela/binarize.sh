#!/bin/bash

set -e

echo "Building..."

cargo build --release

BUILD_DIRECTORY="target/thumbv8m.main-none-eabihf"
EXECUTABLE="$BUILD_DIRECTORY/release/app"
FIRMWARE="$BUILD_DIRECTORY/release/app.bin"

echo "Preparing binary file..."

arm-none-eabi-objcopy -O binary "$EXECUTABLE" "$FIRMWARE"

echo "Flashing..."

if [[ -z "$1" || "$1" == "--pilkki" ]]; then
pilkki write -i "$FIRMWARE"
elif [ "$1" == "--segger" ]; then
commander flash "$FIRMWARE" --device EFM32PG23B200F512IM40 --address 0x08000000
else
echo "What means \"$1\"? Usage:"
echo "$0 [--pilkki|--segger]"
exit -1
fi