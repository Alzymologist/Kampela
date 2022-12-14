# UX overview

The Kampela system consists of Kampela itself and optionally removable NFC token independent from the device but required for authorization and operation of the tool.

For proper signing operation, a hot wallet is required for input and output. In general, these could be two different wallets, the only requirement is that these are brought into network connection within reasonable time before and after transaction signing. In best case scenario there is just one duplex hot wallet.

An optional encoding device could be used if the input hot wallet does not support NFC interfacing.

Thus, we have 3-6 devices that should stay in close proximity of each other, within close distance from external network for Substrate access and user, that handles these devices.

# UX strategy

## Comfort of use

The device should be pleasant to use. Users would have to hold the phone and the Kampela as a stack and operate on it simultaneously. We should not add unnecessary difficulties on top of that. Things like metadata management should happen automatically and without users' help.

Stack configuration must be researched and determined. Possible options are:
- flip: Kampela screen is on the opposite side of the phone screen
- slide: Kampela screen is visible either under or over the phone screen
- book: Kampela screen is to the side of the phone screen

UI decisions on device shape and handling should be made based on live user tests with device mock models or prototypes.

## Friction

Similar to approach taken in Parity Signer development, certain allowed actions that require extra attention should result in friction. All transactions must be read and explicitly confirmed before accessing the secret storage. Removal/change of trust sources should be sufficiently difficult to stop users from doing it mindlessly.

## Forbidden actions

Certain actions should be forbidden altogether even if complete device replacement is the only alternative.

## Disposable Kampela

Users should be ready to get a new Kampela if something goes wrong. There should be a simple and clear safe disposal protocol that anyone could follow - even without authorization. Safe disposal is essentially the best scenario among DOS attacks (which are quite trivial with physical access to device) and should be encouraged.

## Personalization

Kampelas should be highly personalizable, probably in subtle ways. It should be trivial for owner to observe that the device was swapped.

## Few buttons

Due to power and computational limitation, on-board controls would be inevitably limited. Thus the buttons should be really good, reusable and easy to understand.

## Device versions

Users will not have access to device firmware (except for development model probably). Users may want to select own settings during the Kampela initialization (allow secret backup, allow verifier to be changed, block more than one derivation per device). These could be also factory-set so that there are just many different models of Kampela available for customers.

## Signer ecosystem

Kampela should fit nicely within Parity Signer ecosystem and - where possible - should adapt UI components from there.

## Utilize first use moment

The user will be really vigilant only once - on first use. Thus we should utilize this unique moment, minimize amount of annoying attention diluting things and maximize educational impact.

## Manuals

Kampela must be accompanied with sufficient instructions and manuals, possibly even in printed form.

## Migration

When migrating between devices, secret information should be entered anew, since there is no interdevice communication channel (and should not be). All non-secret information could be transferred through NFC from hot wallet; we need to establish the derivation export protocol akin to one used in Signer for batch derivation import, but with broader functionality to make migration as simple as possible (even though inevitably difficult due to seed phrase re-entry at least; see Friction section above).

