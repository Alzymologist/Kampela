# Kampela
Hardware signer and key storage for Substrate ecosystem.
https://kampe.la

-------------------------------------------------
As of September 2023, this monorepo became really hard to work with, and had been splitted into multiple ones under the new [Kalapaja](https://github.com/Kalapaja) organisation.  
⚠️ This repo is kept up for archival reasons, please submit any issues and pull requests in that new org instead.

The new repository structure under Kalapaja is the following:

| New repository | Original subfolder | Description |
|----------------|--------------------|-------------|
|[kampela-firmware](https://github.com/Kalapaja/kampela-firmware)|[`firmware/development`](https://github.com/Alzymologist/Kampela/tree/main/firmware/development)|Firmware sources and associated software projects|
|[kampela-hardware](https://github.com/Kalapaja/kampela-hardware)|['hardware/development'](https://github.com/Alzymologist/Kampela/tree/main/hardware/development)|Hardware design schematics and supplementary files|
|[kampela-common](https://github.com/Kalapaja/kampela-common)|['common'](https://github.com/Alzymologist/Kampela/tree/main/common)|Collection of shared libraries used by both "hot" and "cold" side of the project|
|[siltti](https://github.com/Kalapaja/siltti)|[application](https://github.com/Alzymologist/Kampela/tree/main/application)|Android companion app to interface with the device|
|[docs](https://github.com/Kalapaja/docs)|[docs/development](https://github.com/Alzymologist/Kampela/tree/main/docs/development)|Miscellaneous docs documenting different aspects of the project|



-------------------------------------------------

## Project summary
Kampela is a hardware version of Parity Signer. It is a small card-shaped device (ideally comparable to a credit card form factor, to be carried in a wallet when needed) that accepts data through unidirectional NFC port and shows output on a monochrome electronic paper screen. It has cryptographic strongbox with elliptic curves supported by Substrate —  which only stores private keys (after initial import) and performs all signing operations on-chip.

Kampela is designed to be a comparatively low-cost (below €100, in contrast to €500+ for a Signer-compatible smartphone) single-purpose device, which would only support a single seed phrase per unit, operated via a drastically simplified user interface. However, it would have all the security features of Parity Signer: a full-featured SURI account derivation system, transactions preview, metadata updates, logging.

As something that supports the same QR code output formats as Signer, it would be able to drop-in replace Signer when providing signatures back to a mobile/desktop wallet. The only challenge would be in transforming the current QR-driven input format of Signer into a suitable NFC payload. Alongside with native support in some apps (which would require software modification) this can also be achieved with a tiny “translator” app for a regular (online, not necessary trusted — one’s usual daily driver) mobile phone: it would snap a QR code with the transaction details and (statelessly, without any extra features or complicated UI) transform that into an NFC payload.

Kampela is a natural (and long-discussed) extension of Parity Signer to leverage the most out of modern hardware-based crypto chips and drastically reduce Signer’s attack surface: no mobile OS, not extra platform features, no unexpected communication methods or airplane mode to take care of.

Kampela is not a wallet! It is a signing tool, it is not able to track on-chain balances, validate things over chain-provided information or generate transactions on itself.

## Problem statement

Parity Signer project is a unique solution to provide user-friendly secure airgapped signing with seamless integration into ecosystem. However, there are certain problems that could not be solved at the moment:

Hardware cryptographic chips in modern phones do not support elliptic curve algorithms used in Substrate networks — so in result secrets being exposed to the main RAM of the device for the duration of usage (only hardware-encrypted at rest), where it may potentially leak or be accessed by malicious software.

Current hardware compatibility is limited: only newer iOS devices and few latest models of Android devices support hardware encryption of secrets. This puts supported devices on the more expensive side of the price range.

With iPod Touch being decommissioned, all devices currently in production which support necessary hardware features also include baseband chip — which runs a separate closed-source OS running on a separate, special-purpose chip while commonly having a full DMA access to the main phone’s memory. This chip could not be trusted (see Samsung’s baseband backdoor as an example) and has potential to leak secrets through radio channel or by use of kleptography, overriding on-board entropy generation or execution environment.

## Details

Kampela is an approximately credit card sized box with a rectangular screen and minimalistic controls (capacitive buttons alongside the screen or something similar — to be designed). In addition to development of the device itself (which will be available under suitable FOSS licenses both in terms of software and hardware designs), a reference MVP implementation of QR-to-NFC converter app will also be created.

After seed phrase initialization (either input or random generation — the latter being a potential technical challenge due to its computational/UX intensity and lack of entropy available) it will be stored in an hardware-backed encrypted storage. Address derivations could be created in Kampela either by passing derivation strings from mobile device via NFC (since those are not secret on their own) or by typing on the device itself (with some form of dictionary-driven assisted input, due to limited controls available); resulting public keys could be exported through QR codes, in the exactly the same way as with Parity Signer. Transactions for signing are sent into Kampela via NFC, their contents rendered on low-power e-ink monochrome screen and then user can unlock the device and sign transaction. Unidirectional nature and limited range/direction of both communication interfaces ensures that user remains in full control of data. Kampela has no battery (at least no battery for long-term power supply) and is powered only through NFC — which adds another layer of security, since no long-term backdoors can survive in RAM, and compromising the stored firmware itself is a much higher bar to clear for the attackers. Kampela will use a Secure Element or similar technology (like Arm TrustZone) to store secrets in a way, which will make it as hard as realistically possible to extract the secret keys, even with full physical access to the device and lab equipment being available.

Kampela is envisioned to be as close to open hardware criteria as possible, as long as it won’t compromise the resulting security model directly (for example, if requirement to support arbitrary user-provided firmware would clash with chosen security model, we would choose security model; all such cases would be clearly argumented and documented, of course). All security-sensitive parts of the device (NFC, RNG) would be implemented as much as possible through discrete, fixed-logic components (potentially with FPGA for the more uncommon parts of it, like sr25519  crypto) instead of being done “in software” or trusted to all-in-one integrated chip developers. Firmware for the programmable part of the device is to be written in Rust. On the smartphone companion app, it would use the same proven combination of Rust and native platform-dependent frameworks as Parity Signer currently does.

Unlike other projects in the similar problem space (which routinely withheld certain design specifications, and make others only available under an NDA, see, for example, discontinued project Cobo), we aim to make an extreme push to transparency of our designs, both on hardware and software layers. While this is expected to provide a boost in security and verifiability of Kampela devices, this severely restricts any potential future commercialization effort, since the competition (which wouldn't have to recover R&D costs) would undercut any official manufacturers on profit margins. We'd prefer this extreme competition to be a feature, not a bug; the only project with comparable manufacturing model would be Trezor, but it has a series of well-known shortcomings both in design and hardware choice areas.

## Ecosystem Fit

Currently, there are not too many options to securely handle keys in the Polkadot/Kusama ecosystem. There’s some Ledger support (which is often lagging behind the runtimes update cadence, and doesn’t exist for all the parachains out of the box), and Parity Signer and it’s fork Stylo (which requires an expensive smartphone device being used for this airgapped purpose only, and also has a much wider attack surface due to the device having a general-purpose OS and at least potential WiFi/3G/Bluetooth connectivity). We expect Kampela (having its own set of quite distinct tradeoffs) to supplement those two solutions — and integrate nicely with other layers of the tech stack (wallets, browser extensions etc), being a drop-in replacement of the currently already broadely supported Parity Signer.

## Origins of the name

Kampela, a flatfish — since the device is flat and is intended to resist phishing.

## Why Polkadot network?

Kampela project is a long-missing component to ultimately harden security of the whole Substrate ecosystem and ensure it’s secure and unstoppable status in long-term. Polkadot part of the greater ecosystem, as flagship and stable reference core, is natural space to develop universal solid solution like Kampela.

## Reviewers responses

Q: ​I think the NFC part isn’t very great from a usability perspective.

A: On contrary, NFC transfer of data into the device would be much smoother than with QR movies, especially for large payloads like metadata. The idea is that user will attach Kampela to the back of the smartphone, perform all the operations that do not require security with convenience of familiar smartphone interfaces, then flip the stack over and perform secure operations until output QR is shown. With QR presented on e-ink screen, Kampela could safely be detached from the phone as the QR intended for public broadcast would remain on screen. The task of mechanical engineer in this project (in addition to making device casing cheap to produce and robust) would be to ensure this flow is as comfortable and universal across the phones as possible.

Q: I guess the user does not really care about the RAM exposure as long as my phone is offline all the time.

A: Unfortunately, this is not so. The applications that could corrupt memory used by the signing application could manipulate RNG into leaking arbitrary data (including leaked secrets) through kleptography methods without exposing the leakage in any way. The leaked secrets could be broadcasted secretly through signature screen or other non-disabled communication methods via steganography. Finally, secrets leaked to memory could be stored outside of hardware encrypted storage negating its benefits in case attacker has physical access to the device reducing his need for fully-equipped forensic lab to relatively cheap and widespread memory dumping equipment. The Kampela solves these issues by lack of shared execution environment (and potentially rogue independent blackbox systems on board) and solid rust code with proper zeroization implemented.
