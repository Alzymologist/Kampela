# D1 iteration electronics design progress: 

## Display and touchscreen

For ease of use with the device, this iteration used a 2.7" e-inc touchscreen display. We also examined e-inc displays of other diagonals in terms of usability and usability in the Kampela layout, as well as displays powered by memory technology displays. Alternative display manufacturers with the same parameters were also found to ensure supply reliability.

Displays based on memory display technology look very promising - the refresh rate compared to e-inc allows for a more responsive touch interface. The problem lies in the small number of manufacturers of suitable displays found and the lack of available touch screens for such displays. Security aspects of using this technology should also be carefully analyzed, as sensitive key material does indeed pass through memory display’s memory cells, which have some limited non-volatility. Yet in the future iterations, we want to work more with memory display and try to implement a fast, responsive touch interface, possibly with color.

## Separate layout of the coil and main board

In this iteration, the NFC coil has been separated from the main board. This was done based on the following tasks:

1. When placing the board with the display and the coil in the case, taking into account the thickness of the display, the protection of the display, and the case, it turns out that the coil moves away from the phone at a distance of more than 3mm, which negatively affects the efficiency of wireless power transmission and the overall power reserve in Kampela. The division of the coil and the main board into 2 boards in this case allows you to place the coil in the case as close as possible to the plane of the smartphone while using the device remains just as convenient.

2. The separate layout of the coil and the main board allows you to experiment with different coils in the package without changing the main board. In the next iteration, we will be able to choose the optimal coil option in this way.

## Self-destruct mechanism

In latest iteration, device self-destruct chains were added, activated by the user either by physically damaging the screen or other area of casing and then exposing Kampela to NFC field; we would experiment to determine most reliable and convenient method in the next milestones. Self-destruct indication circuits have also been added so that the user can visually check that the device cannot be used for its intended purpose after self-destruction: instead of RED power overload LED, a BLUE LED indicating successful processor wipe would be illuminated.

## Separation of display and touch screen resets

In the first prototype, lots of cross-talk was found between display and touch panel circuits: power dip on screen refresh occasionally triggers touch touch events, i2c line gets locked, etc. Thus the RESET circuits of the display and the touch screen were separated due to mutual influence during operation. The power circuits of the display and the touch screen were also separated and optimized to avoid their negative influence on each other. Furthermore, this would improve power management of Kampela, as with this separation touch panel could be disabled during screen update, significantly reducing power drain and subsequently brown-out probability during normal operation.

## Improvements to component patterns and board layout

In this iteration, the documentation for the display, touchscreen, cables, and other components has been clarified, and the component patterns have been corrected and optimized. The layout of the board has also been optimized so that the cables take a mechanically unstressed position, and the mounting of components on the board has also been optimized.