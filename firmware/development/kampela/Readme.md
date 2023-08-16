# Requirements
We assume that you have linux.

Set of packages as well as commands could vary depending on your distribution.
Following is valid for ArchLinux.

Install rustup and ARM toolchain:
```sh
[sudo] pacman -S rustup arm-none-eabi-gcc arm-none-eabi-binutils
rustup update
rustup default stable
```

Also you will need a program for flashing.

For Pilkki flasher look [here](https://github.com/Alzymologist/pilkki).

For Segger you need to install [simplicity-commander](https://www.silabs.com/developers/mcu-programming-options).
In case of ArchLinux there is [AUR package](https://aur.archlinux.org/packages/simplicity-commander).

# Preparations

```sh
rustup target add thumbv8m.main-none-eabihf
cargo install flip-link
```


# Build

```sh
cargo build --release
```

## Flashing

In case of Pilkki:
```sh
./binarize.sh --pilkki
```

And in case of Segger
```sh
./binarize.sh --segger
```



