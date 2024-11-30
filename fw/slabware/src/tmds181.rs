use defmt::Format;
use embedded_hal_async::i2c::I2c;

#[derive(Clone, Copy, Format)]
pub enum Tmds181Error {
    I2cError,
    InvalidDeviceId,
}

pub struct Tmds181Config {
    address: u8,
    lane_swap: bool,
    polarity_swap: bool,
    auto_standby: bool,
}

impl Tmds181Config {
    pub fn new() -> Self {
        Self {
            address: 0x5E,
            lane_swap: false,
            polarity_swap: false,
            auto_standby: false,
        }
    }

    pub fn address(mut self, address: u8) -> Self {
        self.address = address;
        self
    }

    pub fn lane_swap(mut self, en: bool) -> Self {
        self.lane_swap = en;
        self
    }

    pub fn polarity_swap(mut self, en: bool) -> Self {
        self.polarity_swap = en;
        self
    }

    pub fn redriver_auto_standby(mut self, en: bool) -> Self {
        self.auto_standby = en;
        self
    }

    fn control_reg0_value(&self) -> u8 {
        let mut value = 0b00000010;
        if self.lane_swap {
            value |= 1 << 7;
        }
        if self.polarity_swap {
            value |= 1 << 6;
        }
        if self.auto_standby {
            value |= 1 << 4;
        }
        value
    }

    pub async fn build<I2C: I2c>(self, i2c: I2C) -> Result<Tmds181<I2C>, Tmds181Error> {
        Tmds181::init(i2c, self).await
    }
}

const ID_AND_VERSION_ADDR: u8 = 0x0;
const CONTROL_REG_ADDR: u8 = 0x9;
const ID_AND_VERSION: [u8; 9] = [0x54, 0x4D, 0x44, 0x53, 0x31, 0x38, 0x31, 0x20, 0x1];

pub struct Tmds181<I2C: I2c> {
    _i2c: I2C,
}

impl<I2C: I2c> Tmds181<I2C> {
    async fn init(mut i2c: I2C, config: Tmds181Config) -> Result<Self, Tmds181Error> {
        let mut data: [u8; 9] = [0; 9];

        // verify device ID and version
        i2c.write_read(config.address, &[ID_AND_VERSION_ADDR], &mut data)
            .await
            .map_err(|_| Tmds181Error::I2cError)?;
        defmt::debug!("TMDS181 device ID: {:#02X}", data);
        if data != ID_AND_VERSION {
            return Err(Tmds181Error::InvalidDeviceId);
        }

        // write control regs with config
        i2c.write(
            config.address,
            &[CONTROL_REG_ADDR, config.control_reg0_value()],
        )
        .await
        .map_err(|_| Tmds181Error::I2cError)?;

        // read back config for debugging
        i2c.write_read(config.address, &[CONTROL_REG_ADDR], &mut data[0..4])
            .await
            .map_err(|_| Tmds181Error::I2cError)?;
        defmt::debug!("TMDS181 control regs: {:#08b}", data[0..4]);

        Ok(Self { _i2c: i2c })
    }
}
