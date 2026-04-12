#![feature(proc_macro_hygiene)]

//use skyline::{hook, install_hook};

mod murabito;

#[skyline::main(name = "skyline_rs_template")]
pub fn main() {
    murabito::install();
}
