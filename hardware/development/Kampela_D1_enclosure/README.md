# D1 iteration mechanical design progress:

## Aluminum body frame

In the process of working on the flooded version of Kampela, the lack of strength of the flooded device was revealed - both in terms of the integrity of the board and in terms of the integrity of the display. Also, in the process of working on the molding of the board with the display, problems with stable and accurate positioning of the display during molding were identified. To avoid these problems, it was decided to add a rigid frame to the layout, which would increase the rigidity of the structure and would allow the elements to be reliably positioned when pouring the compound. Aluminum was chosen as the frame material for the time being, and non-conductive, lighter and more corrosion-resistant plastic materials are considered in future. The influence of metal on the efficiency of wireless power transmission was experimentally investigated and measures were taken to increase efficiency - slots were added to avoid closed turns.

## Active NFC dummy tag inlay

As an additional authentication factor, as well as for direct communication between the NFC smartphone and Kampela, we use an NFC memory chip with an independent antenna. It was decided to implement this scheme in the form of a hermetic key fob detachable from the main device, without which Kampela does not allow to perform its functions. The key fob is implemented in the form of a small printed circuit board poured into a transparent compound and placed inside the main Kampela coil. Structurally, the key fob is implemented in such a way that it does not fall out of the case when using Kampela directly with a smartphone, but if necessary, it is possible to remove it from the Kampela case for inspection that the circuits are indeed indepentent, and, if we manage to implement proper strong signing in active tag, use the fob as second authorization factor. At the moment, the most optimal design of the tag was chosen with a transparent plastic base, a printed circuit board of the tag embedded in it, and the subsequent hermetic filling of the tag.

## Filling and sealing

Experiments with sealed PCB potting showed good technology potential for the device, but also revealed several technical problems:

- When pouring, accurate and reliable positioning of the display and printed circuit boards are required, otherwise, they may "float" in the volume of the poured compound and change their final position.

- Some fill elements are very thin, such as the protective damper around the display and the area around the display and touch screen flex cables. The small thickness makes it difficult to pour without bubbles and cavities at low filling pressures. The use of high pressures for filling electronics and e-inc displays is not possible

- Flexible cables of the display and touch-screen require fixing before pouring in order not to go to the pouring surface and form air pockets.

Based on these issues, it was decided to use a rigid aluminum frame to solve these problems, and to perform a hermetic filling from the back of the device. At the same time, the device remains transparent and sealed and does not allow hidden modifications. In the next milestones, we will work on optimizing the layout and design of Kampela to make it more technologically advanced and more user-friendly.