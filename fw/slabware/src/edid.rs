use crate::i2c::AckKind;
use crate::si2c::I2cSlave;

const EDID_DATA: &[u8] = include_bytes!("SoundSlab.edid");

#[embassy_executor::task]
pub async fn ddc_edid(i2cs: I2cSlave) {
    let mut offset: u8 = 0;
    loop {
        i2cs.wait_for_start().await;
        // address filter will change to Ack
        if let Ok(addr) = i2cs.read(AckKind::Nack).await {
            match addr {
                0xA0 => {
                    if let Ok(new_offset) = i2cs.read(AckKind::Ack).await {
                        offset = new_offset;
                        defmt::debug!("EDID offset set: {:#02X}", offset);
                    } else {
                        break;
                    }
                }
                0xA1 => {
                    let mut count: usize = 0;
                    loop {
                        let ack = i2cs.write(EDID_DATA[offset as usize]).await;
                        offset = offset.wrapping_add(1);
                        count += 1;
                        if ack.is_nack() {
                            defmt::debug!("EDID read: {} byte(s)", count);
                            break;
                        }
                    }
                }
                _ => {
                    defmt::println!("EDID ignored address: {}", addr);
                }
            }
        }
    }
}
