use {
    smash::{
        lua2cpp::*,
        phx::*,
        app::{sv_animcmd::*, lua_bind::*,AttackDirectionAxis},
        lib::lua_const::*,
        hash40
    },
    smashline::*,
    smash_script::*
};
use super::super::*;

unsafe extern "C" fn game_attackairlw(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        
        macros::ATTACK(agent, 0, 0, Hash40::new("haver"), 11.0, 320, 
        100, 0, 36, 
        5.0, 0.0, 6.5, 0.0, None, None, None, 
        1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, 
        false, 0, 0.0, 0, 
        false, false, false, 
        false, true, *COLLISION_SITUATION_MASK_GA, 
        *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, 
        false, Hash40::new("collision_attr_cutup"), 
        *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
        
        macros::ATTACK(agent, 1, 0, Hash40::new("haver"), 11.0, 320, 100, 0, 36, 3.0, 0.0, 1.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
        
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("haver"), 11.0, 320, 110, 0, 40, 5.0, 0.0, 6.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
        macros::ATTACK(agent, 1, 0, Hash40::new("haver"), 11.0, 320, 110, 0, 40, 3.0, 0.0, 1.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
    }
    frame(agent.lua_state_agent, 18.5);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

unsafe extern "C" fn effect_speciallw3(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.5);
    if macros::is_excute(agent) {
        //macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_murabito_axe1"), Hash40::new("tex_murabito_axe2"), 6, Hash40::new("haver"), 0, 1.5, 3, Hash40::new("haver"), 0, 9.8, 3, true, Hash40::new("null"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_BLEND_SRC_ONE, 1, *TRAIL_CULL_NONE, 1.2, 0.2);
        
        //effect!(agent,*MA_MSC_CMD_EFFECT_AFTER_IMAGE3_ON, Hash40::new("tex_item_killsword1"), Hash40::new("tex_item_killsword2"), 6, Hash40::new("haver"), -1, 1.5, 4, Hash40::new("haver"), 0, 9, 4, true, Hash40::new("null"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_BLEND_SRC_ONE, 1);
        effect!(agent,*MA_MSC_CMD_EFFECT_AFTER_IMAGE3_ON, Hash40::new("tex_item_killsword1"), Hash40::new("tex_item_killsword2"), 6, Hash40::new("stickr"), 0, 1.5, 3, Hash40::new("stickr"), 0, 9, 3, true, Hash40::new("null"), Hash40::new("stickr"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_BLEND_SRC_ONE, 1);

    }
    frame(agent.lua_state_agent, 19.0);
    if macros::is_excute(agent) {
        macros::AFTER_IMAGE_OFF(agent, 4);
    }
}
/* 
unsafe extern "C" fn sound_attackairlw(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_murabito_attackair_l01"));
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_sword_swing_m"));
    }
}
*/
unsafe extern "C" fn expression_attackairlw(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        AttackModule::set_attack_reference_joint_id(agent.module_accessor, Hash40::new("haver"), AttackDirectionAxis(*ATTACK_DIRECTION_Z), AttackDirectionAxis(*ATTACK_DIRECTION_Y), AttackDirectionAxis(*ATTACK_DIRECTION_X));
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
        VisibilityModule::set_int64(agent.module_accessor, hash40("item") as i64, hash40("item_axe") as i64);
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

pub fn install() {
    Agent::new("murabito")
    .set_costume(get_costumes())
    .acmd("game_attackairlw", game_attackairlw, Priority::Default)
    .acmd("effect_attackairlw", effect_speciallw3, Priority::Default)
    //.acmd("sound_attackairlw", sound_attackairlw, Priority::Default)
    .acmd("expression_attackairlw", expression_attackairlw, Priority::Default)
    .install();
}