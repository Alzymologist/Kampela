Kampela protects only user secrets, user assets, and user identity.
This text concerns only the effect of various attacks on user.
Attacks are ranged by proximity of the potential attacker to Kampela device.

Integrity breaches are considered entirely unacceptable. Base incentive: High.
Confidentiality breaches are highly undesirable. Base incentive: Medium.
Availability breaches could be tolerated. Base incentive: Low.

Note: availability failures might be used as basis for blackmailing attack though, where, even though the secrets are safe (considering that user did indeed follow backup protocol), financial damage could occur. (think big brute holding your device and demanding a dollar to give it back; for a small enough sum, this transaction might happen regardless of whether Kampela is still operational or even actually returned upon payment). This is not very different from similar attacks elsewhere and would not be considered here much, as only user's wits could protect from these.

There is one special case, where the user is tricked into using account with derivation path known only to attacker; here the legal user and an attacker share components to unlocking the account and would have to negotiate. Here, though, the secret is actully compromized in one signature or public key generation event; thus technically this is an integrity breach (much like signing a single malicious proxy call to delegate full control to attacker). Attacks where public key is substituted outside of Kampela are out of scope here.

# 1 Kampela in direct access, for unlimited period of time (stolen, found, etc)
Note 1. If attacker knows the pin code, the game is over, attacker same as has the secret phrase. Assume here they don't have the pin.
Note 2. User must follow end-of-life disposal protocol, otherwise normal use will end up here.

Base effort: Medium

## Confidentiality

Motivation: Stalk or expose the user.

1-C1
Attacker action: Get all public keys, including the ones the user hides from wide public, stalk user through polkascan.
Effort: Medium.
Incentive: Medium.

## Integrity

Motivation: Get access to user identities and assets.

1-I1
Attacker action: Read secret phrase. Attacker must break through authentification.
Effort: High.
Incentive: High.

1-I2
Attacker action: Carefully disassemble the device, read secret with advanced instrumentation.
Effort: High.
Incentive: High.

1-I3
Attacker action: Proxy account control. Attacker must break through authentification.
Effort: High.
Incentive: High.

1-I4
Attacker action: Attacker takes the device after it is unlocked by user.
Effort: High.
Incentive: High.

## Availability

-

# 2 Kampela in direct access, briefly (stolen and returned)
Note. If attacker knows the pin code, the game is over, attacker same as has the secret phrase. Assume here they don't have the pin.

Base effort: Medium

## Confidentiality

2-C1
Same as direct access for unlimited period of time.

## Integrity

2-I1
Motivation: Get the secret phrase.
Attacker action: Break pincode by observing physical state of the Kampela (scratched buttons, etc).
Effort: High.
Incentive: High.

2-I2
Motivation: Swap chain specs so that later transactions are parsed incorrectly and signed without knowing true content.
Attacker action: Send chain specs NFC payload, accept it.
Effort: High.
Incentive: Medium.

2-I3
Motivation: Swap chain verifier so that later compromised metadata is fed to Kampela, thus transactions are parsed incorrectly and signed without knowing true content.
Attacker action: Send chain specs NFC payload with new verifier, accept it.
Effort: High.
Incentive: Medium.

2-I4
Motivation: Seed phrase swap.
Attacker action: Change seed phrase, user generates wrong public key in companion and uses it to accept assets. Attacker must break through authentification.
Effort: High.
Incentive: High.

2-I5
Motivation: Seed phrase swap.
Attacker action: Coerce user to remove secret. Enter wrong secret (by taking Kampela shortly). User then generates wrong public key in companion and uses it to accept assets.
Effort: High.
Incentive: High.

## Availability

2-A1
Motivation: Destroy or block device.
Attacker action: Physically break device or enter wrong pin multiple times.
Effort: Medium.
Incentive: Low.

2-A2
Motivation: Temporarily stagger user activity.
Attacker action: Delete or damage seed.
Effort: Medium.
Incentive: Low.

2-A3
Motivation: Temporarily stagger user activity.
Attacker action: Remove one or more user addresses manually.
Effort: Medium.
Incentive: Low.

2-A4
Motivation: Temporarily stagger user activity.
Attacker action: Remove one or more chain specs manually.
Effort: Medium.
Incentive: Low.

2-A5
Motivation: Block user assets.
Attacker action: Copy derivation path, remove address manually, if user does not have backup, assets on address are blocked.
Effort: Medium.
Incentive: Medium.

# 3 Kampela in close proximity (watched over shoulder)

Base effort: Low

## Confidentiality

3-C1
Motivation: Peek at user's transaction.
Attacker action: Look over the shoulder or tap NFC field, it would probably be visible on other devices as well.
Effort: Medium
Incentive: Low

3-C2
Motivation: Learn user's public key.
Attacker action: Look over the shoulder, it will be shown on other devices as well.
Effort: Low
Incentive: Low

## Integrity

3-I1
Motivation: Observe pin code or password.
Attacker action: Look over the shoulder, listen to button sounds.
Effort: Medium
Incentive: High (requires access to device later)

3-I2
Motivation: Make user sign malicious transaction.
Attacker action: Send transaction through powerful enough NFT transponder (and hope that user does not read it attentively).
Effort: High
Incentive: High

3-I3
Motivation: Swap chain specs so that later transactions are parsed incorrectly and signed without knowing true content.
Attacker action: Send corrupt chain specs with powerful transponder, make user accept them.
Effort: High
Incentive: Medium

3-I4
Motivation: Get information (possibly secret one) from device.
Attacker action: Listen to electromagnetic noise from device.
Effort: High
Incentive: High
Note. Research needed. Emissions should be minimal.

## Availability

3-A1
Motivation: Break device from a distance
Attacker action: EMP.
Effort: Medium.
Incentive: Low.

Note. Breaking may be unintentional, for example, burning circuits with too high signal (supermarket anti-theft etc).

# 4 Kampela far away

Base effort: None

## Confidentiality

4-C1
Motivation: Learn key ownership information through complimentary app breaches.
Attacker action: Gain control over complimentary app and download log or set of keys.
Effort: Low
Incentive: Low

## Integrity

4-I1
Motivation: Trick user into signing proxy account.
Attacker action: Send proxy call, use social engineering.
Effort: High
Incentive: High

4-I2
Motivation: Trick user into switching to malicious verifier.
Attacker action: Use social engineering to make user change verifier.
Effort: Medium
Incentive: Medium

4-I3
Motivation: Distribute malicious chain data.
Attacker action: Steal verifier key or be malicious verifier.
Effort: High
Incentive: High (could be scaled to many users)

4-I4
Motivation: Trick user into signing transaction blindly.
Attacker action: Use social engineering.
Effort: High
Incentive: High

4-I5
Motivation: Use message signing to make signature usable elsewhere.
Attacker action: Find vulnerability where "sign message" mechanism has insufficient replay protection or message that collides with call, make user sign it.
Effort: High
Incentive: Medium

4-I6
Motivation: Block user funds at derivation unknown to user, demand ransom.
Attacker action: Companion device in manipulated to put the derivation different from the one the user intended. User creates account, does not notice the derivation difference, sends funds to account and can not get access to funds because no derivation.
Effort: Medium
Incentive: High
Note. User also can do this accidentally (forget or loose own derivation), Kampela must store derivations.


## Availability

4-A1
Motivation: Trick user to break device.
Attacker action: Convince user that blocking actions should be performed (self-destruct mechanism, verifier recall, EMP).
Effort: Low
Incentive: Low

# 5 Kampela swapped

Base effort: Medium

## Confidentiality

-

## Integrity

5-I1
Motivation: Steal keys through radio channel.
Attacker action: Make device replica with radio transmitter.
Effort: High
Incentive: High

5-I2
Motivation: Steal keys through kleptography.
Attacker action: Manipulate RNG in factory.
Effort: High
Incentive: High

5-I3
Motivation: Steal keys through kleptography.
Attacker action: Manipulate Kampela crypto libraries.
Effort: High
Incentive: High

5-I4
Motivation: Steal keys by swapping device twice.
Attacker action: Plant device with corrupt crypto storage to be loaded with seed, then steal it.
Effort: High
Incentive: High

## Availability

-

# 6 Companion device compromised

Base effort: Medium

An important note here is that a relatively simple way to perform this would be to swap the companion device. Thus, we should rely on companion device's authentication mechanisms and probably Kampela's ability to authenticate companion device and notify the user.

## Confidentiality

In case of companion app (or actually any wallet's) breach, confidentiality damage would be much more severe, than anything that could happen if Kampela is additionally breached somehow. This case is out of scope.

## Integrity

Motivation: Trick user to export account with derivation path known only to attacker and then send assets there; blackmail to "reveal the secret"
Attacker action: Send a derivation with non-visualizeable symbols into Kampela through corrupted companion app; trick user into removing this derivation after some use.
Effort: Medium
Incentive: High

Motivation: Make user sign transaction by wrong address (same seed, different derivation).
Attacker action: change public key in transaction to a different, corresponding to different derivation.
Effort: Medium
Incentive: Medium

Motivation: Make user sign transaction different from one they were forming.
Attacker action: change transaction core in transaction to a different one.
Effort: Medium
Incentive: High

## Availability

Motivation: Disable the Kampela
Attacker action: Try to engage self-destruct mechanism, scramble derivations tree or physically damage the tool
Effort: High
Incentive: Low
