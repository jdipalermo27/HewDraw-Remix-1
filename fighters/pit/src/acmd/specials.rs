
use super::*;

#[acmd_script( agent = "pit", script = "game_specialairnfires" , category = ACMD_GAME , low_priority)]
unsafe fn pit_special_n_fire_s_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        let stop_rise = Vector3f{x: 1.0, y: 0.0, z: 1.0};
        KineticModule::mul_speed(boma, &stop_rise, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        let hop_vec = Vector3f{ x: 0.0, y: 1.0, z: 0.0 };
        KineticModule::add_speed(boma, &hop_vec);
        ArticleModule::shoot(boma, *FIGHTER_PIT_GENERATE_ARTICLE_BOWARROW, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
    }
}

#[acmd_script( agent = "pit", script = "game_specialairnfirehi" , category = ACMD_GAME , low_priority)]
unsafe fn pit_special_n_fire_hi_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        let stop_rise = Vector3f{x: 1.0, y: 0.0, z: 1.0};
        KineticModule::mul_speed(boma, &stop_rise, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        let hop_vec = Vector3f{ x: 0.0, y: 1.0, z: 0.0 };
        KineticModule::add_speed(boma, &hop_vec);
        ArticleModule::shoot(boma, *FIGHTER_PIT_GENERATE_ARTICLE_BOWARROW, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
    }
}

#[acmd_script( agent = "pit", script = "game_specialsstart" , category = ACMD_GAME , low_priority)]
unsafe fn pit_speacial_s_start_game(fighter: &mut L2CAgentBase) { 
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 11.0);
    if(is_excute){
        WorkModule::on_flag(Flag=FIGHTER_PIT_STATUS_SPECIAL_S_WORK_ID_FLAG_MOVE_FRONT)
        }
    frame(lua_state, 16.0);
    if(is_excute){
        ATTACK(0, 0, hash40("top"), 0.0, 361, 0, 0, 0, 2.0, 0.0, 12.0, 9.0, 0.0, 4.0, 9.0, 1.0, 1.0, ATTACK_SETOFF_KIND_OFF, ATTACK_LR_CHECK_F, false, hash40("no"), 0.0, 0, false, false, false, false, false, COLLISION_SITUATION_MASK_GA, COLLISION_CATEGORY_MASK_NO_FLOOR, COLLISION_PART_MASK_ALL, false, hash40("collision_attr_search"), ATTACK_SOUND_LEVEL_S, COLLISION_SOUND_ATTR_NONE, ATTACK_REGION_NONE)
    }
    frame(lua_state, 31.0);
    if(is_excute){
        WorkModule::on_flag(Flag=FIGHTER_PIT_STATUS_SPECIAL_S_WORK_ID_FLAG_GRAVITY_ONOFF)
    }
    frame(lua_state, 36);
    if(is_excute){
        WorkModule::on_flag(Flag=FIGHTER_PIT_STATUS_SPECIAL_S_WORK_ID_FLAG_MTRANS_AIR_UNABLE)
        AttackModule::clear_all()
    }
}



pub fn install() {
    install_acmd_scripts!(
        pit_special_n_fire_s_game,
        pit_special_n_fire_hi_game,
        pit_special_s_start,
    );
}

