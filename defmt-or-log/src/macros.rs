// Copied from esp-hal: https://github.com/esp-rs/esp-hal/blob/main/esp-hal-common/src/fmt.rs

#[macro_export]
macro_rules! if_defmt {
    ($defmt:expr, $log:expr) => {
        if $crate::DEFMT {
            $defmt
        } else {
            $log
        }
    };
}

#[macro_export]
macro_rules! assert {
    ($($x:tt)*) => {
        $crate::if_defmt!($crate::defmt::assert!($($x)*), ::core::assert!($($x)*))
    };
}

#[macro_export]
macro_rules! assert_eq {
    ($($x:tt)*) => {
        $crate::if_defmt!($crate::defmt::assert_eq!($($x)*), ::core::assert_eq!($($x)*))
    };
}

#[macro_export]
macro_rules! assert_ne {
    ($($x:tt)*) => {
        $crate::if_defmt!($crate::defmt::assert_ne!($($x)*), ::core::assert_ne!($($x)*))
    };
}

#[macro_export]
macro_rules! debug_assert {
    ($($x:tt)*) => {
        $crate::if_defmt!($crate::defmt::debug_assert!($($x)*), ::core::debug_assert!($($x)*))
    };
}

#[macro_export]
macro_rules! debug_assert_eq {
    ($($x:tt)*) => {
        $crate::if_defmt!($crate::defmt::debug_assert_eq!($($x)*), ::core::debug_assert_eq!($($x)*))
    };
}

#[macro_export]
macro_rules! debug_assert_ne {
    ($($x:tt)*) => {
        $crate::if_defmt!($crate::defmt::debug_assert_ne!($($x)*), ::core::debug_assert_ne!($($x)*))
    };
}

#[macro_export]
macro_rules! todo {
    ($($x:tt)*) => {
        $crate::if_defmt!($crate::defmt::todo!($($x)*), ::core::todo!($($x)*))
    };
}

#[macro_export]
macro_rules! unreachable {
    ($($x:tt)*) => {
        $crate::if_defmt!($crate::defmt::unreachable!($($x)*), ::core::unreachable!($($x)*))
    };
}

#[macro_export]
macro_rules! panic {
    ($($x:tt)*) => {
        $crate::if_defmt!($crate::defmt::panic!($($x)*), ::core::panic!($($x)*))
    };
}

#[macro_export]
macro_rules! trace {
    ($($x:tt)*) => {
        $crate::if_defmt!($crate::defmt::trace!($($x)*), $crate::log::trace!($($x)*))
    };
}

#[macro_export]
macro_rules! debug {
    ($($x:tt)*) => {
        $crate::if_defmt!($crate::defmt::debug!($($x)*), $crate::log::debug!($($x)*))
    };
}

#[macro_export]
macro_rules! info {
    ($($x:tt)*) => {
        $crate::if_defmt!($crate::defmt::info!($($x)*), $crate::log::info!($($x)*))
    };
}

#[macro_export]
macro_rules! warn {
    ($($x:tt)*) => {
        $crate::if_defmt!($crate::defmt::warn!($($x)*), $crate::log::warn!($($x)*))
    };
}

#[macro_export]
macro_rules! error {
    ($($x:tt)*) => {
        $crate::if_defmt!($crate::defmt::error!($($x)*), $crate::log::error!($($x)*))
    };
}

#[macro_export]
macro_rules! intern {
    ($s:literal) => {
        $crate::if_defmt!($crate::defmt::intern!($s), $s)
    };
}

#[macro_export]
macro_rules! unwrap {
    ($x:expr) => {
        $crate::if_defmt!($crate::defmt::unwrap!($x), $x.unwrap())
    };
}

#[macro_export]
macro_rules! expect {
    ($x:expr, $msg:expr) => {
        $crate::if_defmt!($crate::defmt::expect!($x, $msg), $x.expect($msg))
    };
}

#[macro_export]
macro_rules! unimplemented {
    ($($x:tt)*) => {
        $crate::if_defmt!($crate::defmt::unimplemented!($($x)*), ::core::unimplemented!($($x)*))
    };
}
