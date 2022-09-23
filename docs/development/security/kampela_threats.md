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
    <th  rowspan="15">Incentive: High</th>
    <td  rowspan="15"></td>
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
      <br>(2) discourage blind signing
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
    <td  rowspan="11"></td>
    <td>2-I4 Swap secret unbeknownst to user, catch funds on account unrelated to user.
      <br>Counter:
      <br>(1) user is encouraged to freeze the secret (reminders, instructions)
    </td>
  </tr>
  <tr>
    <td>2-I5 Coerce user to "fix" the secret, manipulate secret on entering, catch funds on account unrelated to user.</td>
  </tr>
  <tr>
    <td>3-I2 Make user sign a malicious transaction (sent with powerful transponder).</td>
  </tr>
  <tr>
    <td>3-I4 Listen to device.</td>
  </tr>
  <tr>
    <td>4-I1 Make user sign proxy (social).</td>
  </tr>
  <tr>
    <td>4-I3 Distribute malicious chain data.</td>
  </tr>
  <tr>
    <td>4-I4 Make user sign transaction blindly (social).</td>
  </tr>
  <tr>
    <td>5-I1 Steal secret (device emission).</td>
  </tr>
  <tr>
    <td>5-I2 Steal secret via damaged RNG.</td>
  </tr>
  <tr>
    <td>5-I3 Steal secret via damaged crypto libraries in Kampela.</td>
  </tr>
  <tr>
    <td>5-I4 Steal secret by swapping the device twice.</td>
  </tr>
  <tr id="first">
    <th  rowspan="4">Incentive: Medium</th>
    <td  rowspan="4"></td>
    <td>1-C1 and 2-C1 Stalk user after discovering all public keys.</td>
    <td>2-I2 Swap specs (steal Kampela).</td>
  </tr>
  <tr>
    <td>2-A5 Copy path from Kampela, delete address, demand ransom to restore.</td>
    <td>2-I3 Swap verifier.</td>
  </tr>
  <tr>
    <td>4-I2 Malicious verifier (social).</td>
    <td>3-I3 Swap specs (transponder).</td>
  </tr>
  <tr>
    <td>6-I2 Sign with wrong public key (wrong derivation).</td>
    <td>3-I3 Bytes and collision.</td>
  </tr>
  <tr id="first">
    <th  rowspan="5">Incentive: Low</th>
    <td>3-C1 Observe user transaction.</td>
    <td>2-A1 Destroy or block Kampela.</td>
    <td>6-A1 Engage through companion self-destruction, derivations damage or tool damage.</td>
  </tr>
  <tr>
    <td>3-C2 Observe user public key.</td>
    <td>2-A2 Delete or damage the secret.</td>
    <td rowspan="4"></td>
  </tr>
  <tr>
    <td>4-C1 Discover log or keys through the app.</td>
    <td>2-A3 Delete address.</td>
  </tr>
  <tr>
    <td>4-A1 Trick user to break Kampela.</td>
    <td>2-A4 Delete specs.</td>
  </tr>
  <tr>
    <td></td>
    <td>3-A1 EMP.</td>
  </tr>
</table> 
