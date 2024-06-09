# LEDs
set_property -dict { \
    PACKAGE_PIN M14 \
    IOSTANDARD LVCMOS33 \
  } [get_ports { io_leds[0] }];
set_property -dict { \
    PACKAGE_PIN M15 \
    IOSTANDARD LVCMOS33 \
  } [get_ports { io_leds[1] }];
set_property -dict { \
    PACKAGE_PIN G14 \
    IOSTANDARD LVCMOS33 \
  } [get_ports { io_leds[2] }];
set_property -dict { \
    PACKAGE_PIN D18 \
    IOSTANDARD LVCMOS33 \
  } [get_ports { io_leds[3] }];

# LCD
set_property -dict { \
    PACKAGE_PIN Y17 \
    IOSTANDARD LVCMOS33 \
    DRIVE 12 \
  } [get_ports { io_lcd_reset }];
set_property -dict { \
    PACKAGE_PIN H15 \
    IOSTANDARD LVCMOS33 \
    DRIVE 8 \
  } [get_ports { io_lcd_dim }];

# LCD Cluster 0
set_property -dict { \
    PACKAGE_PIN R14 \
    IOSTANDARD LVCMOS33 \
    SLEW FAST \
    DRIVE 16 \
  } [get_ports { io_lcdSpi0_scl }];
set_property -dict { \
    PACKAGE_PIN V12 \
    IOSTANDARD LVCMOS33 \
    SLEW FAST \
    DRIVE 16 \
  } [get_ports { io_lcdSpi0_sda }];
set_property -dict { \
    PACKAGE_PIN V17 \
    IOSTANDARD LVCMOS33 \
    SLEW FAST \
    DRIVE 16 \
  } [get_ports { io_lcdSpi0_cs[0] }];
set_property -dict { \
    PACKAGE_PIN P14 \
    IOSTANDARD LVCMOS33 \
    SLEW FAST \
    DRIVE 16 \
  } [get_ports { io_lcdSpi0_cs[1] }];
set_property -dict { \
    PACKAGE_PIN V18 \
    IOSTANDARD LVCMOS33 \
    SLEW FAST \
    DRIVE 16 \
  } [get_ports { io_lcdSpi0_dc }];


# LCD Cluster 1
set_property -dict { \
    PACKAGE_PIN W16 \
    IOSTANDARD LVCMOS33 \
    SLEW FAST \
    DRIVE 16 \
  } [get_ports { io_lcdSpi1_scl }];
set_property -dict { \
    PACKAGE_PIN J15 \
    IOSTANDARD LVCMOS33 \
    SLEW FAST \
    DRIVE 16 \
  } [get_ports { io_lcdSpi1_sda }];
set_property -dict { \
    PACKAGE_PIN T14 \
    IOSTANDARD LVCMOS33 \
    SLEW FAST \
    DRIVE 16 \
  } [get_ports { io_lcdSpi1_cs[0] }];
set_property -dict { \
    PACKAGE_PIN U14 \
    IOSTANDARD LVCMOS33 \
    SLEW FAST \
    DRIVE 16 \
  } [get_ports { io_lcdSpi1_cs[1] }];
set_property -dict { \
    PACKAGE_PIN U15 \
    IOSTANDARD LVCMOS33 \
    SLEW FAST \
    DRIVE 16 \
  } [get_ports { io_lcdSpi1_dc }];


# LCD Cluster 2
set_property -dict { \
    PACKAGE_PIN W14 \
    IOSTANDARD LVCMOS33 \
    SLEW FAST \
    DRIVE 16 \
  } [get_ports { io_lcdSpi2_scl }];
set_property -dict { \
    PACKAGE_PIN Y14 \
    IOSTANDARD LVCMOS33 \
    SLEW FAST \
    DRIVE 16 \
  } [get_ports { io_lcdSpi2_sda }];
set_property -dict { \
    PACKAGE_PIN V20 \
    IOSTANDARD LVCMOS33 \
    SLEW FAST \
    DRIVE 16 \
  } [get_ports { io_lcdSpi2_cs[0] }];
set_property -dict { \
    PACKAGE_PIN W18 \
    IOSTANDARD LVCMOS33 \
    SLEW FAST \
    DRIVE 16 \
  } [get_ports { io_lcdSpi2_cs[1] }];
set_property -dict { \
    PACKAGE_PIN W19 \
    IOSTANDARD LVCMOS33 \
    SLEW FAST \
    DRIVE 16 \
  } [get_ports { io_lcdSpi2_dc }];


# LCD Cluster 3
set_property -dict { \
    PACKAGE_PIN T12 \
    IOSTANDARD LVCMOS33 \
    SLEW FAST \
    DRIVE 16 \
  } [get_ports { io_lcdSpi3_scl }];
set_property -dict { \
    PACKAGE_PIN U12 \
    IOSTANDARD LVCMOS33 \
    SLEW FAST \
    DRIVE 16 \
  } [get_ports { io_lcdSpi3_sda }];
set_property -dict { \
    PACKAGE_PIN T20 \
    IOSTANDARD LVCMOS33 \
    SLEW FAST \
    DRIVE 16 \
  } [get_ports { io_lcdSpi3_cs[0] }];
set_property -dict { \
    PACKAGE_PIN Y18 \
    IOSTANDARD LVCMOS33 \
    SLEW FAST \
    DRIVE 16 \
  } [get_ports { io_lcdSpi3_cs[1] }];
set_property -dict { \
    PACKAGE_PIN Y19 \
    IOSTANDARD LVCMOS33 \
    SLEW FAST \
    DRIVE 16 \
  } [get_ports { io_lcdSpi3_dc }];
