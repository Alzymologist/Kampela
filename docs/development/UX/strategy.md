# UX strategy

## Comfort of use

The device should be pleasant to use. Users would have to hold it together with phone and flip this stack over, probably several times. We should not add unnecessary difficulties on top of that. Things like metadata management should happen automatically and without users' help.

## Friction

Similar to approach taken in Parity Signer development, certain things that are allowed but only should be done with care should result in friction. All transactions must be read and explicitly confirmed before the secret storage access. Removal/change of trust sources should be sufficiently difficult to stop users from doing it mindlessly.

## Forbidden actions

Certain actions should be forbidden altogether even if complete device replacement is the only alternative.

## Disposable Kampela

Users should be ready to get a new Kampela if something goes wrong. There should be a simple and clear safe disposal protocol that anyone could follow - even without authorization. Safe disposal is essentially the best scenario among DOS attacks (which are quite trivial with physical access to device) and should be encouraged.

## Personalization

Kampelas should be highly personizeable, probably in subtle ways. It should be trivial to observe that the device was swapped.

## Few buttons

Due to power and computational limitation, on-board controls would be inevitably limited. Thus the buttons should be really good, reusable and easy to understand.

## Device versions

Users would not have access to device firmware (except for development model probably). Some options that could only be set once (post-setup seed backup, verifier change, derivations enablement) could be factory-set so that there are just many different models of Kampela available for customers.

## Signer ecosystem

Kampela should fit nicely within Parity Signer ecosystem and - where possible - should adapt UI components from there.

## Utilize first use moment

The user would be really vigilant only once - on first use. Thus we should utilize this unique moment, minimize amount of annoying attentios-diluting things and maximize educational impact.

