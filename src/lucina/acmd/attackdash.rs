use {
    smash::{
        lua2cpp::*,
        phx::*,
        app::{sv_animcmd::*, lua_bind::*},
        lib::lua_const::*
    },
    smashline::*,
    smash_script::*
};
use super::*;

unsafe extern "C" fn game_attackdash(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) { 
        ArticleModule::generate_article(agent.module_accessor, FIGHTER_LUCINA_GENERATE_ARTICLE_POO, false, -1);
        ArticleModule::shoot_exist(agent.module_accessor, 
            FIGHTER_LUCINA_GENERATE_ARTICLE_POO, 
            ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), 
            false
        );
    }

    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}
/* 

unsafe extern "C" fn effect_attackdash(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_murabito_axe1"), Hash40::new("tex_murabito_axe2"), 6, Hash40::new("haver"), 0, 1.5, 3, Hash40::new("haver"), 0, 9.8, 3, true, Hash40::new("null"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_BLEND_SRC_ONE, 101, *TRAIL_CULL_NONE, 1.2, 0.2);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        macros::AFTER_IMAGE_OFF(agent, 4);
    }
}

unsafe extern "C" fn sound_attackdash(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_sword_swing_m"));
    }
}

unsafe extern "C" fn expression_attackdash(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        AttackModule::set_attack_reference_joint_id(agent.module_accessor, Hash40::new("haver"), smash::app::AttackDirectionAxis(*ATTACK_DIRECTION_Z), smash::app::AttackDirectionAxis(*ATTACK_DIRECTION_Y), smash::app::AttackDirectionAxis(*ATTACK_DIRECTION_X));
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_slashm"), 0);
    }
}

*/

unsafe extern "C" fn poo(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 100.0, 361, 0, 0, 0, 7.0, 0.0, 0.0, 0.0, None, None, None, 0.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_none"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_MAGIC);
    }
}

pub fn install(agent: &mut smashline::Agent) {
    agent.acmd("game_attackdash", game_attackdash, Priority::Default);
    /* 
    agent.acmd("effect_attackdash", effect_attackdash, Priority::Default);
    agent.acmd("sound_attackdash", sound_attackdash, Priority::Default);
    agent.acmd("expression_attackdash", expression_attackdash, Priority::Default);
    */
    /* 
    Agent::new("lucina_poo")
    .game_acmd("game_regular", poo, Default)
    */
}