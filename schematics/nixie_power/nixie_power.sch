EESchema Schematic File Version 4
EELAYER 30 0
EELAYER END
$Descr A4 11693 8268
encoding utf-8
Sheet 1 1
Title ""
Date ""
Rev ""
Comp ""
Comment1 ""
Comment2 ""
Comment3 ""
Comment4 ""
$EndDescr
$Comp
L power:GND #PWR03
U 1 1 60241D6B
P 4950 5350
F 0 "#PWR03" H 4950 5100 50  0001 C CNN
F 1 "GND" H 4955 5177 50  0000 C CNN
F 2 "" H 4950 5350 50  0001 C CNN
F 3 "" H 4950 5350 50  0001 C CNN
	1    4950 5350
	1    0    0    -1  
$EndComp
$Comp
L Device:R R3
U 1 1 602416D2
P 5550 2400
F 0 "R3" H 5620 2446 50  0000 L CNN
F 1 "47R" H 5620 2355 50  0000 L CNN
F 2 "Resistor_THT:R_Axial_DIN0207_L6.3mm_D2.5mm_P10.16mm_Horizontal" V 5480 2400 50  0001 C CNN
F 3 "~" H 5550 2400 50  0001 C CNN
	1    5550 2400
	1    0    0    -1  
$EndComp
$Comp
L Device:R R4
U 1 1 60241AB3
P 7200 2150
F 0 "R4" H 7270 2196 50  0000 L CNN
F 1 "220K" H 7270 2105 50  0000 L CNN
F 2 "Resistor_THT:R_Axial_DIN0207_L6.3mm_D2.5mm_P10.16mm_Horizontal" V 7130 2150 50  0001 C CNN
F 3 "~" H 7200 2150 50  0001 C CNN
	1    7200 2150
	1    0    0    -1  
$EndComp
$Comp
L Device:R R1
U 1 1 60241D16
P 3900 3700
F 0 "R1" H 3970 3746 50  0000 L CNN
F 1 "1K" H 3970 3655 50  0000 L CNN
F 2 "Resistor_THT:R_Axial_DIN0207_L6.3mm_D2.5mm_P10.16mm_Horizontal" V 3830 3700 50  0001 C CNN
F 3 "~" H 3900 3700 50  0001 C CNN
	1    3900 3700
	1    0    0    -1  
$EndComp
$Comp
L Device:R R2
U 1 1 60241EEB
P 3900 4450
F 0 "R2" H 3970 4496 50  0000 L CNN
F 1 "10K" H 3970 4405 50  0000 L CNN
F 2 "Resistor_THT:R_Axial_DIN0207_L6.3mm_D2.5mm_P10.16mm_Horizontal" V 3830 4450 50  0001 C CNN
F 3 "~" H 3900 4450 50  0001 C CNN
	1    3900 4450
	1    0    0    -1  
$EndComp
$Comp
L Timer:NE555P U1
U 1 1 6023F90D
P 4950 4050
F 0 "U1" H 5118 4631 50  0000 C CNN
F 1 "NE555P" H 5118 4540 50  0000 C CNN
F 2 "Package_DIP:DIP-8_W7.62mm" H 5600 3650 50  0001 C CNN
F 3 "http://www.ti.com/lit/ds/symlink/ne555.pdf" H 5800 3650 50  0001 C CNN
	1    4950 4050
	-1   0    0    -1  
$EndComp
$Comp
L Transistor_FET:IRF740 Q1
U 1 1 60248333
P 4900 2100
F 0 "Q1" H 5104 2146 50  0000 L CNN
F 1 "IRF740" H 5104 2055 50  0000 L CNN
F 2 "Package_TO_SOT_THT:TO-220-3_Vertical" H 5150 2025 50  0001 L CIN
F 3 "http://www.vishay.com/docs/91054/91054.pdf" H 4900 2100 50  0001 L CNN
	1    4900 2100
	1    0    0    -1  
$EndComp
$Comp
L Device:CP_Small C2
U 1 1 6024DAF1
P 4300 2450
F 0 "C2" H 4016 2496 50  0000 L CNN
F 1 "470uF/250V" H 4252 2405 50  0000 R CNN
F 2 "Capacitor_THT:CP_Radial_D10.0mm_P5.00mm" H 4300 2450 50  0001 C CNN
F 3 "~" H 4300 2450 50  0001 C CNN
	1    4300 2450
	1    0    0    -1  
$EndComp
$Comp
L Device:C_Small C3
U 1 1 6024E564
P 5550 1900
F 0 "C3" H 5642 1946 50  0000 L CNN
F 1 "100p/400V" H 5642 1855 50  0000 L CNN
F 2 "Capacitor_THT:C_Disc_D10.5mm_W5.0mm_P10.00mm" H 5550 1900 50  0001 C CNN
F 3 "~" H 5550 1900 50  0001 C CNN
	1    5550 1900
	1    0    0    -1  
$EndComp
$Comp
L Device:D D1
U 1 1 60250B98
P 5950 1600
F 0 "D1" H 5950 1817 50  0000 C CNN
F 1 "400V/2A" H 5950 1726 50  0000 C CNN
F 2 "Diode_THT:D_DO-27_P12.70mm_Horizontal" H 5950 1600 50  0001 C CNN
F 3 "~" H 5950 1600 50  0001 C CNN
	1    5950 1600
	-1   0    0    -1  
$EndComp
$Comp
L Device:CP_Small C4
U 1 1 60259785
P 6300 2150
F 0 "C4" H 6388 2196 50  0000 L CNN
F 1 "15u/350V" H 6388 2105 50  0000 L CNN
F 2 "Capacitor_THT:CP_Radial_D13.0mm_P7.50mm" H 6300 2150 50  0001 C CNN
F 3 "~" H 6300 2150 50  0001 C CNN
	1    6300 2150
	1    0    0    -1  
$EndComp
$Comp
L Device:C_Small C1
U 1 1 6026BAA9
P 3900 5000
F 0 "C1" H 3992 5046 50  0000 L CNN
F 1 "2n2" H 3992 4955 50  0000 L CNN
F 2 "Capacitor_THT:C_Disc_D7.5mm_W2.5mm_P5.00mm" H 3900 5000 50  0001 C CNN
F 3 "~" H 3900 5000 50  0001 C CNN
	1    3900 5000
	1    0    0    -1  
$EndComp
$Comp
L Transistor_BJT:BC547 Q2
U 1 1 6026CADB
P 6100 4900
F 0 "Q2" H 6291 4946 50  0000 L CNN
F 1 "BC547" H 6291 4855 50  0000 L CNN
F 2 "Package_TO_SOT_THT:TO-92_Inline" H 6300 4825 50  0001 L CIN
F 3 "https://www.onsemi.com/pub/Collateral/BC550-D.pdf" H 6100 4900 50  0001 L CNN
	1    6100 4900
	-1   0    0    -1  
$EndComp
$Comp
L Device:R_POT_US RV1
U 1 1 6026DCC6
P 6800 4100
F 0 "RV1" H 6732 4146 50  0000 R CNN
F 1 "1k" H 6732 4055 50  0000 R CNN
F 2 "Potentiometer_THT:Potentiometer_Piher_PT-10-V10_Vertical" H 6800 4100 50  0001 C CNN
F 3 "~" H 6800 4100 50  0001 C CNN
	1    6800 4100
	-1   0    0    -1  
$EndComp
Wire Wire Line
	6100 1600 6300 1600
Wire Wire Line
	6300 1600 6300 2050
Wire Wire Line
	6300 1600 7200 1600
Connection ~ 6300 1600
Wire Wire Line
	7200 1600 7200 2000
Wire Wire Line
	5800 1600 5550 1600
Wire Wire Line
	5550 1600 5550 1800
Wire Wire Line
	5000 1900 5000 1600
Wire Wire Line
	5000 1600 5550 1600
Connection ~ 5550 1600
$Comp
L power:GND #PWR04
U 1 1 6027491E
P 5000 2700
F 0 "#PWR04" H 5000 2450 50  0001 C CNN
F 1 "GND" H 5005 2527 50  0000 C CNN
F 2 "" H 5000 2700 50  0001 C CNN
F 3 "" H 5000 2700 50  0001 C CNN
	1    5000 2700
	1    0    0    -1  
$EndComp
Wire Wire Line
	5000 2300 5000 2650
Connection ~ 5000 2650
Wire Wire Line
	5000 2650 5000 2700
Wire Wire Line
	4300 2550 4300 2650
Wire Wire Line
	4300 2650 5000 2650
Wire Wire Line
	4300 2350 4300 1600
Wire Wire Line
	5550 2550 5550 2650
Wire Wire Line
	5550 2650 5000 2650
Wire Wire Line
	5550 2250 5550 2000
Wire Wire Line
	6300 2250 6300 2650
Wire Wire Line
	6300 2650 5550 2650
Connection ~ 5550 2650
$Comp
L pspice:INDUCTOR L1
U 1 1 602850E7
P 4650 1600
F 0 "L1" H 4650 1815 50  0000 C CNN
F 1 "100mH" H 4650 1724 50  0000 C CNN
F 2 "Inductor_THT:L_Toroid_Vertical_L14.7mm_W8.6mm_P5.58mm_Pulse_KM-1" H 4650 1600 50  0001 C CNN
F 3 "~" H 4650 1600 50  0001 C CNN
	1    4650 1600
	1    0    0    -1  
$EndComp
Wire Wire Line
	4400 1600 4300 1600
Connection ~ 4300 1600
Wire Wire Line
	4900 1600 5000 1600
Connection ~ 5000 1600
Wire Wire Line
	4950 3650 4950 3400
Wire Wire Line
	5450 4250 5650 4250
Wire Wire Line
	5650 4250 5650 3400
Wire Wire Line
	5650 3400 4950 3400
Connection ~ 4950 3400
Wire Wire Line
	4450 3850 4450 2100
Wire Wire Line
	4450 2100 4700 2100
Wire Wire Line
	3900 3550 3900 3400
Wire Wire Line
	3900 3400 4950 3400
Wire Wire Line
	3900 3850 3900 4050
Wire Wire Line
	3900 4050 4450 4050
Wire Wire Line
	3900 4300 3900 4050
Connection ~ 3900 4050
Wire Wire Line
	3900 4600 3900 4750
Wire Wire Line
	3900 5100 3900 5350
Wire Wire Line
	5450 4050 6000 4050
Wire Wire Line
	4450 4250 4450 4600
Wire Wire Line
	4450 4750 3900 4750
Connection ~ 3900 4750
Wire Wire Line
	3900 4750 3900 4900
Wire Wire Line
	5450 3850 5550 3850
Wire Wire Line
	5550 3850 5550 4600
Wire Wire Line
	5550 4600 4450 4600
Connection ~ 4450 4600
Wire Wire Line
	4450 4600 4450 4750
Wire Wire Line
	4950 4450 4950 5350
Connection ~ 4950 5350
Wire Wire Line
	3900 5350 4950 5350
Wire Wire Line
	4950 5350 6000 5350
Wire Wire Line
	6000 5100 6000 5350
Wire Wire Line
	6000 4050 6000 4700
Wire Wire Line
	6800 5350 6000 5350
Connection ~ 6000 5350
NoConn ~ 4950 4600
NoConn ~ 5550 4250
NoConn ~ 5550 4050
NoConn ~ 5650 4050
NoConn ~ 4450 2650
NoConn ~ 4450 3400
$Comp
L Device:R R5
U 1 1 6028E1C3
P 6800 4900
F 0 "R5" H 6870 4946 50  0000 L CNN
F 1 "470" H 6870 4855 50  0000 L CNN
F 2 "Resistor_THT:R_Axial_DIN0207_L6.3mm_D2.5mm_P10.16mm_Horizontal" V 6730 4900 50  0001 C CNN
F 3 "~" H 6800 4900 50  0001 C CNN
	1    6800 4900
	1    0    0    -1  
$EndComp
Wire Wire Line
	6800 5050 6800 5350
Wire Wire Line
	6800 4250 6800 4750
Wire Wire Line
	6650 4100 6450 4100
Wire Wire Line
	6450 4100 6450 4900
Wire Wire Line
	6450 4900 6300 4900
Wire Wire Line
	7200 2300 7200 3600
Wire Wire Line
	7200 3600 6800 3600
Wire Wire Line
	6800 3600 6800 3950
$Comp
L Connector:Screw_Terminal_01x02 J2
U 1 1 6029BB9F
P 7650 1600
F 0 "J2" H 7730 1592 50  0000 L CNN
F 1 "Screw_Terminal_01x02" H 7730 1501 50  0000 L CNN
F 2 "TerminalBlock:TerminalBlock_bornier-2_P5.08mm" H 7650 1600 50  0001 C CNN
F 3 "~" H 7650 1600 50  0001 C CNN
	1    7650 1600
	1    0    0    -1  
$EndComp
$Comp
L Connector:Screw_Terminal_01x02 J1
U 1 1 6029C839
P 3400 1600
F 0 "J1" H 3318 1817 50  0000 C CNN
F 1 "Screw_Terminal_01x02" H 3318 1726 50  0000 C CNN
F 2 "TerminalBlock:TerminalBlock_bornier-2_P5.08mm" H 3400 1600 50  0001 C CNN
F 3 "~" H 3400 1600 50  0001 C CNN
	1    3400 1600
	-1   0    0    -1  
$EndComp
Wire Wire Line
	3600 1600 3900 1600
Wire Wire Line
	7450 1600 7200 1600
Connection ~ 7200 1600
Wire Wire Line
	3900 1600 3900 3400
Connection ~ 3900 1600
Wire Wire Line
	3900 1600 4300 1600
Connection ~ 3900 3400
$Comp
L power:GND #PWR0101
U 1 1 602A299E
P 3700 1700
F 0 "#PWR0101" H 3700 1450 50  0001 C CNN
F 1 "GND" H 3705 1527 50  0000 C CNN
F 2 "" H 3700 1700 50  0001 C CNN
F 3 "" H 3700 1700 50  0001 C CNN
	1    3700 1700
	1    0    0    -1  
$EndComp
$Comp
L power:GND #PWR0102
U 1 1 602A30F6
P 7350 1700
F 0 "#PWR0102" H 7350 1450 50  0001 C CNN
F 1 "GND" H 7355 1527 50  0000 C CNN
F 2 "" H 7350 1700 50  0001 C CNN
F 3 "" H 7350 1700 50  0001 C CNN
	1    7350 1700
	1    0    0    -1  
$EndComp
Wire Wire Line
	7450 1700 7350 1700
Wire Wire Line
	3600 1700 3700 1700
$EndSCHEMATC