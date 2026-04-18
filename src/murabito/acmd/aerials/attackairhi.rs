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

/*
if macros::is_excute(fighter) {
    macros::ATTACK(fighter, /* dont edit */
        0, /* id */
        0, /* part (dont edit)*/
        Hash40::new("arml"), /* bone */
        67.0 /* Damage, changed from 12.0 */, 
        361, /* angle */
        80,  0, 30, /* knockback growth, fixed knockback, base knockback */
        3.0 /* Size, changed from 3.0 */, 
        3.2, 0.0, 0.0, /* position */
        Some(50.0), Some(0.0), Some(0.0), /* position 2 */
        1.0, /* hitlag */
        1.0, /* sdi */
        *ATTACK_SETOFF_KIND_ON, /* clang rebound (ignore) */
        *ATTACK_LR_CHECK_F, /* facing restriction (ignore) */
        false, /* set weight */
        0, /* shield damage (max 2)*/
        0.0, /* trip chance */
        0, /* rehit (for multihits, how long til a hitbox can hit again)*/
        false, /* reflectable */
        false, /* absorbable */
        false, /* flinchless */
        false, /* disable hitlag */
        true, /* direct hitbox (true if from the character, false if on weapon)*/
        *COLLISION_SITUATION_MASK_GA, /* ground or air */
        *COLLISION_CATEGORY_MASK_ALL, /* hitbits (ignore) */
        *COLLISION_PART_MASK_ALL, /* collision part (ignore) */
        false, /* friendly fire */
        Hash40::new("collision_attr_normal"), /* effect */
        *ATTACK_SOUND_LEVEL_M, /* sfx volume/level (s, m, or l)*/
        *COLLISION_SOUND_ATTR_PUNCH, /* sfx type */
        *ATTACK_REGION_PUNCH
    ); /* type (mostly for spirits only)*/
}
*/

unsafe extern "C" fn game_attackairhi(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(agent.lua_state_agent, 7.0);
    macros::ATTACK(agent, 0, 0, Hash40::new("stickr"), 1.3, 110, 100, 100, 10, 7.0, 0.0, 5.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
    macros::ATTACK(agent, 1, 0, Hash40::new("stickr"), 1.3, 110, 100, 100, 10, 5.0, 0.0, 1.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);

    for _ in 0..9 {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("stickr"), 1.7, 90, 0, 40, 10, 7.0, 0.0, 5.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
            macros::ATTACK(agent, 1, 0, Hash40::new("stickr"), 1.7, 90, 0, 40, 10, 5.0, 0.0, 1.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
        }
        wait(agent.lua_state_agent, 2.0);
        if macros::is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
        }
    }
    frame(agent.lua_state_agent, 28.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        macros::ATTACK(agent, 0, 0, Hash40::new("stickr"), 3.0, 90, 50, 0, 120, 7.0, 0.0, 6.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
        macros::ATTACK(agent, 1, 0, Hash40::new("stickr"), 3.0, 90, 50, 0, 120, 5.0, 0.0, 1.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
    }
    frame(agent.lua_state_agent, 31.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

unsafe extern "C" fn effect_attackairhi(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_murabito_axe1"), Hash40::new("tex_murabito_axe2"), 6, Hash40::new("haver"), 0, 1.5, 3, Hash40::new("haver"), 0, 9.8, 3, true, Hash40::new("null"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_BLEND_SRC_ONE, 101, *TRAIL_CULL_NONE, 1.2, 0.2);
    }
    frame(agent.lua_state_agent, 33.0);
    if macros::is_excute(agent) {
        macros::AFTER_IMAGE_OFF(agent, 4);
    }
}

unsafe extern "C" fn sound_attackairhi(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_sword_swing_m"));
    }
}
/* 
unsafe extern "C" fn expression_attackairhi(agent: &mut L2CAgentBase) {
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


pub fn install(agent: &mut smashline::Agent) {
    agent.acmd("game_attackairhi", game_attackairhi, Priority::Default);
    agent.acmd("effect_attackairhi", effect_attackairhi, Priority::Default);
    agent.acmd("sound_attackairhi", sound_attackairhi, Priority::Default);
    //agent.acmd("expression_attackairhi", expression_attackairhi, Priority::Default);
}