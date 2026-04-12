#![feature(
    concat_idents,
    proc_macro_hygiene
)]
#![allow(
    unused_macros
)]

use skyline::{hook, install_hook};

mod murabito;

#[skyline::main(name = "skyline_rs_template")]
pub fn main() {
    murabito::install();
}
