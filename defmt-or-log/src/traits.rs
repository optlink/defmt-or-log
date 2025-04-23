/// A trait that requires both `core::fmt::Debug` and `defmt::Format`.
pub trait FormatAndDebug: core::fmt::Debug + defmt::Format {}

impl<T> FormatAndDebug for T where T: core::fmt::Debug + defmt::Format {}

/// Copy of defmt::Debug2Format, except it behaves transparently if defmt is disabled
pub struct Debug2Format<'a, T: core::fmt::Debug + ?Sized>(pub &'a T);

impl<T: core::fmt::Debug + ?Sized> core::fmt::Debug for Debug2Format<'_, T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.0.fmt(f)
    }
}

impl<T: core::fmt::Debug + ?Sized> defmt::Format for Debug2Format<'_, T> {
    fn format(&self, f: defmt::Formatter<'_>) {
        defmt::Debug2Format(self.0).format(f)
    }
}

/// Copy of defmt::Display2Format, except it behaves transparently if defmt is disabled
pub struct Display2Format<'a, T: core::fmt::Display + ?Sized>(pub &'a T);

impl<T: core::fmt::Display + ?Sized> core::fmt::Display for Display2Format<'_, T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.0.fmt(f)
    }
}

impl<T: core::fmt::Display + ?Sized> defmt::Format for Display2Format<'_, T> {
    fn format(&self, f: defmt::Formatter<'_>) {
        defmt::Display2Format(self.0).format(f)
    }
}
