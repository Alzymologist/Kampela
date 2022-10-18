## Ledger insides

official website:

<https://support.ledger.com/hc/article_attachments/4406450402961/LedgerNanoX_PCB_rev1.jpg>

<https://support.ledger.com/hc/article_attachments/4404778576401/front_pcb.png>

<https://support.ledger.com/hc/article_attachments/4410975918481/revision_3_front.jpg>

<https://support.ledger.com/hc/article_attachments/4410972052497/revision_4_front.jpg>

better quality pic: <https://twitter.com/walletfail/status/1129328479865065473>

## Recognizeable details

1. <https://www.mouser.fi/ProductDetail/STMicroelectronics/STM32WB55CCU6?qs=vLWxofP3U2waGkUGKmQZWQ%3D%3D>

2. Ledger argues extensively that the secure element is more preferable than safe memory chip:
<https://www.ledger.com/academy/security/not-all-chips-are-born-equal>

Secure element is not open-source, though.

The security chip in pics referenced above is apparently the one Ledger-marked.

"Ledger J2M...", produced by ST.

It is possible that the chip is a close relative of this one:

<https://www.st.com/en/secure-mcus/st33j2m0.html>

3. There are no hardware-based Schnorkkel supporting ICs, thus the making and using of at least Sr25519 pairs will possibly happen outside the security chip.

We would have to rely on memory zeroize in the chip that does the calculations.

Also, see this: <https://thenextweb.com/news/ledger-nano-s-hack-cryptocurrency>

(And how do *we* ensure that the chip is genuine?)

## PUF chips

Maxim provides a number of PUF chips:

- <https://datasheets.maximintegrated.com/en/ds/MAX32520.pdf> microcontroller to combine secret storage and calculations

- <https://datasheets.maximintegrated.com/en/ds/DS28E38.pdf> secure verifiable storage, with ECDSA; calculate pairs externally;

- <https://datasheets.maximintegrated.com/en/ds/DS28E50.pdf> secure verifiable storage, with SHA-3; calculate pairs externally;

Note. How good is PUF actually? (i.e. how much enthropy it really provides? what about these articles claiming PUF can be predicted? what kind of PUF is there after all - check the patent?)

So far was unable to find any other PUF chips.
