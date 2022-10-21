## Ledger insides

### Known images

official website:

<https://support.ledger.com/hc/article_attachments/4406450402961/LedgerNanoX_PCB_rev1.jpg>

<https://support.ledger.com/hc/article_attachments/4404778576401/front_pcb.png>

<https://support.ledger.com/hc/article_attachments/4410975918481/revision_3_front.jpg>

<https://support.ledger.com/hc/article_attachments/4410972052497/revision_4_front.jpg>

better quality pic: <https://twitter.com/walletfail/status/1129328479865065473>

### Recognizeable details

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

### Maxim

Maxim provides a number of PUF chips:

- <https://datasheets.maximintegrated.com/en/ds/MAX32520.pdf> microcontroller to combine secret storage and calculations

- <https://datasheets.maximintegrated.com/en/ds/DS28E38.pdf> secure verifiable storage, with ECDSA; calculate pairs externally;

- <https://datasheets.maximintegrated.com/en/ds/DS28E50.pdf> secure verifiable storage, with SHA-3; calculate pairs externally;

### Silabs

Silabs provides MCUs with PUF:

<https://www.silabs.com/documents/public/data-sheets/efm32pg23-datasheet.pdf>

(B model)

### Microchip

<https://www.microchip.com/en-us/products/security/secure-mpus>

Microchip claims PUF in MPUs. Some undisclosed tamper protection with docs available apparently only through NDA.
Datasheets discuss only anti-tamper pins.

- <https://ww1.microchip.com/downloads/aemDocuments/documents/CPG/ProductDocuments/ProductBrief/CEC173x_Data_Brief.pdf> PUF specifically claimed here

- FPGA <https://www.microchip.com/en-us/products/fpgas-and-plds/system-on-chip-fpgas/polarfire-soc-fpgas>

### NXP

- <https://www.nxp.com/docs/en/fact-sheet/P71D321.pdf> PUF claimed, need to check actual datasheets.

- <https://www.nxp.com/docs/en/data-sheet/LPC540xx.pdf>

- <https://www.nxp.com/docs/en/data-sheet/LPC54018JxM-LPC54S018JxM.pdf>

- <https://www.nxp.com/docs/en/data-sheet/LPC55S0x_LPC550x_DS.pdf>

- <https://www.nxp.com/docs/en/nxp/data-sheets/LPC55S1x_LPC551x_DS.pdf> (USB)

- <https://www.nxp.com/docs/en/nxp/data-sheets/LPC55S2x_LPC552x_DS.pdf> (USB)

- <https://www.nxp.com/docs/en/nxp/data-sheets/LPC55S6x_DS.pdf> (USB)

Document on usage: <https://www.nxp.com/docs/en/application-note/AN12324.pdf>

### Notes

1. We do not protect the user secret after it was decoded from side channel attacks, because at this point it could be read on backup.
We only protect from the unauthorized decoding itself.

2. How good is PUF actually? (i.e. how much enthropy it really provides? what about these articles claiming PUF can be predicted? what kind of PUFs are there after all in each case?)

3. All microcontrollers are based on 32-bit ARM Cortex-M4F, or -M33, or -M4.

## Other security ICs

### Infineon

1. <https://www.infineon.com/cms/en/product/security-smart-card-solutions/cipurse-products/>
Some chips, for example, CIPURSE have NFC interface, not acceptable.

2. Government ID applications chips have EAL6+ certification.

<https://www.mouser.fi/datasheet/2/196/Infineon_OPTIGA_TRUST_M_SLS32AIA_DataSheet_v03_00_-1927815.pdf>

None of the chips boast PUF.
Datasheets claim "tamper proof NVM". Need additional info on that.

3. Trusted computing modules have EAL4+

<https://www.mouser.fi/datasheet/2/196/Infineon_OPTIGA_TPM_SLB_9673_FW26_DataSheet_v01_01-3006843.pdf>

### STMicroelectronics

<https://www.st.com/en/secure-mcus/secure-hardware-platforms.html>

There is a number of MCUs certified with EAL6+, but no PUF actively presented.

## Other notes

<https://trustedcomputinggroup.org/membership/certification/>

TCG appears to provide good indication of interoperability, but the criteria are quite obscure.
