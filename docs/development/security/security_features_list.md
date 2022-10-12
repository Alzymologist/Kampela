## Hardware
- Touch screen for data entering on Kampela. Shuffling buttons on pin entering. Stable keyboard for text entering.
- Additional removeable token required when entering the pin.
- Personalized Kampela.
- PUF on crypto chip.
- Kampela is active only briefly after disconnect with companion device. Kampela is not powered.
- Kampela firmware can't be changed unless Kampela is physically opened, or in "hacker edition".
- Tamper-evident casing.
- Inner circuit produces minimal noise.
- Build signature is checked on-board.
- Self-destruction can be initiated on Kampela only locally, no api for remote one or one through companion device.
- Self-destruction must be available without the detachable token or any other authorization.
- We need to make sure that NFC can provide stable and full energization of the circuit. Undefined behavior due to power surges or lack of energy should be ruled out. We might consider using a small energy storage to provide short-term power supply stability.

## Firmware
- Kampela stores all derivations created on it.
- Kampela is able to display derivation path as hex combination.
- All incoming data must be parsed in Kampela and displayed to user.
- Signatures for all verifiable data must be checked on Kampela.
- Forbid blind signing.
- Sufficiently complex pin.
- Limit the number of pin entering attempts before Kampela is irreparably blocked.
- Request pin when accessing the secret (backup, signing).
- Request pin when adding data into Kampela.
- Kampela can receive payloads only from introduces companion devices.
- Freeze the secret.
- Request pin when introducing a new companion device.
- Message parsing displays both text and hex, with highlighted suspicious symbols in hex.

## Supply chain
- Trustworthy and mass-produced crypto chip.
- Verification challenge for crypto chip.
- Factory QA.
- Firmware has reproducible builds.
- Kampela is cheap and is easily replaceable.

## User materials
- User instructions and manuals (both physically printed and online)

## Ecosystem
- Support verifier system.
- Warn on companion device side that a new Kampela is in action (this checks only removeable emitting token).

