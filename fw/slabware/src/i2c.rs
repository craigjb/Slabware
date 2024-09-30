fn i2c_start_blocking(i2c: &I2c0) {
    i2c.master_status().modify(|_, w| w.start().set_bit());
    while i2c.master_status().read().start().bit_is_set() {}
}

fn i2c_stop_blocking(i2c: &I2c0) {
    i2c.master_status().modify(|_, w| w.stop().set_bit());
    while i2c.master_status().read().busy().bit_is_set() {}
}

fn i2c_tx(i2c: &I2c0, data: u8) {
    i2c.tx_data().write(|w| unsafe {
        w.valid()
            .set_bit()
            .enable()
            .set_bit()
            .disable_on_data_conflict()
            .clear_bit()
            .value()
            .bits(data)
            .repeat()
            .clear_bit()
    });
}

fn i2c_tx_nack_blocking(i2c: &I2c0) {
    i2c.tx_ack().write(|w| {
        w.value()
            .set_bit()
            .valid()
            .set_bit()
            .enable()
            .set_bit()
            .repeat()
            .clear_bit()
            .disable_on_data_conflict()
            .clear_bit()
    });
    while i2c.tx_ack().read().valid().bit_is_set() {}
}

fn i2c_tx_ack_blocking(i2c: &I2c0) {
    i2c.tx_ack().write(|w| {
        w.value()
            .clear_bit()
            .valid()
            .set_bit()
            .enable()
            .set_bit()
            .repeat()
            .clear_bit()
            .disable_on_data_conflict()
            .clear_bit()
    });
    while i2c.tx_ack().read().valid().bit_is_set() {}
}

fn i2c_rx(i2c: &I2c0) -> u8 {
    i2c.rx_data().read().value().bits()
}
