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


mod attackdash;

pub static mut FIGHTER_LUCINA_GENERATE_ARTICLE_POO: i32 = 1;

pub fn install(agent: &mut smashline::Agent) {
    attackdash::install(agent);

    unsafe {
        FIGHTER_LUCINA_GENERATE_ARTICLE_POO += smashline::clone_weapon("krool", *WEAPON_KIND_KROOL_IRONBALL, "lucina", "poo", false);
    }
}