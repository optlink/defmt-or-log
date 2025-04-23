use defmt::Format;
use defmt_or_log::*; // Import the log macros and the FormatAndDebug trait

#[derive(Debug, Format)]
struct S {
    a: u32,
    b: bool,
}

// A function that takes something which can be formatted
fn do_something<T: FormatAndDebug>(t: T) {
    error!("T is {:?}", t); // logs with defmt or log depending on the current configuration
}

fn main() {
    // <Here you would set up your global_logger for log or defmt>

    // The entire code in this examples compiles, no matter which backend is used

    let s = S { a: 1, b: true };
    do_something(s); // log some struct

    let e = "not_an_int".parse::<i32>().unwrap_err(); //Example for a type for which core::fmt::Debug is implemented, but defmt::Format is not
    do_something(Debug2Format(&e)); // use the Debug2Format adapter, which behaves transparently if defmt is disabled, and uses defmt::Debug2Format if defmt is enabled

    do_something(42); // log a primitive (which is always possible, because core::fmt::Debug and defmt::Format are implemented for all primitives)
}
