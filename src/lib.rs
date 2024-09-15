#![no_std]

#[cfg(not(feature = "async"))]
use embedded_hal::i2c::I2c;
use embedded_hal::i2c::SevenBitAddress;
#[cfg(feature = "async")]
use embedded_hal_async::i2c::I2c;

pub const HUSB238_ADDR: SevenBitAddress = 0x08;

#[repr(u8)]
pub enum Register {
    PdStatus0 = 0x00,
    PdStatus1 = 0x01,
    SrcPdo5V = 0x02,
    SrcPdo9V = 0x03,
    SrcPdo12V = 0x04,
    SrcPdo15V = 0x05,
    SrcPdo18V = 0x06,
    SrcPdo20V = 0x07,
    SrcPdo = 0x08,
    GoCommand = 0x09,
}

#[repr(u8)]
#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Current {
    _0_5a = 0x00,
    _0_7a = 0x01,
    _1_0a = 0x02,
    _1_25a = 0x03,
    _1_5a = 0x04,
    _1_75a = 0x05,
    _2_0a = 0x06,
    _2_25a = 0x07,
    _2_5a = 0x08,
    _2_75a = 0x09,
    _3_0a = 0x0A,
    _3_25a = 0x0B,
    _3_5a = 0x0C,
    _4_0a = 0x0D,
    _4_5a = 0x0E,
    _5_0a = 0x0F,
}

impl From<u8> for Current {
    fn from(value: u8) -> Self {
        match value {
            0x00 => Current::_0_5a,
            0x01 => Current::_0_7a,
            0x02 => Current::_1_0a,
            0x03 => Current::_1_25a,
            0x04 => Current::_1_5a,
            0x05 => Current::_1_75a,
            0x06 => Current::_2_0a,
            0x07 => Current::_2_25a,
            0x08 => Current::_2_5a,
            0x09 => Current::_2_75a,
            0x0A => Current::_3_0a,
            0x0B => Current::_3_25a,
            0x0C => Current::_3_5a,
            0x0D => Current::_4_0a,
            0x0E => Current::_4_5a,
            0x0F => Current::_5_0a,
            _ => unreachable!(),
        }
    }
}

impl<'a> Into<&'a str> for Current {
    fn into(self) -> &'a str {
        match self {
            Current::_0_5a => "0.5A",
            Current::_0_7a => "0.7A",
            Current::_1_0a => "1.0A",
            Current::_1_25a => "1.25A",
            Current::_1_5a => "1.5A",
            Current::_1_75a => "1.75A",
            Current::_2_0a => "2.0A",
            Current::_2_25a => "2.25A",
            Current::_2_5a => "2.5A",
            Current::_2_75a => "2.75A",
            Current::_3_0a => "3.0A",
            Current::_3_25a => "3.25A",
            Current::_3_5a => "3.5A",
            Current::_4_0a => "4.0A",
            Current::_4_5a => "4.5A",
            Current::_5_0a => "5.0A",
        }
    }
}

#[repr(u8)]
#[derive(PartialEq, Clone, Copy, Debug)]
pub enum SrcPdo {
    Unattached = 0x00,
    _5v = 0x10,
    _9v = 0x20,
    _12v = 0x30,
    _15v = 0x40,
    _18v = 0x50,
    _20v = 0x60,
    Reserved = 0x70,
}

impl From<u8> for SrcPdo {
    fn from(value: u8) -> Self {
        match value {
            0x00 => SrcPdo::Unattached,
            0x10 => SrcPdo::_5v,
            0x20 => SrcPdo::_9v,
            0x30 => SrcPdo::_12v,
            0x40 => SrcPdo::_15v,
            0x50 => SrcPdo::_18v,
            0x60 => SrcPdo::_20v,
            _ => SrcPdo::Reserved,
        }
    }
}

impl<'a> Into<&'a str> for SrcPdo {
    fn into(self) -> &'a str {
        match self {
            SrcPdo::Unattached => "Unattached",
            SrcPdo::_5v => "5V",
            SrcPdo::_9v => "9V",
            SrcPdo::_12v => "12V",
            SrcPdo::_15v => "15V",
            SrcPdo::_18v => "18V",
            SrcPdo::_20v => "20V",
            SrcPdo::Reserved => "Reserved",
        }
    }
}

pub const SRC_PDO_MASK: u8 = 0xF0;

#[repr(u8)]
pub enum Command {
    Request = 0x01,
    GetSrcCap = 0x04,
    HardReset = 0x10,
}

pub struct Husb238<I2C> {
    i2c: I2C,
}

#[maybe_async_cfg::maybe(
    sync(cfg(not(feature = "async")), self = "Husb238",),
    async(feature = "async", keep_self)
)]
impl<I2C, E> Husb238<I2C>
where
    I2C: I2c<Error = E>,
{
    pub fn new(i2c: I2C) -> Self {
        Self { i2c }
    }

    pub async fn get_src_pdo(&mut self) -> Result<SrcPdo, E> {
        let mut buf = [0u8; 1];
        self.i2c
            .write_read(HUSB238_ADDR, &[Register::SrcPdo as u8], &mut buf)
            .await?;
        Ok((buf[0] & SRC_PDO_MASK).into())
    }

    pub async fn set_src_pdo(&mut self, src_pdo: SrcPdo) -> Result<(), E> {
        self.i2c
            .write(HUSB238_ADDR, &[Register::SrcPdo as u8, src_pdo as u8])
            .await
    }

    pub async fn go_command(&mut self, command: Command) -> Result<(), E> {
        self.i2c
            .write(HUSB238_ADDR, &[Register::GoCommand as u8, command as u8])
            .await
    }
}
