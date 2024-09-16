#[cfg(test)]
mod tests {
    use embedded_hal_mock::eh1::i2c::{Mock, Transaction};
    use husb238::*;

    #[test]
    fn test_get_src_pdo() {
        let expectations = [Transaction::write_read(
            HUSB238_ADDR,
            vec![Register::SrcPdo as u8],
            vec![0x20],
        )];
        let mut i2c = Mock::new(&expectations);
        let mut husb238 = Husb238::new(i2c.clone());

        let result = husb238.get_src_pdo().unwrap();
        assert_eq!(result, SrcPdo::_9v);

        i2c.done();
    }

    #[test]
    fn test_set_src_pdo() {
        let expectations = [Transaction::write(
            HUSB238_ADDR,
            vec![Register::SrcPdo as u8, SrcPdo::_12v as u8],
        )];
        let mut i2c = Mock::new(&expectations);
        let mut husb238 = Husb238::new(i2c.clone());

        husb238.set_src_pdo(SrcPdo::_12v).unwrap();

        i2c.done();
    }

    #[test]
    fn test_go_command() {
        let expectations = [Transaction::write(
            HUSB238_ADDR,
            vec![Register::GoCommand as u8, Command::Request as u8],
        )];
        let mut i2c = Mock::new(&expectations);
        let mut husb238 = Husb238::new(i2c.clone());

        husb238.go_command(Command::Request).unwrap();

        i2c.done();
    }

    #[test]
    fn test_get_pd_status0() {
        let expectations = [Transaction::write_read(
            HUSB238_ADDR,
            vec![Register::PdStatus0 as u8],
            vec![0x58],
        )];
        let mut i2c = Mock::new(&expectations);
        let mut husb238 = Husb238::new(i2c.clone());

        let result = husb238.get_pd_status0().unwrap();
        assert_eq!(result, (Voltage::_18v, Current::_2_5a));

        i2c.done();
    }

    #[test]
    fn test_get_actual_voltage_and_current() {
        let expectations = [Transaction::write_read(
            HUSB238_ADDR,
            vec![Register::PdStatus0 as u8],
            vec![0x58],
        )];
        let mut i2c = Mock::new(&expectations);
        let mut husb238 = Husb238::new(i2c.clone());

        let result = husb238.get_actual_voltage_and_current().unwrap();
        assert_eq!(result, (Some(18.0), 2.5));

        i2c.done();
    }

    #[test]
    fn test_src_pdo_to_string() {
        let result: &str = SrcPdo::_9v.into();
        assert_eq!(result, "9V");
    }

    #[test]
    fn test_src_pdo_from_u8() {
        let result: SrcPdo = 0x20.into();
        assert_eq!(result, SrcPdo::_9v);
    }

    #[test]
    fn test_current_from_u8() {
        let result: Current = 0x06.into();
        assert_eq!(result, Current::_2_0a);
    }

    #[test]
    fn test_current_to_string() {
        let result: &str = Current::_2_0a.into();
        assert_eq!(result, "2.0A");
    }

    #[test]
    fn test_voltage_from_u8() {
        let result: Voltage = 0x50.into();
        assert_eq!(result, Voltage::_18v);
    }

    #[test]
    fn test_voltage_to_string() {
        let result: &str = Voltage::_18v.into();
        assert_eq!(result, "18V");
    }
}
