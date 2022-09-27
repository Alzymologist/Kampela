<!DOCTYPE html>
<html>
<head>
<style>
table, th {
  border: 2px solid black;
  border-collapse: collapse;
}
td {
  border: 1px solid black;
  border-right: 2px solid black;
  border-collapse: collapse;
}
#first td {
  border-top: 2px solid black;
  border-collapse: collapse;
}
</style>
</head>
<body>

<h2>Effort vs incentive, summarized</h2>

<p>Soft and comfortable straw people right here, but someone has to start.</p>

<table>
  <tr>
    <th></th>
    <th>Effort: Low</th>
    <th>Effort: Medium</th>
    <th>Effort: High</th>
  </tr>
  <tr id="first">
    <th  rowspan="16">Incentive: High</th>
    <td  rowspan="16"></td>
    <td>3-I1 Observe pin entering, use pin later.
      <br>Counter:
      <br>(1) user instructions
      <br>(2) touch screen buttons with shuffling
      <br>(3) optional: demand the second factor present for every pin entering
    </td>
    <td>1-I1/2-I1 Read secret from storage by breaking the pin.
      <br>Counter:
      <br>(1) pin must be difficut to break, guess or steal (trust user here?)
      <br>(2) allow limited (how severely?) number of attempts
      <br>(3) user responsibly creates, stores and safely uses the pin
      <br>Who, when and how sets the pin? User, right before setting up the secret.
      <br>Changing or resetting the secret allows to change the pin.
    </td>
  </tr>
  <tr>
    <td>4-I6 Manipulate companion into making derivation with path unknown to user, ransom.
      <br>Counter:
      <br>(1) make Kampela store all created derivations
      <br>(2) demand pin (and second factor?) when removing derivation from Kampela
      <br>(3) encourage user to backup derivations (how?)
    </td>
    <td>1-I2 Disassemble device, read secret instrumentally.
      <br>Counter:
      <br>(1) using PUF, breaking crypto chip would damage the secret
      <br>(2) trustworthy crypto chip
    </td>
  </tr>
  <tr>
    <td>6-I1 Manipulate companion into making derivation with unreadable symbols in path and/or unknown to user, ransom.
      <br>Counter:
      <br>(1) make Kampela store all created derivations
      <br>(2) additionally display derivation path as set of hex characters from Kampela
    </td>
    <td>1-I3 Proxy address by breaking the pin.
      <br>Counter: same as 1-I1/2-I1
    </td>
  </tr>
  <tr>
    <td>6-I3 Manipulate companion into swapping the transaction content.
      <br>Counter:
      <br>(1) parse transaction on Kampela and display it to user, so that user signs definitely what was parsed
      <br>(2) forbid blind signing
    </td>
    <td>1-I4 Take device already unlocked by user.
      <br>Counter:
      <br>(1) demand pin second time when accessing secret
      <br>(2) demand pin when adding data or signing transaction
      <br>This still leaves uncovered removal/swap of the specs and derivation. So:
      <br>(3) allow device to receive any payloads only from the introduced companion devices
    </td>
  </tr>
  <tr>
    <td  rowspan="12"></td>
    <td>2-I4 Swap secret unbeknownst to user, catch funds on account unrelated to user.
      <br>Counter:
      <br>(1) user is encouraged to freeze the secret (reminders, instructions)
      <br>(2) once the secret is frozen, it could not be changed
    </td>
  </tr>
  <tr>
    <td>2-I5 Coerce user to "fix" the secret, manipulate secret on entering, catch funds on account unrelated to user.
      <br>Counter: same as 2-I4
      <br>Here we can only make Kampela somewhat uncomfortable to use when the secret is not frozen.
    </td>
  </tr>
  <tr>
    <td>2-I6 Maliciously re-programm Kampela.
      <br>Counter:
      <br>(1) any changes in code can be loaded only if Kampela is opened up
      <br>(2) Kampela body must be tamper-evident
    </td>
  </tr>
  <tr>
    <td>3-I2 Make user sign a malicious transaction (sent with powerful transponder).
      <br>Counter:
      <br>(1) parse transaction on Kampela and display it to user, so that user signs definitely what was parsed
      <br>(2) forbid blind signing
      <br>(3) Kampela can receive data only from the introduced companion devices
    </td>
  </tr>
  <tr>
    <td>3-I4 Listen to device.
      <br>Research needed here. Noise from Kampela is expected to be minimal, but that must be checked.
    </td>
  </tr>
  <tr>
    <td>4-I1 Make user sign proxy (social).
      <br>Counter:
      <br>(1) Kampela parses and shows the action of signing off account as a proxy.
      <br>Not much Kampela itself can do against social manipulation.
    </td>
  </tr>
  <tr>
    <td>4-I3 Distribute malicious chain data.
      <br>Counter:
      <br>(1) user is encouraged to trust only reliable verifiers
      <br>(2) user is encouraged to have as few verifiers total as possible to manage verifier info easily
      <br>(3) malicious verifier, if public and broadcasting, can be pointed out by community
      <br>(4) if the chain data used for transaction generation goes on chain, this becomes entirely obsolete
    </td>
  </tr>
  <tr>
    <td>4-I4 Make user sign transaction blindly (social).
      <br> Counter:
      <br>(1) forbid blind signing
    </td>
  </tr>
  <tr>
    <td>5-I1 Steal secret (device swapped for a replica with emission).
      <br> Counter (to demonstrate that the chip is genuine):
      <br>(1) Kampela must be able to sign a verification challenge with its internal secret.
      <br>Internal secret is intrinsic to a device and factory known only a public key from it.
      <br>Maybe part of the internal is from chip itself and part is from Kampela assembly,
      <br>so that the entire secret is not known unless the two cooperate to know it.
    </td>
  </tr>
  <tr>
    <td>5-I2 Steal secret via damaged RNG.
      <br>Counter: same as 5-I1
    </td>
  </tr>
  <tr>
    <td>5-I3 Steal secret via damaged crypto libraries in Kampela.
      <br>Counter:
      <br>(1) sign reproducible builds, check signature on-board, etc.
    </td>
  </tr>
  <tr>
    <td>5-I4 Steal secret by swapping the device twice.
      <br>Counter:
      <br>(1) verify chip authenticity
    </td>
  </tr>
  <tr id="first">
    <th  rowspan="4">Incentive: Medium</th>
    <td  rowspan="4"></td>
    <td>1-C1 and 2-C1 Stalk user after discovering all public keys.
      <br>Counter:
      <br>(1) pin is needed to unlock the Kampela - not helps if already unlocked device is taken
      <br>(2) user is encouraged to delete Super Secret Derivations and keep them backed up elsewhere
    </td>
    <td>2-I2 Swap specs (steal Kampela).
      <br>Counter:
      <br>(1) any new device can be introduced only with the pincode, even on unlocked device
    </td>
  </tr>
  <tr>
    <td>2-A5 Copy path from Kampela, delete address, demand ransom to restore.
      <br>Counter:
      <br>(1) user is encouraged to keep backups in some form
    </td>
    <td>2-I3 Swap verifier.
      <br>Counter:
      <br>(1) user is expected to check the verifier public key when loading updates
      <br>(2) payload with verifier change is expected to come from known device,
      <br>introducing a new device is pin-protected
    </td>
  </tr>
  <tr>
    <td>4-I2 Malicious verifier (social).
      <br>Counter:
      <br>(1) community reputation?
    </td>
    <td>3-I3 Swap specs (transponder).
      <br>Counter:
      <br>(1) payloads can be accepted only from a known companion device
    </td>
  </tr>
  <tr>
    <td>6-I2 Sign with wrong public key (wrong derivation).
      <br>Counter:
      <br>(1) Kampela displays and user is expected to check the signing address
    </td>
    <td>3-I3 Bytes and collision.
      <br>Counter:
      <br>(1) display message and hexadecimal payload to protect from outrageously
      <br> suspicious messages (possibly with undisplayable symbols highlighted)
      <br>(2) hope that at some point upstream will implement context for signing
      <br>But really this boils down to too much effort for questionable profit.
    </td>
  </tr>
  <tr id="first">
    <th  rowspan="5">Incentive: Low</th>
    <td>3-C1 Observe user transaction.
      <br>Transactions are on chain anyways, only trouble is association of address with user.
      <br>See 1-C1/2-C1.
    </td>
    <td>2-A1 Destroy or block Kampela.
      <br>Counter:
      <br>Back up secrets and derivations safely. Make Kampela sufficiently cheap to be replaceable.
    </td>
    <td>6-A1 Engage through companion self-destruction, derivations damage or tool damage.
      <br>Counter:
      <br>(1) initiate self-destruction from Kampela? no api for remote?
    </td>
  </tr>
  <tr>
    <td>3-C2 Observe user public key.
      <br>See 3-C1.
    </td>
    <td>2-A2 Delete or damage the secret.
      <br>Counter:
      <br>(1) user is encouraged to back up the secret
      <br>(2) freeze secret into memory
    </td>
    <td rowspan="4"></td>
  </tr>
  <tr>
    <td>4-C1 Discover log or public keys through the app.
      <br>Counter:
      <br>(1) app may have an association of some public keys; separate app is suggested for Super Secret Derivation;
      <br>(2) log for each public key is already on-chain and public
    </td>
    <td>2-A3 Delete address.
      <br>Counter:
      <br>(1) encourage back-up
    </td>
  </tr>
  <tr>
    <td>4-A1 Trick user to break Kampela.
      <br>Counter:
      <br>(1) Backups. And cheap enough Kampela.
    </td>
    <td>2-A4 Delete specs.
      <br>Counter:
      <br>(1) Only possible on already unlocked Kampela.
    </td>
  </tr>
  <tr>
    <td></td>
    <td>3-A1 EMP.
      <br>Counter:
      <br>(1) Backups. And cheap enough Kampela.
    </td>
  </tr>
</table>

There must be a set of checks for Kampela in place.

1. Code/design are audited, well, at least looked into, as any open source.
2. Factory receives chips as designed.
3. Assembly in factory is as designed.
4. Firmware in factory is as designed.
5. Assembly/hardware remains the same while in transit to user.
6. Firmware same remains the same while in transit to user.
7. Assembly/hardware can not be swapped quickly and discreetly while Kampela is with user.
8. Firmware can not be swapped quickly and discreetly while Kampela is with user.

Summary of secrets in Kampela.

1. Part of chip own secret key, as set in chip factory.
2. Part of chip own secret key, added in Kampela assembly factory. As the Kampela exits the Kampela assembly factory, the chip secret key is complete, and factory can record the Kampela public key. User checks that the chip is genuine by signing a challenge with chip secret key and verifying with public key known at Kampela factory. This covers S5 at least.
</body>
</html>
