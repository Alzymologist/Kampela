# Kampela screens

Kampela has two screens:

- QR screen (for QR exports and signatures)

- Display screen (for other data: derivations list, specs available, verifiers etc)

All output from Kampela must be through either of these screens.

# Kampela gets commands only through interface

By receiving appropriate NFC from introduced device, Kampela can:

- load chain specs (save specs in storage)
- create an address (save public address information in storage, create export QR on Qr screen.)
- sign a transaction
- sign a text message
- remove chain specs
- remove an address
- show info in Kampela (nothing is sent, Kampela own controls are used)

NFCs from alien devices are not accepted. Introducing device requires pin code.
Any action on Kampela requires on-Kampela user confirmation (i.e. user reading from Kampela screen the incoming transaction) and a pin entry.

# Kampela is as expected

There must be a set of checks for Kampela in place.

1. Code/design are audited, well, at least looked into, as any open source.
2. Factory receives chips as designed.
3. Assembly in factory is as designed.
4. Firmware in factory is as designed.
5. Assembly/hardware remains the same while in transit to user.
6. Firmware same remains the same while in transit to user.
7. Assembly/hardware can not be swapped quickly and discreetly while Kampela is with user.
8. Firmware can not be swapped quickly and discreetly while Kampela is with user.

# Summary of secret-related storage in Kampela.

1. Part of chip own secret key, as set in chip factory.
2. Part of chip own secret key, added in Kampela assembly factory.
As the Kampela exits the Kampela assembly factory, the chip secret key is complete, and factory records the Kampela public key.
User checks that the chip is genuine by signing a challenge with chip secret key and verifying with public key known at Kampela factory.
3. Firmware build is stable. Its hash must match the officially published one. Hash is checked, ledger-like protocol here.
4. User own secret (seed phrase, secret key?) - entered by user, stored encrypted with chip-intrinsic PUF-containing key (same as 2 or separate?).
5. Pin code - for unlocking the Kampela; possibly mix into user secret encrypting key (4).
6. Companion phone(s) public key(s).
