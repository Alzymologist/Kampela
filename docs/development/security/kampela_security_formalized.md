Kampela protects only user secrets, user assets, and user identity.
This text deals only with the effects of various attacks on user.
Attacks are ranged by proximity of the potential attacker to Kampela device.

Integrity breaches are considered entirely unacceptable. Base incentive: High.
Confidentiality breaches are highly undesirable. Base incentive: Medium.
Availability breaches could be tolerated. Base incentive: Low.

Note: availability failures might be used as basis for blackmailing attack though, where, even though the secrets are safe (considering that user did indeed follow backup protocol), financial damage could occur. Think big brute holding your device and demanding a dollar to give it back; for a small enough sum, this transaction might happen regardless of whether Kampela is still operational or even actually returned upon payment. This is not very different from similar attacks elsewhere and would not be considered here much, as only user's wits could protect from these.

There is one special case, where the user is tricked into using account with derivation path known only to attacker; here the legal user and an attacker share components to unlocking the account and would have to negotiate. Here, though, the secret is actually compromized in one signature or public key generation event; thus technically this is an integrity breach (much like signing a single malicious proxy call to delegate full control to attacker). Attacks where public key is substituted outside of Kampela are out of scope here.


# 1 Kampela in direct access, for unlimited period of time (stolen, found, etc)

Note 1. If an attacker knows the pin code, the game is over, it is the same as if the attacker has the secret phrase. We will assume that the attacker does not have the pin.

Note 2. User must follow end-of-life disposal protocol, otherwise normal use will end up here.

Base effort: Medium


## Confidentiality

Motivation: Stalk or expose the user.


1-C1

Attacker action: Get all public keys, including the ones the user hides from wide public, stalk user through polkascan.

Effort: Medium

Incentive: Medium


## Integrity

Motivation: Get access to user identities and assets.


1-I1

Attacker action: Read secret phrase. Attacker must break through authentification.

Effort: High

Incentive: High


1-I2

Attacker action: Carefully disassemble the device, read secret with advanced instrumentation.

Effort: High

Incentive: High


1-I3

Attacker action: Proxy account control. Attacker must break through authentification.

Effort: High

Incentive: High


1-I4

Attacker action: Attacker takes the device after it is unlocked by user.

Effort: High

Incentive: High


## Availability

Note. If user loses whether the detachable token, or Kampela, or both, the availability is inevitably lost.


# 2 Kampela in direct access, briefly (stolen and returned, device remains the same)

Note. If attacker knows the pin code, the game is over, attacker same as has the secret phrase. Assume here they don't have the pin.

Base effort: Medium


## Confidentiality

2-C1

Same as direct access for unlimited period of time.


## Integrity

2-I1

Motivation: Get the secret phrase.

Attacker action: Break pincode by observing physical state of the Kampela (scratched buttons, etc).

Effort: High

Incentive: High


2-I2

Motivation: Swap chain specs so that later transactions are parsed incorrectly and signed without knowing true content.

Attacker action: Send chain specs NFC payload, accept it.

Effort: High

Incentive: Medium


2-I3

Motivation: Swap chain verifier so that later compromised metadata is fed to Kampela, thus transactions are parsed incorrectly and signed without knowing true content.

Attacker action: Send chain specs NFC payload with new verifier, accept it.

Effort: High

Incentive: Medium


2-I4

Motivation: Seed phrase swap.

Attacker action: Change seed phrase, user generates wrong public key in companion and uses it to accept assets. Attacker must break through authentification.

Effort: High

Incentive: High


2-I5

Motivation: Seed phrase swap.

Attacker action: Coerce user to remove secret. Enter wrong secret (by taking Kampela shortly). User then generates wrong public key in companion and uses it to accept assets.

Effort: High

Incentive: High


2-I6

Motivation: Maliciously re-programm Kampela.

Attacker action: Kampela is shortly taken from user, re-programmed (for example, to compromize the secret through defective randomness), returned to user.

Effort: High

Incentive: High


## Availability

2-A1

Motivation: Destroy or block device.

Attacker action: Physically break device or enter wrong pin multiple times.

Effort: Medium

Incentive: Low


2-A2

Motivation: Temporarily stagger user activity.

Attacker action: Delete or damage seed.

Effort: Medium

Incentive: Low


2-A3

Motivation: Temporarily stagger user activity.

Attacker action: Remove one or more user addresses manually.

Effort: Medium

Incentive: Low


2-A4

Motivation: Temporarily stagger user activity.

Attacker action: Remove one or more chain specs manually.

Effort: Medium

Incentive: Low


2-A5

Motivation: Block user assets.

Attacker action: Copy derivation path, remove address manually, if user does not have backup, assets on address are blocked.

Effort: Medium

Incentive: Medium


# 3 Kampela in close proximity (watched over shoulder)

Base effort: Low


## Confidentiality

3-C1

Motivation: Peek at user's transaction.

Attacker action: Look over the shoulder or tap NFC field, it would probably be visible on other devices as well.

Effort: Low

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

Effort: Medium

Incentive: Low

Note. Breaking may be unintentional, for example, burning circuits with too high signal (supermarket anti-theft etc).


# 4 Kampela far away (attacks without breaking the companion app)

Base effort: None


## Confidentiality



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

Motivation: Compromise device firmware during an update, expose data and assets.

Attacker action: Publish/propose Kampela update that would install damaged/incorrect firmware into Kampela. User may sign transactions they did not agree to, or create addresses they did not wish to. Attacks may occur only if certain conditions are met, and thus be difficult to detect.

Effort: High

Incentive: High


4-I7

Motivation: Make user sign a wrong message.

Attacker action: Compose "sign_message" content with utf-8 symbols that are not printable and/or are metasymbols (reverse text direction, move caret, etc.), so user reads and signs different messages.

Effort: Medium

Incentive: Low


4-I8

Motivation: discover user pin code (device is later stolen to use the pincode and retrieve the secret).

Attacker action: Kampela, when used, is close to the companion device, maybe attached, definitely in visible proximity. Compromized companion device may record pictures of user entering pin code, analyze sensor data or button sounds to deduce what buttons user is pressing when entering the pin code.

Effort: Medium

Incentive: High


## Availability

4-A1

Motivation: Trick user to break device.

Attacker action: Convince user that blocking actions should be performed (self-destruct mechanism, verifier recall, EMP).

Effort: Low

Incentive: Low


# 5 Swap used Kampela (original device is replaced after secret entering)

Base effort: Medium

This includes only attacks through swap itself. Attacks through taken Kampela are discussed above, in 1-2.

User may not notice that the secret is missing (if the device does not show initiation screen) or that the secret is changed.
User most likely will not notice that the pin is not checked (but asked).

Swapped device contains a different secret or no secret at all. Swapped device may be a different Kampela or Kampela-like fake device.


## Confidentiality

5-C1

Motivation: Plant a tracking device (mic, gps tracker).

Attacker action: Plant fake Kampela-looking device with a tracker. No need to retrieve the original device here.

Effort: Medium

Incentive: Medium


## Integrity

5-I1

Motivation: Steal or expose the secret by making user re-enter the secret in Kampela replica.

Attacker action: Plant fake Kampela and compel user to re-enter the secret. Any damaging modification to Kampela (see 7-I1..7-I5) is applicable here. Original device may or may not be retrieved.

Effort: High

Incentive: High


5-I2

Motivation: Steal pincode by planting Kampela replica with retranslator.

Attacker action: Steal original Kampela and replace it with replica that demands pincode (as in any normal Kampela operation) and retranslates it somehow to the attacker. With original Kampela and pincode, the attacker has full access to the secret.

Effort: High

Incentive: High


5-I3

Motivation: Make user generate an address user has no or hindered access to. Attacker *has* access to the address, catches the funds transferred into the address.

Attacker action: Plant fake Kampela with a different secret inside. User creates address and sends funds to it, attacker can get or transfer funds before user notices the error.

Effort: Medium

Incentive: High


## Availability

5-A1

Motivation: Make user generate an address neither user nor attacker have full access to, demand ransom.

Attacker action: Plant fake Kampela with a different secret inside. User creates complex address with unexpected derivation and/or password, and sends funds to it, attacker demands ransom.

Effort: Medium

Incentive: High


# 6 Companion device compromised

Base effort: Low

An important note here is that a relatively simple way to perform this would be to swap the companion device. Thus, we should rely on companion device's authentication mechanisms and probably Kampela's ability to authenticate companion device and notify the user.


## Confidentiality

6-C1

Motivation: Learn key ownership information through complimentary app breaches.

Attacker action: Gain control over complimentary app and download log or set of keys.

Effort: Low

Incentive: Low


## Integrity

6-I1

Motivation: Trick user to export account with derivation path known only to attacker and then send assets there; blackmail to "reveal the secret"

Attacker action: Send a derivation with non-visualizeable symbols into Kampela through corrupted companion app; trick user into removing this derivation after some use. Alternatively: manipulate companion app to put the derivation different from the one the user intended. User creates account, does not notice the derivation difference, sends funds to account and can not get access to funds because no derivation.

Effort: Medium

Incentive: High


6-I2

Motivation: Make user sign transaction by wrong address (same seed, different derivation).

Attacker action: change public key in transaction to a different, corresponding to different derivation.

Effort: Medium

Incentive: Medium


6-I3

Motivation: Make user sign transaction different from one they were forming.

Attacker action: change transaction core in transaction to a different one.

Effort: Medium

Incentive: High


6-I4

Motivation: Steal user secret when it is loaded from shards (SSS protocol).

Attacker action: compromised phone camera sends all images to attacker. Each shard scan contains also the recovery phrase, thus the attacker would have all the data needed.

Effort: Low

Incentive: High


## Availability

6-A1

Motivation: Disable the Kampela

Attacker action: Try to engage self-destruct mechanism, scramble derivations tree or physically damage the tool

Effort: High

Incentive: Low


# 7 Swap unused Kampela (user gets fake device before secret entering)

Base effort: High

User gets malicious Kampela-like device instead of the real Kampela. Swap happens in factory, during the shipping, etc.


## Confidentiality


## Integrity

7-I1

Motivation: Steal secret through radio channel.

Attacker action: Make device replica with radio transmitter.

Effort: High

Incentive: High


7-I2

Motivation: Expose secret through kleptography.

Attacker action: Manipulate RNG in factory.

Effort: High

Incentive: High


7-I3

Motivation: Expose secret through kleptography.

Attacker action: Manipulate Kampela crypto libraries.

Effort: High

Incentive: High


7-I4

Motivation: Fog user operations (address generation, transaction signing) by malicious Kampela firmware.

Attacker action: Install damaged/incorrect firmware into Kampela. User may sign transactions they did not agree to, or create addresses they did not wish to. Attacks may occur only if certain conditions are met, and thus be difficult to detect.

Effort: High

Incentive: High


7-I5

Motivation: Steal secret stored in damaged device.

Attacker action: Modify Kampela to disable protection, get the user to use it, and then steal Kampela. Seems possible only if the identity of the user is known to begin with.

Effort: High

Incentive: High


## Availability


