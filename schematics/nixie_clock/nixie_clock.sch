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
L MCU_ST_STM32F1:STM32F103C8Tx U?
U 1 1 609BDFAF
P 5420 3500
F 0 "U?" H 5370 1911 50  0000 C CNN
F 1 "STM32F103C8Tx" H 5370 1820 50  0000 C CNN
F 2 "Package_QFP:LQFP-48_7x7mm_P0.5mm" H 4820 2100 50  0001 R CNN
F 3 "http://www.st.com/st-web-ui/static/active/en/resource/technical/document/datasheet/CD00161566.pdf" H 5420 3500 50  0001 C CNN
	1    5420 3500
	1    0    0    -1  
$EndComp
Text GLabel 6710 4400 2    50   Input ~ 0
LEFT_BTN
Text GLabel 6710 4500 2    50   Input ~ 0
RIGHT_BTN
Text GLabel 6710 4800 2    50   Input ~ 0
OK_BTN
Text GLabel 4290 4200 0    50   Input ~ 0
BME_SDA
Text GLabel 4290 4100 0    50   Input ~ 0
BME_SCL
Wire Wire Line
	6710 4800 6020 4800
Wire Wire Line
	6020 4500 6710 4500
Wire Wire Line
	6710 4400 6020 4400
Wire Wire Line
	4720 4100 4290 4100
Wire Wire Line
	4720 4200 4290 4200
Text GLabel 6700 3800 2    50   Input ~ 0
LCD_CLK
Text GLabel 6700 3900 2    50   Input ~ 0
LCD_MISO
Text GLabel 6700 4000 2    50   Input ~ 0
LCD_MOSI
Wire Wire Line
	6710 3800 6020 3800
Wire Wire Line
	6700 3900 6020 3900
Wire Wire Line
	6710 4000 6020 4000
$EndSCHEMATC
