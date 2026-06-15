use {
    smash::{
        //app::{sv_animcmd::*, lua_bind::*, *},
        lib::{lua_const::*}
    },
    crate::MARKED_COLORS
};

mod tilts;
mod smashes;
mod aerials;
mod attackdash;
mod attackjab;
mod downspecial;

pub static mut FIGHTER_MURABITO_GENERATE_ARTICLE_SPINAXE: i32 = 21;

pub fn install() {
    tilts::install();
    smashes::install();
    aerials::install();
    attackdash::install();
    attackjab::install();
    downspecial::install();

    unsafe {
        FIGHTER_MURABITO_GENERATE_ARTICLE_SPINAXE += smashline::clone_weapon("krool", *WEAPON_KIND_KROOL_IRONBALL, "murabito", "spinaxe", false);
    }
}

pub fn get_costumes() -> Vec<usize> {
	let costumes = &mut Vec::new();
    unsafe {
        let marked = *&raw mut MARKED_COLORS;
        for i in 0..marked.len() {
            if marked[i] {
                costumes.push(i);
            }
        }
    }
    costumes.to_vec()
}