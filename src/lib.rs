#![feature(proc_macro_hygiene)]

//use skyline::{hook, install_hook};
pub static mut MARKED_COLORS: [bool; 256] = [false; 256];

mod murabito;

#[skyline::main(name = "skyline_rs_template")]
pub fn main() {
    murabito::install();

    unsafe {
        extern "C" {
            fn arcrop_register_event_callback(
                ty: arcropolis_api::Event,
                callback: arcropolis_api::EventCallbackFn,
            );
        }
        arcrop_register_event_callback(arcropolis_api::Event::ModFilesystemMounted, mods_mounted);
    }
}

extern "C" fn mods_mounted(_ev: arcropolis_api::Event) {
    const MARKER_FILE: &str = "murabito.marker";
    let mut lowest_color: i32 = -1;
    let mut marked_slots: Vec<i32> = vec![];
    for x in 0..256 {
        if let Ok(_) = std::fs::read(format!(
            "mods:/fighter/murabito/model/body/c{:02}/{}",
            x, MARKER_FILE
        )) {
            unsafe {
                marked_slots.push(x as _);
                MARKED_COLORS[x as usize] = true;
                if lowest_color == -1 {
                    lowest_color = x as _ ;
                }
            }
        }
    }


    if lowest_color == -1 {
        // if no marker exist, leave
        return;
    }
}