# CoilDisp

This devboard uses `STN32L432KCU` mc to test power harvesting to power a e-ink screen.

## Power harvest

The board supports connection of printed coil or external coil. Additionally, a UFL connector for VNA for coil model parameters analysis is implemented

## Power storage

The power is stored in smd ceramic capacitors; external connections for electrolytic capacitors to support londer unpowered states and balance high poewr consumption during screen refresh cycle are implemented

## Screen

The component for testing is PervasiveDisplays model E2154CS0C1

[Datasheet](https://www.pervasivedisplays.com/wp-content/uploads/2021/12/1P255-00_03_E2154CS0C1_202111203doc.pdf) and [programming guide](https://docs.pervasivedisplays.com/epd-usage/epd-driving-sequence/small-epds) are available at PervasiveDisplays website.
