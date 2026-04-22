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


unsafe extern "C" fn game_attackairlw(agent: &mut L2CAgentBase) {
    print("FUCKING");
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        
        macros::ATTACK(agent, 0, 0, Hash40::new("haver"), 11.0, 320, 100, 0, 36, 4.0, 0.0, 6.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
        macros::ATTACK(agent, 1, 0, Hash40::new("haver"), 11.0, 320, 100, 0, 36, 2.5, 0.0, 1.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 2, 0, Hash40::new("haver"), 11.0, 240, 110, 0, 40, 4.0, 0.0, 6.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
        macros::ATTACK(agent, 3, 0, Hash40::new("haver"), 11.0, 240, 110, 0, 40, 2.5, 0.0, 1.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

unsafe extern "C" fn effect_attackairlw(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        //macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_murabito_axe1"), Hash40::new("tex_murabito_axe2"), 6, Hash40::new("haver"), 0, 1.5, 3, Hash40::new("haver"), 0, 9.8, 3, true, Hash40::new("null"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_BLEND_SRC_ONE, 101, *TRAIL_CULL_NONE, 1.2, 0.2);
        //macros::QUAKE(agent, *SHAKE_KIND_MEDIUM);

        
        macros::AFTER_IMAGE4_ON_arg29(
            agent, 
            Hash40::new("tex_common_white"), // Using white for testing; swap to killsword once visible
            Hash40::new("tex_common_white"), 
            8,
            Hash40::new("haver"),
            0.0, 2.0, 0.0,  // Base: 2 units up the X-axis
            Hash40::new("haver"),
            0.0, 11.0, 0.0, // Tip: 11 units up the X-axis (aligned with your hitbox 11.0)
            true, 
            Hash40::new("null"),
            Hash40::new("haver"),
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, 
            *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ADD, 101, *TRAIL_CULL_NONE, 1.4, 0.1
        );
    }
    frame(agent.lua_state_agent, 19.0);
    if macros::is_excute(agent) {
        macros::AFTER_IMAGE_OFF(agent, 8);
    }
}

unsafe extern "C" fn sound_attackairlw(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_homerunbat_hit"));
        //macros::PLAY_SE(agent, Hash40::new("se_common_sword_swing_m"));
    }
}

unsafe extern "C" fn expression_attackairlw(agent: &mut L2CAgentBase) {
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



pub fn install(agent: &mut smashline::Agent) {
    agent.acmd("game_attackairlw", game_attackairlw, Priority::Default);
    agent.acmd("effect_attackairlw", effect_attackairlw, Priority::Default);
    agent.acmd("sound_attackairlw", sound_attackairlw, Priority::Default);
    agent.acmd("expression_attackairlw", expression_attackairlw, Priority::Default);
}