set_property BITSTREAM.CONFIG.CONFIGRATE 66 [current_design]
# set_property CONFIG_MODE SPIx4 [current_design]
# set_property BITSTREAM.CONFIG.SPI_32BIT_ADDR NO [current_design]
# set_property BITSTREAM.CONFIG.SPI_BUSWIDTH 4 [current_design]

create_clock -period 10.0 -name SYSCLK [get_ports SYSCLK]
create_clock -period 6.0 -name HDMICLK [get_ports HDMI_CLK_P]

# videoClk
create_clock -period 18.0 -name videoClk [get_nets spiClockArea_slabControl_io_videoClk]
