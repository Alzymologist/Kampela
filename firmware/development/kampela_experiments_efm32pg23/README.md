# Overview  

This is out first attempt to use `efm32pg23` chip for Kampela purposes.  

It mostly deals with security element (SE) basic functionality and with creating and using the keys from seeds.

The code is build for [`EFM32PG23B310F512IM48-C`](https://www.silabs.com/mcu/32-bit-microcontrollers/efm32pg23-series-2/device.efm32pg23b310f512im48) and tested with [`PG23-PK2504A`](https://www.silabs.com/development-tools/mcu/32-bit/efm32pg23-pro-kit?tab=overview) devboard.  

What it does:  

- creates AES-GCM wrapped key that will be used to encrypt and decrypt the user secret
- encrypts user secret, decrypts it back using the key (also needs internal IV data and user pincode - both are hardcoded at the moment)
- creates random `u8` slices with given length, to be used, for example, as entropy when a new seed is generated
- generates seed phrase from entropy and entropy from seed phrase (following `tiny-bip39`, draft-rewritten for `no_std`)
- generates seed from seed phrase (following `substrate-bip39`, draft-rewritten for `no_std`)
- cuts derivation data received as `&str` info separate junctions (following `sp_core`, draft-rewritten for `no_std`)
- creates `Ed25519` keypair, makes derivations (hard-only), signs, verifies signature (uses `sp_core` in `full_crypto` mode)
- creates `Sr25519` keypair, makes derivations (both hard and soft), signs, verifies signature (uses `schnorrkel` with external rng); note: `sp_core` in `full_crypto` mode does not support external rng thus causing the soft derivations and signing to panic)
- creates `Ecdsa` keypair, makes derivations (hard-only), signs (uses `sp_core` in `full_crypto` mode, verification apparently requires too much memory (?!) and is not yet added; maybe will switch to other ecdsa crate eventually).

- blinks with devboard left and right LED lights to indicate error codes and/or to signal program start and completion

# Prerequisites  

- `thumbv8m.main-none-eabihf` target, for Cortex-M33 with FPU
- nightly toolchain for this target
- [`arm-none-eabi-binutils`](https://archlinux.org/packages/community/x86_64/arm-none-eabi-binutils/) on ArchLinux
- [`Simplicity Commander`](https://community.silabs.com/s/article/simplicity-commander?language=en_US)


# Notes on build and testing  

1. The allocator used here currently (`embedded-alloc`) requires nightly toolchain.  

Switch to nightly with your preferred method, e.g.  

(while in `/app` folder)

`$ rustup override set nightly`  

2. While in `/app` folder, build the app:  

`$ cargo build --release`

File `app` in `/app/target/thumbv8m.main-none-eabihf/release/` is the `.elf` file.

Rename it into `app.elf`.

3. Transform `.elf` file into `.bin` file, e.g.  

(while in `/app/target/thumbv8m.main-none-eabihf/release/`, on ArchLinux)

`$ arm-none-eabi-objcopy -O binary app.elf app.bin`

4. Upload the `.bin` file. `Simplicity Commander` does the job.

5. If the `.bin` file loaded correctly, the program will start running.  

Devboard signals that it is alive and did not panic yet by occasional blinking.  

Currently, it blinks left LED 10 times on start, and right LED 10 times on finish.  

If a panic occurs, the right LED blinking will never be observed.  

So far the debugging was straightforward enough so that the blinking signals were sufficient for it.  
