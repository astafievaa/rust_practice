#[allow(unused)]
/// Ошибки устройств умного дома
pub enum Error {
    /// Общая ошибка 1
    Error1,
    /// Общая ошибка 2
    Error2,
    /// Ошибка розетки
    OutletError(String),
    // Ошибка термометра
    ThermometerError(String),
}
