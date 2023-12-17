use crate::smart_house::Error;

/// Розетка
#[allow(unused)]
pub struct Outlet {}

impl Outlet {
    /// описание розетки
    pub fn _description(&self) -> String {
        todo!();
    }

    /// включить розетку
    pub fn _on(&self) -> Result<(), Error> {
        todo!();
    }

    /// выключить розетку
    pub fn _off(&self) -> Result<(), Error> {
        todo!();
    }

    /// текущее потребление энергии в мВт
    pub fn _power_consumption(&self) -> Result<u32, Error> {
        todo!();
    }
}
