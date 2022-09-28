# How the user can check that the device matches the specs?

- No transmitting parts.
- Secure chip as specified.
- Functional RNG.
- No tampering (e.g. no change in software/hardware after device has left the factory).

Comments:
1. Not truly possible for user to check that RNG is indeed random. Any check would find only outrageous flaws and may erroneously assure user of safety.
2. User may check though that there is no signal emission.
3. Anti-tampering protection can assure user only of no out-of-factory tampering.

Notes:
1. User will have to trust the manufacturer to install correct hardware and software.
2. Users can and will disassemble their own devices, potential in-factory tampering can be revealed. Not much good for users already compromised though.


# What data is stored in Kampela and Kampela companion?

Each Kampela may have only one secret. Primitive hardware allows no convenient way to select secret and avoid confusion.


## Decision 1. Can the entered secret be corrected in Kampela?

Pros:
- Fix typos on start
- Flexible Kampela

Cons:
- Seed phrase swap(2-I4), also more silly things such as 2-A2

Possible solution: allow user to set secret and freeze it into memory separately.
Keep sufficiently annoying reminders to freeze the secret.
Note. Copious explanations in manual needed.


## Decision 2. What is protected by pin?

### Pin protects the access to secret storage.

Secret storage is activated when keypair is generated during:
- signing
- calculating public key for a new address

Modifying secret before it is frozen? Pin is used.

Adding or removing chain specs and chain verifiers? Secret not used. No pin. 2-I2, 2-I3, 2-A3.

Removing address if addresses are stored on Kampela? No pin. 2-A2.

### Pin both unlocks Kampela and protects secret storage.
Pros:
- Substantially more difficult to mess with specs, verifiers, addresses.

Cons:
- Pin is effectively entered twice all the time (annoying, too often used pin). Note: sensor moving buttons to avoid 2-I2?

### Two pins.
Cons:
- User will make them identical and simple on top of that. Or forgets them. Or both.

### Pin to access secret storage AND Kampela has a set of friendly devices
Kampela needs to be acquainted with companion device before use. This requires pin.
Companion device has unique identifier, Kampela stores all introduced devices identifiers (where? in db, next to chain specs?)
Of course, companion device can be banned.

Cons:
- Kampela safety with specs and verifiers depends on possibly always online companion device.
- User will loose access or will be staggered if companion phone is

Proposed solution: by default make user enter pin for unlocking and to access the secret.
Allow users remove pin entry on unlocking. If they live in the middle of a swamp and hate pins.

## Decision 3. What kind of secret is stored in Kampela?

Seed phrase ("bottom drive obey lake...")?
Secret key ("0xfac7959dbfe72f...")?
Any of them?

This doesn't make much difference for signing itself (combined secret construction is same).
The only difference is how user will load the secret in Kampela.

Manual entering through keyboard is possible for both options.

Allow both? Would this cause much confusion?

Allow both and strongly recommend seed phrase?

Allow both only in advanced mode settings? (proposed variant)

Allow only seed phrase?

Typing hex string may be more difficult (64 hex symbols, non-readable, unable to backup properly), but some users may need it.
Additionally, Kampela does not allow to change the secret once it is entered, typo probability is really high for secret key.


## Decision 4. Where are public keys and associated data stored?

To form a signature, Kampela needs the secret and full derivation path (soft `/`, hard `//`, password `///`).

Signable transaction contains public key and encryption algorithm to identify *who* signs the transaction.

Getting corresponding derivation path can happen in Kampela itself or in companion app.

### 4A Public keys and associated data are stored on Kampela

Companion in this case has to store only public keys.
Kampela searches for derivation by public key, calculates keypair, checks public key, calculates signature.

Cons:
- memory taken on Kampela
- resources (power) to make database search (unsure how relevant this is)
- Kampela stores non-secret data **together** that could cause confidentiality issues (1-C1, 2-C1)

Unclear: how is the password (`///`) for passworded accounts entered? Proposal: unicode symbols via hex combinations.

Perform confidentiality breach on Kampela seems more demanding than on companion (for starters, companion is online, Kampela needs physical stealing).

### 4B Public keys and associated data are stored on companion

Kampela stores nothing.

Kampela uses derivation to calculate keypair, compares public key with the one from signable transaction, calculates signature.

Kampela is still used to export public keys and data into companion when setting the companion up.


No particular difference between 4A and 4B. Public keys are on companion either way. Kampela has to have the ability to export public key data (as a QR) into companion either way.
Lean to storing derivations in Kampela, to avoid keys double export and user loosing derivation path (4-I6, 2-A5).

## Decision 5. Chain verifiers.

Each chain has a verifier, i.e. an address that is trusted to check and publish correct metadata and to lesser extent chain specs.

### Is entirely unverified data allowed?

Proposed: no by default. Allow in advanced settings.

Note. If the proposal to involve metadata hash in signing will pass, this changes to default yes.


## Decision 6. Chain specs.

Chain specs are used to display parsed transactions more conveniently.

### Does Kampela need to use them at all?

- base58 prefix could be in metadata; what if it is not?
- units-decimals could just be ignored; ugly, but users may not even care.

Pros:
- prettier display

Cons:
- more storage on Kampela
- verified specs, additional way to mess with user

Specs must be stored somewhere, at least to have genesis hash and chain name associated.


## Decision 7. Chain isolation.

Pros:
- user can not accidentally apply wrong address even if the clien does not separate chains

Cons:
- users can and will get confused. See Signer complains.

Also, if isolate: where?

Suggestion: isolate in companion.


## Decision 8. Is removing derivations from Kampela allowed?

Pros:
- typos
- user cleanup
- secret accounts created briefly to be used and then to be deleted until next use

Cons:
- accidentally or maliciously remove good derivation and there was no backup

# Proposed design draft.

Kampela stores secret protected by pin.

Kampela stores data for each chain, protected by pin.
- genesis hash
- chain name
- base58, units, decimals

Say, Kampela creates addresses, does not store them, exports piblic key data as qr code.
Unclear. Double scan may be dangerous (companion device is online and assumed not secure), can export wrong public key this way.
Single scan from Kampela: how user keeps the derivations recorded then?
Also, loosing the derivation path is quite real.

If Kampela does not store the derivations, they are stored elsewhere, for example, in pjs. This is less confidential than Kampela behind the pin.
So maybe store the derivations after all.

If Kampela stores the derivations, it keeps:
- public key
- derivation
- pwd or no pwd
- encryption algorithm
- network(s)

# How user distinguishes addresses?

Imagine the mess: a few Kampelas (personal funds seed, work account seed, Very Important work account secret), lots of derivations (overlapping for different seeds, because `//polkadot` is a convenient default derivation), a few associated companion devices.

Identicons are not super effective here. They are blind-ish and if user has more than 10 to look at, they all look too similar.
Base58 addresses, hex representation - could be ok to check that account is correct, same as identicon, but not search with, probably.

User may want to give the account the name on creating. How and where? And what will prompt user to remember the derivations at all? How to distinguish identical derivations from different Kampelas?

Possible: create upon secret freezing the name for device. Keys would look like `DeviceName smth-smth` with `smth-smth` optionally entered by user on Kampela keyboard or rather the public part of derivation itself.

# Do we allow to change the pin?

Really?

# When pin is entered?

(1) when Kampela is just connected to companion phone
(2) when command is already formed and sent through companion phone
(3) right before the action (accept specs, create derivation, etc)
