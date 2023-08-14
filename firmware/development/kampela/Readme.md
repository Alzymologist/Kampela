Wee are trying to build:

cargo build --release

... and get error that we don't have `thumbv8m.main-none-eabihf` target.
So, we are going to add this target:

rustup target add thumbv8m.main-none-eabihf

And here we can see that wee installed wrong rust package (rust), so we have to remove it and install rustup instead:

pacman -R rust

pacman -S rustup arm-none-eabi-gcc
rustup update
rustup default stable

Also we installed arm-none-eabi-gcc which is required for build.
And finally:
rustup target add thumbv8m.main-none-eabihf
cargo install flip-link
cardo build --release

You will need simplicity commander, so, install AUR package manager:
cargo install rua
UPD: aur package is outdated.
Get PKGBUILD here:
https://aur.archlinux.org/cgit/aur.git/plain/PKGBUILD?h=simplicity-commander

run `makepkg`, it will downliad zip file, then run `sha256sum <zip file>`  and replace in PKGBUILD.

and then run `makepkg -i`

Run ./binarize.sh to upload firmware.
