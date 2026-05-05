use {
    smash::{
        lua2cpp::*,
        hash40,
        phx::*,
        app::{sv_animcmd::*, lua_bind::*, *},
        lib::{lua_const::*}
    },
    smash_script::*,
    smashline::*
};

mod tilts;
mod smashes;
mod aerials;
mod attackdash;
mod attackjab;

pub static mut FIGHTER_MURABITO_GENERATE_ARTICLE_SPINAXE: i32 = 21;

pub fn install(agent: &mut smashline::Agent) {
    tilts::install(agent);
    smashes::install(agent);
    aerials::install(agent);
    attackdash::install(agent);
    attackjab::install(agent);
    downspecial::install(agent);

    unsafe {
        FIGHTER_MURABITO_GENERATE_ARTICLE_SPINAXE += smashline::clone_weapon("krool", *WEAPON_KIND_KROOL_IRONBALL, "murabito", "spinaxe", false);
    }
}