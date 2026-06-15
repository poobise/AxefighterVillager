#![feature(proc_macro_hygiene)]
use {
    arcropolis_api::*,
     std::{
        fs::*
    }
};
//use skyline::{hook, install_hook};
pub static mut MARKED_COLORS: [bool; 256] = [false; 256];

mod murabito;
//mod lucina;

#[skyline::main(name = "skyline_rs_template")]
pub fn main() {
    //lucina::install();
    
    unsafe {
        extern "C" {
            fn arcrop_register_event_callback(ty: Event, callback: EventCallbackFn);
        }
        arcrop_register_event_callback(Event::ModFilesystemMounted, mods_mounted);
    }
}

extern "C" fn mods_mounted(_ev: Event) {
    const MARKER_FILE: &str = "AxeFighterVillager.marker";
    let mut lowest_color: i32 = -1;
    let mut marked_slots: Vec<i32> = vec![];
    for x in 0..256 {
        if let Ok(_) = read(format!(
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
    let color_num = {
        unsafe {
            let mut index = lowest_color;
            while index < 256 && MARKED_COLORS[index as usize] {
                index += 1;
            }
            index - lowest_color
        }
    };

    println!("LOWEST: {} - COLOR NUM: {}", lowest_color, color_num);
    
    murabito::install();
}