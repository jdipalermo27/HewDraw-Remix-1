
use super::*;

#[acmd_script( agent = "packun", scripts = ["game_specialnstart", "game_specialairnstart"], category = ACMD_GAME, low_priority )]
unsafe fn packun_special_n_start_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let stance = VarModule::get_int(boma.object(), vars::packun::instance::CURRENT_STANCE);
    frame(lua_state, 1.0);
    if stance != 2 {
        FT_MOTION_RATE(fighter, 0.7);
    }
    else {
        FT_MOTION_RATE(fighter, 9.0/(9.0 - 1.0));
    }
    frame(lua_state, 9.0);
    FT_MOTION_RATE(fighter, 1.0);
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_PACKUN_STATUS_SPECIAL_N_FLAG_GENERATE_ARTICLE_SPIKEBALL);
    }
    frame(lua_state, 11.0);
    FT_MOTION_RATE(fighter, 0.7);
}

#[acmd_script( agent = "packun", scripts = [ "game_specialsshoot", "game_specialairsshoot" ] , category = ACMD_GAME , low_priority)]
unsafe fn packun_special_s_shoot_game(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let stance = VarModule::get_int(boma.object(), vars::packun::instance::CURRENT_STANCE);
    let charged = WorkModule::get_int(boma, *FIGHTER_PACKUN_INSTANCE_WORK_ID_INT_SPECIAL_S_COUNT) == 75;
    let hit = false;
    if is_excute(agent) {
        FighterAreaModuleImpl::enable_fix_jostle_area(boma, 10.0, 3.0);
    }
    if stance == 0 {
        FT_DESIRED_RATE(agent, 5.0, 6.0);
        frame(lua_state, 5.0);
        FT_MOTION_RATE(agent, 1.0);
        if is_excute(agent) {
            if charged {
                ATTACK(agent, 0, 0, Hash40::new("mouth"), 14.0, 30, 66, 0, 60, 9.0, 2.0, 0.0, 0.0, Some(7.0), Some(0.0), Some(0.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BITE);
            }
            else {
                ATTACK(agent, 0, 0, Hash40::new("mouth"), 10.0, 30, 66, 0, 60, 9.0, 2.0, 0.0, 0.0, Some(7.0), Some(0.0), Some(0.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BITE);
            }
            VarModule::on_flag(boma.object(), vars::packun::status::FLAME_ACTIVE);
        }
        wait(lua_state, 5.0);
        FT_DESIRED_RATE(agent, 40.0, 30.0);
        if is_excute(agent) {
            AttackModule::clear_all(boma);
            VarModule::off_flag(boma.object(), vars::packun::status::FLAME_ACTIVE);
        }
    }
    else if stance == 1 {
        frame(lua_state, 2.0);
        if !WorkModule::is_flag(boma, *FIGHTER_PACKUN_STATUS_SPECIAL_S_FLAG_FAILURE) {
            if is_excute(agent) {
                ATTACK(agent, 0, 0, Hash40::new("top"), 0.0, 18, 100, 30, 0, 5.0, 0.0, 7.0, 7.0, Some(0.0), Some(7.0), Some(10.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 0, false, false, true, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
            }
        }
        frame(lua_state, 10.0);
        if !WorkModule::is_flag(boma, *FIGHTER_PACKUN_STATUS_SPECIAL_S_FLAG_FAILURE) {
            if is_excute(agent) {
                ArticleModule::generate_article(boma, *FIGHTER_PACKUN_GENERATE_ARTICLE_POISONBREATH, false, -1);
            }
        }
        frame(lua_state, 21.0);
        FT_MOTION_RATE(agent, 0.9);
        if is_excute(agent) {
            FighterAreaModuleImpl::enable_fix_jostle_area(boma, 5.0, 5.0);
            AttackModule::clear_all(boma);
        }
        wait(lua_state, 10.0);
        if is_excute(agent) {
            WorkModule::on_flag(boma, *FIGHTER_PACKUN_STATUS_SPECIAL_S_FLAG_CHANGE_KINETIC);
        }
        frame(lua_state, 20.0);
        FT_MOTION_RATE(agent, 0.55);
    }
    else if stance == 2 {
        FT_MOTION_RATE(agent, (11.0/5.0));
        frame(lua_state, 5.0);
        FT_MOTION_RATE(agent, 1.0);
        if is_excute(agent) {
            if charged {
                ATTACK(agent, 0, 0, Hash40::new("mouth"), 20.0, 180, 100, 30, 0, 9.0, 2.0, 0.0, 0.0, Some(7.0), Some(0.0), Some(0.0), 1.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_saving"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BITE);
                ATTACK(agent, 1, 0, Hash40::new("mouth"), 20.0, 90, 100, 80, 0, 9.0, 2.0, 0.0, 0.0, Some(7.0), Some(0.0), Some(0.0), 1.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_saving"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BITE);
                AttackModule::set_add_reaction_frame(boma, 0, 17.0, false);
            }
            else {
                ATTACK(agent, 0, 0, Hash40::new("mouth"), 13.0, 180, 100, 30, 0, 9.0, 2.0, 0.0, 0.0, Some(7.0), Some(0.0), Some(0.0), 1.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 8.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BITE);
                ATTACK(agent, 1, 0, Hash40::new("mouth"), 13.0, 90, 100, 80, 0, 9.0, 2.0, 0.0, 0.0, Some(7.0), Some(0.0), Some(0.0), 1.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 8.0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BITE);
                ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 2.0);
                AttackModule::set_add_reaction_frame(boma, 0, 9.0, false);
            }
        }
        wait(lua_state, 3.0);
        if stance == 2 {
            if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) || AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_SHIELD) {
                FT_MOTION_RATE(agent, 0.5);
            }
        }
        if is_excute(agent) {
            if stance == 2 {
                AttackModule::clear_all(boma);
            }
        }
    }
}

#[acmd_script( agent = "packun", script = "effect_specialsend" , category = ACMD_EFFECT , low_priority)]
unsafe fn packun_special_s_end_effect(agent: &mut L2CAgentBase) {
	let lua_state = agent.lua_state_agent;
	let boma = agent.boma();
    let stance = VarModule::get_int(boma.object(), vars::packun::instance::CURRENT_STANCE);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
        if stance == 0 {
            EFFECT_FOLLOW(agent, Hash40::new("sys_flash"), Hash40::new("mouth"), 2, -2, 0, 0, 0, 0, 0.75, false);
            EFFECT_FOLLOW(agent, Hash40::new("sys_hit_fire"), Hash40::new("mouth"), 3, -1, 0, 0, 0, 0, 0.6, true);
        }
        if stance == 1 {
            EFFECT_FOLLOW(agent, Hash40::new("packun_poison_max"), Hash40::new("mouth"), 3, -1, 0, 0, 0, 0, 1, true);
        }
        if stance == 2 {
            EFFECT_FOLLOW_FLIP(agent, Hash40::new("packun_bite_line2"), Hash40::new("packun_bite_line2"), Hash40::new("mouth"), 5, -3, 0, 10, 50, -20, 0.9, true, *EF_FLIP_YZ);
            LAST_EFFECT_SET_RATE(agent, 0.5);
            EFFECT_FOLLOW_FLIP(agent, Hash40::new("packun_bite"), Hash40::new("packun_bite"), Hash40::new("mouth"), 3, 0, 0, 0, -150, 20, 0.9, true, *EF_FLIP_YZ);
            LAST_EFFECT_SET_RATE(agent, 0.5);
        }
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        EFFECT_DETACH_KIND(agent, Hash40::new("packun_poison_max"), -1);
    }
}

#[acmd_script( agent = "packun", script = "effect_specialsshoot" , category = ACMD_EFFECT , low_priority)]
unsafe fn packun_special_s_shoot_effect(agent: &mut L2CAgentBase) {
	let lua_state = agent.lua_state_agent;
	let boma = agent.boma();
    let stance = VarModule::get_int(boma.object(), vars::packun::instance::CURRENT_STANCE);
    if is_excute(agent) {
        LANDING_EFFECT(agent, Hash40::new("sys_action_smoke_h"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 4.0);
    if is_excute(agent) {
        if stance == 0 {
            EFFECT(agent, Hash40::new("packun_spikeball_shoot"), Hash40::new("mouth"), 2, -0.6, 0, 0, 90, -100, 1, 0, 0, 0, 0, 0, 0, true);
            LAST_EFFECT_SET_COLOR(agent, 1.0, 0.35, 0.02);
            let effect = if WorkModule::get_int(boma, *FIGHTER_PACKUN_INSTANCE_WORK_ID_INT_SPECIAL_S_COUNT) == 75 { Hash40::new("sys_flame") } else { Hash40::new("packun_atk_air_b_fire") };
            let size = if WorkModule::get_int(boma, *FIGHTER_PACKUN_INSTANCE_WORK_ID_INT_SPECIAL_S_COUNT) == 75 { 0.8 } else { 1.5 };
            EFFECT_FOLLOW(agent, effect, Hash40::new("mouth"), 7.5, 0, 0, 0, 0, 0, size, true);
        }
        else if stance == 2 {
            EFFECT_FOLLOW_NO_STOP_FLIP(agent, Hash40::new("packun_bite_line"), Hash40::new("packun_bite_line"), Hash40::new("top"), -5, 11, 19, 0, -130, 35, 1, true, *EF_FLIP_YZ);
        }
        wait(lua_state, 1.0);
        if is_excute(agent) {
            if stance == 2 {
                EFFECT_FOLLOW_FLIP(agent, Hash40::new("packun_bite_line2"), Hash40::new("packun_bite_line2"), Hash40::new("top"), -12, 9, 20, 10, 50, 10, 0.8, true, *EF_FLIP_YZ);
                EFFECT_FOLLOW_FLIP(agent, Hash40::new("packun_bite"), Hash40::new("packun_bite"), Hash40::new("top"), -9, 11, 18, 0, -120, 20, 1, true, *EF_FLIP_YZ);

            }
        }
    }
    frame(lua_state, 6.0);
    if stance == 1 {
        if WorkModule::is_flag(boma, *FIGHTER_PACKUN_STATUS_SPECIAL_S_FLAG_FAILURE) {
            if is_excute(agent) {
                agent.clear_lua_stack();
                lua_args!(agent, Hash40::new("packun_poison_breath2"), Hash40::new("mouth"), 5, -0.6, 0, 0, 90, -100, 1.2, true);
                smash::app::sv_animcmd::EFFECT_FOLLOW_NO_SCALE(agent.lua_state_agent);
                agent.pop_lua_stack(1);
            }
        }
        else {
            if is_excute(agent) {
                agent.clear_lua_stack();
                lua_args!(agent, Hash40::new("packun_poison_breath"), Hash40::new("mouth"), 5, -0.6, 0, 0, 90, -100, 1.1, true);
                smash::app::sv_animcmd::EFFECT_FOLLOW_NO_SCALE(agent.lua_state_agent);
                agent.pop_lua_stack(1);
                LAST_EFFECT_SET_RATE(agent, 1.6);
            }
        }
    }
    frame(lua_state, 25.0);
    if is_excute(agent) {
        EFFECT_DETACH_KIND(agent, Hash40::new("packun_poison_breath"), -1);
    }
    frame(lua_state, 29.0);
    if stance == 1 {
        if WorkModule::is_flag(boma, *FIGHTER_PACKUN_STATUS_SPECIAL_S_FLAG_FAILURE) {
            if is_excute(agent) {
                agent.clear_lua_stack();
                lua_args!(agent, Hash40::new("packun_poison_mouth2"), Hash40::new("mouth"), 6, -1.5, 0, 0, 90, -130, 1, true);
                smash::app::sv_animcmd::EFFECT_FOLLOW_NO_SCALE(agent.lua_state_agent);
                agent.pop_lua_stack(1);
            }
        }
        else {
            if is_excute(agent) {
                agent.clear_lua_stack();
                lua_args!(agent, Hash40::new("packun_poison_mouth"), Hash40::new("mouth"), 4, -2, 0, 0, 90, -125, 1, true);
                smash::app::sv_animcmd::EFFECT_FOLLOW_NO_SCALE(agent.lua_state_agent);
                agent.pop_lua_stack(1);
            }
        }
    }
}

#[acmd_script( agent = "packun", script = "effect_specialairsend", category = ACMD_EFFECT, low_priority )]
unsafe fn packun_special_air_s_end_effect(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
	let boma = agent.boma();
    let stance = VarModule::get_int(boma.object(), vars::packun::instance::CURRENT_STANCE);
    if is_excute(agent) {
        if stance == 0 {
            EFFECT_FOLLOW(agent, Hash40::new("sys_flash"), Hash40::new("mouth"), 2, -2, 0, 0, 0, 0, 0.75, false);
            EFFECT_FOLLOW(agent, Hash40::new("sys_hit_fire"), Hash40::new("mouth"), 3, -1, 0, 0, 0, 0, 0.6, true);
        }
        if stance == 1 {
            EFFECT_FOLLOW(agent, Hash40::new("packun_poison_max"), Hash40::new("mouth"), 3, -1, 0, 0, 0, 0, 1, true);
        }
        if stance == 2 {
            EFFECT_FOLLOW_FLIP(agent, Hash40::new("packun_bite_line2"), Hash40::new("packun_bite_line2"), Hash40::new("mouth"), 5, -3, 0, 10, 50, -20, 0.9, true, *EF_FLIP_YZ);
            LAST_EFFECT_SET_RATE(agent, 0.5);
            EFFECT_FOLLOW_FLIP(agent, Hash40::new("packun_bite"), Hash40::new("packun_bite"), Hash40::new("mouth"), 3, 0, 0, 0, -150, 20, 0.9, true, *EF_FLIP_YZ);
            LAST_EFFECT_SET_RATE(agent, 0.5);
        }
    }
    frame(lua_state, 8.0);
    if is_excute(agent) {
        EFFECT_DETACH_KIND(agent, Hash40::new("packun_poison_max"), -1);
    }
}

#[acmd_script( agent = "packun", script = "effect_specialairsshoot", category = ACMD_EFFECT, low_priority )]
unsafe fn packun_special_air_s_shoot_effect(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
	let boma = agent.boma();
    let stance = VarModule::get_int(boma.object(), vars::packun::instance::CURRENT_STANCE);
    frame(lua_state, 4.0);
    if is_excute(agent) {
        if stance == 0 {
            EFFECT(agent, Hash40::new("packun_spikeball_shoot"), Hash40::new("mouth"), 2, -0.6, 0, 0, 90, -100, 1, 0, 0, 0, 0, 0, 0, true);
            LAST_EFFECT_SET_COLOR(agent, 1.0, 0.35, 0.02);
            let effect = if WorkModule::get_int(boma, *FIGHTER_PACKUN_INSTANCE_WORK_ID_INT_SPECIAL_S_COUNT) == 75 { Hash40::new("sys_flame") } else { Hash40::new("packun_atk_air_b_fire") };
            let size = if WorkModule::get_int(boma, *FIGHTER_PACKUN_INSTANCE_WORK_ID_INT_SPECIAL_S_COUNT) == 75 { 0.8 } else { 1.5 };
            EFFECT_FOLLOW(agent, effect, Hash40::new("mouth"), 7.5, 0, 0, 0, 0, 0, size, true);
        }
        else if stance == 2 {
            EFFECT_FOLLOW_NO_STOP_FLIP(agent, Hash40::new("packun_bite_line"), Hash40::new("packun_bite_line"), Hash40::new("top"), -5, 11, 19, 0, -130, 35, 1, true, *EF_FLIP_YZ);
        }
        wait(lua_state, 1.0);
        if is_excute(agent) {
            if stance == 2 {
                EFFECT_FOLLOW_FLIP(agent, Hash40::new("packun_bite_line2"), Hash40::new("packun_bite_line2"), Hash40::new("top"), -12, 9, 20, 10, 50, 10, 0.8, true, *EF_FLIP_YZ);
                EFFECT_FOLLOW_FLIP(agent, Hash40::new("packun_bite"), Hash40::new("packun_bite"), Hash40::new("top"), -9, 11, 18, 0, -120, 20, 1, true, *EF_FLIP_YZ);

            }
        }
    }
    frame(lua_state, 6.0);
    if stance == 1 {
        if WorkModule::is_flag(boma, *FIGHTER_PACKUN_STATUS_SPECIAL_S_FLAG_FAILURE) {
            if is_excute(agent) {
                agent.clear_lua_stack();
                lua_args!(agent, Hash40::new("packun_poison_breath2"), Hash40::new("mouth"), 5, -0.6, 0, 0, 90, -100, 1.2, true);
                smash::app::sv_animcmd::EFFECT_FOLLOW_NO_SCALE(agent.lua_state_agent);
                agent.pop_lua_stack(1);
            }
        }
        else {
            if is_excute(agent) {
                agent.clear_lua_stack();
                lua_args!(agent, Hash40::new("packun_poison_breath"), Hash40::new("mouth"), 5, -0.6, 0, 0, 90, -100, 1.1, true);
                smash::app::sv_animcmd::EFFECT_FOLLOW_NO_SCALE(agent.lua_state_agent);
                agent.pop_lua_stack(1);
                LAST_EFFECT_SET_RATE(agent, 1.6);
            }
        }
    }
    frame(lua_state, 25.0);
    if is_excute(agent) {
        EFFECT_DETACH_KIND(agent, Hash40::new("packun_poison_breath"), -1);
    }
    frame(lua_state, 29.0);
    if stance == 1 {
        if WorkModule::is_flag(boma, *FIGHTER_PACKUN_STATUS_SPECIAL_S_FLAG_FAILURE) {
            if is_excute(agent) {
                agent.clear_lua_stack();
                lua_args!(agent, Hash40::new("packun_poison_mouth2"), Hash40::new("mouth"), 6, -1.5, 0, 0, 90, -130, 1, true);
                smash::app::sv_animcmd::EFFECT_FOLLOW_NO_SCALE(agent.lua_state_agent);
                agent.pop_lua_stack(1);
            }
        }
        else {
            if is_excute(agent) {
                agent.clear_lua_stack();
                lua_args!(agent, Hash40::new("packun_poison_mouth"), Hash40::new("mouth"), 4, -2, 0, 0, 90, -125, 1, true);
                smash::app::sv_animcmd::EFFECT_FOLLOW_NO_SCALE(agent.lua_state_agent);
                agent.pop_lua_stack(1);
            }
        }
    }
}

#[acmd_script( agent = "packun", scripts = ["sound_specialsshoot", "sound_specialairsshoot"], category = ACMD_SOUND, low_priority )]
unsafe fn packun_special_s_shoot_sound(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let stance = VarModule::get_int(boma.object(), vars::packun::instance::CURRENT_STANCE);
    frame(lua_state, 1.0);
    if is_excute(agent) {
        //sound!(agent, *MA_MSC_CMD_SOUND_STOP_SE_STATUS);
        if stance == 0 {
            PLAY_SE(agent, Hash40::new("se_packun_special_n03"));
            if WorkModule::get_int(boma, *FIGHTER_PACKUN_INSTANCE_WORK_ID_INT_SPECIAL_S_COUNT) == 75 {
                PLAY_SE(agent, Hash40::new("se_common_fire_m"));
            }
        }
    }
    frame(lua_state, 3.0);
    if is_excute(agent) {
        if stance == 1 {
            PLAY_SE(agent, Hash40::new("se_packun_special_s03"));
        }
        else if stance == 2 {
            PLAY_SE(agent, Hash40::new("se_packun_attackhard_s03"));
        }
    }
    wait(lua_state, 3.0);
    if is_excute(agent) {
        if stance == 2 {
            PLAY_SE(agent, Hash40::new("se_packun_attackhard_s04"));
        }
    }
}


#[acmd_script( agent = "packun", script = "game_specialhi" , category = ACMD_GAME , low_priority)]
unsafe fn packun_special_hi_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let stance = StanceInfo::from(VarModule::get_int(boma.object(), vars::packun::instance::CURRENT_STANCE));
    FT_MOTION_RATE(fighter, (10.0/15.0));
    frame(lua_state, 15.0);
    FT_MOTION_RATE(fighter, 1.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("arml"), 10.0 * stance.damage_other, 30, 70, 0, 70, 3.5, 6.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("armr"), 10.0 * stance.damage_other, 30, 70, 0, 70, 3.5, 6.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 2, 0, Hash40::new("arml"), 10.0 * stance.damage_other, 30, 70, 0, 70, 5.0, 1.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 3, 0, Hash40::new("armr"), 10.0 * stance.damage_other, 30, 70, 0, 70, 5.0, 1.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 38.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("arml"), 7.0 * stance.damage_other, 40, 70, 0, 70, 3.5, 6.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("armr"), 7.0 * stance.damage_other, 40, 70, 0, 70, 3.5, 6.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 2, 0, Hash40::new("arml"), 7.0 * stance.damage_other, 40, 70, 0, 70, 5.0, 1.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 3, 0, Hash40::new("armr"), 7.0 * stance.damage_other, 40, 70, 0, 70, 5.0, 1.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 70.0);
    FT_MOTION_RATE(fighter, 0.5);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 86.0);
    FT_MOTION_RATE(fighter, 3.03);
    frame(lua_state, 90.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_PACKUN_STATUS_SPECIAL_HI_DIVE);
    }

}

#[acmd_script( agent = "packun", script = "game_specialairhi" , category = ACMD_GAME , low_priority)]
unsafe fn packun_special_air_hi_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let stance = StanceInfo::from(VarModule::get_int(boma.object(), vars::packun::instance::CURRENT_STANCE));
    FT_MOTION_RATE(fighter, (10.0/15.0));
    frame(lua_state, 15.0);
    FT_MOTION_RATE(fighter, 1.0);
    if is_excute(fighter) {
        if !boma.is_status(*FIGHTER_PACKUN_STATUS_KIND_SPECIAL_HI_END) {
            ATTACK(fighter, 0, 0, Hash40::new("arml"), 10.0 * stance.damage_other, 40, 70, 0, 50, 3.5, 6.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 1, 0, Hash40::new("armr"), 10.0 * stance.damage_other, 40, 70, 0, 50, 3.5, 6.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 2, 0, Hash40::new("arml"), 10.0 * stance.damage_other, 40, 70, 0, 50, 5.0, 1.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 3, 0, Hash40::new("armr"), 10.0 * stance.damage_other, 40, 70, 0, 50, 5.0, 1.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
        }
    }
    frame(lua_state, 38.0);
    if is_excute(fighter) {
        if !boma.is_status(*FIGHTER_PACKUN_STATUS_KIND_SPECIAL_HI_END) {
            ATTACK(fighter, 0, 0, Hash40::new("arml"), 7.0 * stance.damage_other, 50, 70, 0, 50, 3.5, 6.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 1, 0, Hash40::new("armr"), 7.0 * stance.damage_other, 50, 70, 0, 50, 3.5, 6.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 2, 0, Hash40::new("arml"), 7.0 * stance.damage_other, 50, 70, 0, 50, 5.0, 1.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 3, 0, Hash40::new("armr"), 7.0 * stance.damage_other, 50, 70, 0, 50, 5.0, 1.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
        }
    }
    frame(lua_state, 70.0);
    FT_MOTION_RATE(fighter, 0.5);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 86.0);
    FT_MOTION_RATE(fighter, 3.03);
    frame(lua_state, 90.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_PACKUN_STATUS_SPECIAL_HI_DIVE);
    }

}

#[acmd_script( agent = "packun", script = "game_speciallwbiteattack", category = ACMD_GAME, low_priority )]
unsafe fn packun_special_lw_bite_attack_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let stance = StanceInfo::from(VarModule::get_int(boma.object(), vars::packun::instance::CURRENT_STANCE));
    if WorkModule::get_float(boma, *FIGHTER_PACKUN_STATUS_SPECIAL_LW_WORK_FLOAT_CHARGE_RATE) > 1.0 {
        if is_excute(fighter) {
            if stance.label == 1 {
                ATTACK(fighter, 0, 0, Hash40::new("mouth"), 20.0, 55, 55, 0, 75, 7.9, 2.4, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_curse_poison"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BITE);
                AttackModule::set_poison_param(boma, 0, 136, 45, 3.0, false);
            }
            else {
                ATTACK(fighter, 0, 0, Hash40::new("mouth"), 26.0 * stance.damage_head, 55, 55, 0, 75, 7.9, 2.4, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BITE);
            }
        }
    }
    else {
        if is_excute(fighter) {
            if stance.label == 1 {
                ATTACK(fighter, 0, 0, Hash40::new("mouth"), 0.0, 55, 55, 0, 75, 7.9, 2.4, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_curse_poison"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BITE);
                AttackModule::set_poison_param(boma, 0, 136, 45, 2.0, false);
            }
            else {
                ATTACK(fighter, 0, 0, Hash40::new("mouth"), 0.0 * stance.damage_head, 55, 55, 0, 75, 7.9, 2.4, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BITE);
            }
            WorkModule::on_flag(boma, *FIGHTER_PACKUN_STATUS_SPECIAL_LW_FLAG_ATTACK_LERP);
        }
    }
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_PACKUN_STATUS_SPECIAL_LW_FLAG_BITE_ATTACK);
        search!(fighter, *MA_MSC_CMD_SEARCH_SEARCH_SCH_CLR_ALL);
    }
}

#[acmd_script( agent = "packun", script = "game_speciallwbite_attack", category = ACMD_GAME, low_priority )]
unsafe fn packun_special_lw_bite__attack_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let stance = StanceInfo::from(VarModule::get_int(boma.object(), vars::packun::instance::CURRENT_STANCE));
    if WorkModule::get_float(boma, *FIGHTER_PACKUN_STATUS_SPECIAL_LW_WORK_FLOAT_CHARGE_RATE) > 1.0 {
        if is_excute(fighter) {
            if stance.label == 1 {
                ATTACK(fighter, 0, 0, Hash40::new("mouth"), 20.0, 55, 55, 0, 75, 7.9, 2.4, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_curse_poison"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BITE);
                AttackModule::set_poison_param(boma, 0, 136, 45, 3.0, false);
            }
            else {
                ATTACK(fighter, 0, 0, Hash40::new("mouth"), 26.0 * stance.damage_head, 55, 55, 0, 75, 7.9, 2.4, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BITE);
            }
        }
    }
    else {
        if is_excute(fighter) {
            if stance.label == 1 {
                ATTACK(fighter, 0, 0, Hash40::new("mouth"), 0.0, 55, 55, 0, 75, 7.9, 2.4, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_curse_poison"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BITE);
                AttackModule::set_poison_param(boma, 0, 136, 45, 2.0, false);
            }
            else {
                ATTACK(fighter, 0, 0, Hash40::new("mouth"), 0.0 * stance.damage_head, 55, 55, 0, 75, 7.9, 2.4, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BITE);
            }
            WorkModule::on_flag(boma, *FIGHTER_PACKUN_STATUS_SPECIAL_LW_FLAG_ATTACK_LERP);
        }
    }
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_PACKUN_STATUS_SPECIAL_LW_FLAG_BITE_ATTACK);
        search!(fighter, *MA_MSC_CMD_SEARCH_SEARCH_SCH_CLR_ALL);
    }
}

#[acmd_script( agent = "packun", script = "effect_speciallwbite", category = ACMD_EFFECT, low_priority )]
unsafe fn packun_special_lw_bite_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let stance = StanceInfo::from(VarModule::get_int(boma.object(), vars::packun::instance::CURRENT_STANCE));
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_v_smoke_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("packun_longrange_start"), Hash40::new("mouth"), 0, 0, 0, 180, 0, 0, 0.85, true);
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("packun_longrange_bite_line"), Hash40::new("mouth"), 6, 0, 0, 0, 90, 0, 0.8, true);
    }
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        if stance.label == 1 {
            EFFECT_FOLLOW(fighter, Hash40::new("packun_poison_max"), Hash40::new("mouth"), 6, -0.6, 0, 0, 0, 0, 1.0, true);
            LAST_EFFECT_SET_COLOR(fighter, 0.5, 0.5, 0.5);
            EFFECT_FOLLOW(fighter, Hash40::new("packun_poison_max"), Hash40::new("mouth"), 6, -0.6, 0, 0, 0, 0, 0.9, true);
            EFFECT_FOLLOW(fighter, Hash40::new("packun_poison_mouth"), Hash40::new("mouth"), 6, -0.6, 0, 0, 0, 0, 1.0, true);
            LAST_EFFECT_SET_RATE(fighter, 2.0);
        }
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        let size = if stance.label != 2 { 0.85 } else { 1.0 };
        EFFECT_FOLLOW(fighter, Hash40::new("packun_longrange_bite"), Hash40::new("mouth"), 0, 0, 0, 180, 0, 0, size, true);
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        EFFECT_DETACH_KIND(fighter, Hash40::new("packun_longrange_bite"), -1);
    }
    frame(lua_state, 17.0);
    if is_excute(fighter) {
        let size = if stance.label != 2 { 0.9 } else { 1.0 };
        EFFECT_FOLLOW(fighter, Hash40::new("packun_longrange_bite_line2"), Hash40::new("mouth"), 5, 0, 0, 0, 90, 0, size, true);
    }
}

#[acmd_script( agent = "packun", script = "effect_specialairlwbite", category = ACMD_EFFECT, low_priority )]
unsafe fn packun_special_air_lw_bite_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let stance = StanceInfo::from(VarModule::get_int(boma.object(), vars::packun::instance::CURRENT_STANCE));
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("packun_longrange_start"), Hash40::new("mouth"), 0, 0, 0, 180, 0, 0, 0.85, true);
    }
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        if stance.label == 1 {
            EFFECT_FOLLOW(fighter, Hash40::new("packun_poison_max"), Hash40::new("mouth"), 6, -0.6, 0, 0, 0, 0, 1.0, true);
            LAST_EFFECT_SET_COLOR(fighter, 0.5, 0.5, 0.5);
            EFFECT_FOLLOW(fighter, Hash40::new("packun_poison_max"), Hash40::new("mouth"), 6, -0.6, 0, 0, 0, 0, 0.9, true);
            EFFECT_FOLLOW(fighter, Hash40::new("packun_poison_mouth"), Hash40::new("mouth"), 6, -0.6, 0, 0, 0, 0, 1.0, true);
            LAST_EFFECT_SET_RATE(fighter, 2.0);
        }
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        let size = if stance.label != 2 { 0.85 } else { 1.0 };
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("packun_longrange_bite_line"), Hash40::new("mouth"), 6, 0, 0, 0, 90, 0, 0.9, true);
        EFFECT_FOLLOW(fighter, Hash40::new("packun_longrange_bite"), Hash40::new("mouth"), 0, 0, 0, 180, 0, 0, size, true);
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        EFFECT_DETACH_KIND(fighter, Hash40::new("packun_longrange_bite"), -1);
    }
    frame(lua_state, 17.0);
    if is_excute(fighter) {
        let size = if stance.label != 2 { 0.9 } else { 1.0 };
        EFFECT_FOLLOW(fighter, Hash40::new("packun_longrange_bite_line2"), Hash40::new("mouth"), 5, 0, 0, 0, 90, 0, size, true);
    }
}

pub fn install() {
    install_acmd_scripts!(
        packun_special_n_start_game,
        packun_special_s_shoot_game,
        packun_special_s_end_effect,
        packun_special_s_shoot_effect,
        packun_special_air_s_end_effect,
        packun_special_air_s_shoot_effect,
        packun_special_s_shoot_sound,
        packun_special_hi_game,
        packun_special_air_hi_game,
        packun_special_lw_bite_attack_game,
        packun_special_lw_bite__attack_game,
        packun_special_lw_bite_effect,
        packun_special_air_lw_bite_effect,
    );
}
