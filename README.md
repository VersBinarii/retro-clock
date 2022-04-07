# Retro Clock

Nixie clock based on stm32f103 "BluePill"

# Description

Its based on stm32f103 RTC peripheral.
The nixie valves are driven with 74LS145 BCD to decimal decoder
The time is outputed through the NIXIE as wel as through the smal I2C based OLED.

# Operation

Long press 'Ok' putton to enter into edit mode. 
Then short press selects the digit to edit. 'Left' and 'Right' buttons increase/decrease the value.
Long press 'Ok' again to confirm new time. 
