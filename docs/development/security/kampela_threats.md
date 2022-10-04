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

<table>
  <tr>
    <th></th>
    <th>Effort: Low</th>
    <th>Effort: Medium</th>
    <th>Effort: High</th>
  </tr>
  <tr id="first">
    <th  rowspan="20">Incentive: High</th>
    <td  rowspan="20"></td>
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
      <br>Changing or resetting the secret allows to change the pin if not yet frozen.
    </td>
  </tr>
  <tr>
    <td>5-I3 User creates address in planted Kampela with unknown secret, attacker gets funds.
      <br>Counter:
      <br>(1) personalized Kampela
    </td>
    <td>1-I2 Disassemble device, read secret instrumentally.
      <br>Counter:
      <br>(1) using PUF, breaking crypto chip would damage the secret
      <br>(2) trustworthy crypto chip
    </td>
  </tr>
  <tr>
    <td>5-A1 User creates address in planted Kampela with unknown secret, attacker demands ransom.
      <br>Counter:
      <br>(1) personalized Kampela
    </td>
    <td>1-I3 Proxy address by breaking the pin.
      <br>Counter: same as 1-I1/2-I1
    </td>
  </tr>
  <tr>
    <td>6-I1 Manipulate companion into making derivation with unreadable symbols in path and/or unknown to user, ransom.
      <br>Counter:
      <br>(1) make Kampela store all created derivations
      <br>(2) additionally display derivation path as set of hex characters from Kampela
    </td>
    <td>1-I4 Take device already unlocked by user.
      <br>Counter:
      <br>(1) Kampela is active for a very short time after disconnect with companion device,
      <br>can add the switch that powers down the device if the field is lost.
      <br>(2) demand pin second time when accessing secret
      <br>(3) demand pin when adding data or signing transaction
      <br>This still leaves uncovered removal/swap of the specs and derivation. So:
      <br>(4) allow device to receive any payloads only from the introduced companion devices
    </td>
  </tr>
  <tr>
    <td>6-I3 Manipulate companion into swapping the transaction content.
      <br>Counter:
      <br>(1) parse transaction on Kampela and display it to user, so that user signs definitely what was parsed
      <br>(2) forbid blind signing
    </td>
    <td>2-I4 Swap secret unbeknownst to user, catch funds on account unrelated to user.
      <br>Counter:
      <br>(1) user is encouraged to freeze the secret (reminders, instructions)
      <br>(2) once the secret is frozen, it could not be changed
    </td>
  </tr>
  <tr>
    <td  rowspan="15"></td>
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
    <td>4-I6 Install damaged firmware on Kampela suring the update.
      <br> Counter:
      <br>(1) block updates except in Kampela "hacker edition" (that is not expected to be used for real secrets)
    </td>
  </tr>
  <tr>
    <td>5-I1 Plant malicious Kampela replica instead of used Kampela, make user re-enter the secret, steal or expose or misuse it.
      <br> Counter:
      <br>(1) user instructions - secret re-entry must be highly suspicious
      <br>(2) personalize Kampela
      <br>(3) introduce Kampela to the companion device, so that the companion device also would
      <br> have to be swapped in this case or will notify user of the Kampela swap
    </td>
  </tr>
  <tr>
    <td>5-I2 Plant malicious Kampela that demands pin and retranslates it, steal the original Kampela, get both the Kampela and the pin.
      <br> Counter:
      <br>(1) personalize Kampela
      <br>(2) introduce Kampela to the companion device, so that the companion device also would
      <br> have to be swapped in this case or will notify user of the Kampela swap (flimsy,
      <br> because responsive part is removeable from Kampela, Kampela itself is silent)
    </td>
  </tr>
  <tr>
    <td>7-I1 Replica with retranslator to steal the secret.
      <br> Counter (to demonstrate that the chip is genuine):
      <br>(1) checkable genuine chip:
      <br>Kampela must be able to sign a verification challenge with its internal secret.
      <br>Internal secret is intrinsic to a device and factory known only a public key from it.
      <br>Maybe part of the internal is from chip itself and part is from Kampela assembly,
      <br>so that the entire secret is not known unless the two cooperate to know it.
      <br>(2) visibly intact casing
      <br>(3) QA and traceability if manufacturer goes rogue
    </td>
  </tr>
  <tr>
    <td>7-I2 Replica with damaged RNG.
      <br>Counter:
      <br>(1) checkable genuine chip, see 7-I1
    </td>
  </tr>
  <tr>
    <td>7-I3 Replica with damaged crypto libraries.
      <br>Counter:
      <br>(1) sign reproducible builds, check signature on-board, etc.
    </td>
  </tr>
  <tr>
    <td>7-I4 Replica with malicious firmware.
      <br>Counter:
      <br>(1) use stable build, check build signature
    </td>
  </tr>
  <tr>
    <td>7-I5 Steal secret stored in damaged device.
      <br>Counter:
      <br>(1) verify chip authenticity
    </td>
  </tr>
  <tr id="first">
    <th  rowspan="5">Incentive: Medium</th>
    <td  rowspan="5"></td>
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
    <td>5-C1 Plant replica Kampela with a tracking device.
      <br>Counter:
      <br>(1) personalized Kampela
    </td>
    <td>4-I5 Bytes and collision.
      <br>Counter:
      <br>(1) display message and hexadecimal payload to protect from outrageously
      <br> suspicious messages (possibly with undisplayable symbols highlighted)
      <br>(2) hope that at some point upstream will implement context for signing
      <br>But really this boils down to too much effort for questionable profit.
    </td>
  </tr>
  <tr>
    <td>6-I2 Sign with wrong public key (wrong derivation).
      <br>Counter:
      <br>(1) Kampela displays and user is expected to check the signing address
    </td>
    <td></td>
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
    <td>4-A1 Trick user to break Kampela.
      <br>Counter:
      <br>(1) Backups. And cheap enough Kampela.
    </td>
    <td>2-A3 Delete address.
      <br>Counter:
      <br>(1) encourage back-up
    </td>
  </tr>
  <tr>
    <td>6-C1 Discover log or public keys through the app.
      <br>Counter:
      <br>(1) app may have an association of some public keys; separate app is suggested for Super Secret Derivation;
      <br>(2) log for each public key is already on-chain and public
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
</body>
</html>
