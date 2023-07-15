// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

unsafe fn final_cutter_landing_bugfix(fighter: &mut L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_KIRBY_STATUS_KIND_SPECIAL_HI2)
    && MotionModule::frame(fighter.module_accessor) <= 2.0 {
        fighter.set_situation(L2CValue::I32(*SITUATION_KIND_AIR));
    }
}

unsafe fn horizontal_cutter(fighter: &mut L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_STATUS_KIND_SPECIAL_HI) {
        if (((fighter.is_situation(*SITUATION_KIND_GROUND) && fighter.status_frame() == 15)
            || (fighter.is_situation(*SITUATION_KIND_AIR) && fighter.status_frame() == 17))
            && ControlModule::get_stick_x(fighter.module_accessor).abs() >= 0.85)
            && ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL_RAW) {
            if ControlModule::get_stick_x(fighter.module_accessor) * PostureModule::lr(fighter.module_accessor) < 0.0 {
                REVERSE_LR(fighter);
            }
            let hcutter_status = CustomStatusModule::get_agent_status_kind(fighter.battle_object, statuses::kirby::SPECIAL_HI_H);
            StatusModule::change_status_request_from_script(fighter.module_accessor, hcutter_status, false);
        }
    }
}

// unsafe fn disable_dash_attack_slideoff(fighter: &mut L2CFighterCommon) {
//     if fighter.is_status(*FIGHTER_STATUS_KIND_ATTACK_DASH) && AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD) {
//         VarModule::off_flag(fighter.battle_object, vars::common::status::ATTACK_DASH_ENABLE_AIR_FALL);
//         VarModule::off_flag(fighter.battle_object, vars::common::status::ATTACK_DASH_ENABLE_AIR_CONTINUE);
//     }
// }

// unsafe fn stone_control(fighter: &mut L2CFighterCommon) {
//     if fighter.is_status(*FIGHTER_STATUS_KIND_SPECIAL_LW) && fighter.status_frame() <= 30 {
//         fighter.clear_lua_stack();
//         lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, 1.0, 0.0);
//         app::sv_kinetic_energy::set_limit_speed(fighter.lua_state_agent);
//         fighter.clear_lua_stack();
//     }
// }

#[fighter_frame( agent = FIGHTER_KIND_KIRBY )]
pub fn hammer_landcancel(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        if fighter.is_status(*FIGHTER_KIRBY_STATUS_KIND_SPECIAL_S_ATTACK) {
            if fighter.is_situation(*SITUATION_KIND_GROUND) && fighter.is_prev_situation(*SITUATION_KIND_AIR) {
                AttackModule::clear_all(fighter.module_accessor);
                MotionModule::change_motion_force_inherit_frame(fighter.module_accessor, Hash40::new("special_s"), 33.0, 1.0, 1.0);
                MotionModule::set_rate(fighter.module_accessor, (55.0 - 33.0)/25.0);    // equates to 17F landing lag
            }
        }
    }
}

// Magic Series
unsafe fn magic_series(boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    let cat1 = cat[0];
    let cat4 = cat[3];
    if(    (WorkModule::get_int(boma, *FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_COPY_CHARA) == FIGHTER_KIND_RYU)
        || (WorkModule::get_int(boma, *FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_COPY_CHARA) == FIGHTER_KIND_KEN)
        || (WorkModule::get_int(boma, *FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_COPY_CHARA) == FIGHTER_KIND_LUCARIO)
        || (WorkModule::get_int(boma, *FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_COPY_CHARA) == FIGHTER_KIND_DOLLY)
        || (WorkModule::get_int(boma, *FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_COPY_CHARA) == FIGHTER_KIND_BAYONETTA)){
        // Level 1: Jab and Dash Attack Cancels
        if [*FIGHTER_STATUS_KIND_ATTACK, *FIGHTER_STATUS_KIND_ATTACK_DASH].contains(&status_kind) {
            if (AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) && !boma.is_in_hitlag())
                || (AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_SHIELD) && !boma.is_in_hitlag()) {
                // Check for tilt attack inputs
                if boma.is_cat_flag(Cat1::AttackS3) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_S3,false);
                }
                if boma.is_cat_flag(Cat1::AttackHi3) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_HI3,false);
                }
                if boma.is_cat_flag(Cat1::AttackLw3) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_LW3,false);
                }

                // Check for smash attack inputs
                if boma.is_cat_flag(Cat1::AttackS4) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_S4_START,true);
                }
                if boma.is_cat_flag(Cat1::AttackHi4) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_HI4_START,true);
                }
                if boma.is_cat_flag(Cat1::AttackLw4) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_LW4_START,true);
                }

                // Check for special attack inputs
                if boma.is_cat_flag(Cat1::SpecialN) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_N,false);
                }
                if boma.is_cat_flag(Cat1::SpecialS) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_S,false);
                }
                if boma.is_cat_flag(Cat1::SpecialHi) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_HI,false);
                }
                if boma.is_cat_flag(Cat1::SpecialLw) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_LW,false);
                }

                // Check for jump inputs during dash attack (on hit)
                if status_kind == *FIGHTER_STATUS_KIND_ATTACK_DASH
                && (AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT)
                && !boma.is_in_hitlag()) {
                    boma.check_jump_cancel(false);
                }

                //----------------------------------------------------------------------------------------------------------------------
                // Check for command inputs
                // Ryu
                if WorkModule::get_int(boma, *FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_COPY_CHARA) == *FIGHTER_KIND_RYU {
                    WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N_COMMAND);
                    WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N2_COMMAND);
                    if boma.is_cat_flag( Cat4::SpecialNCommand) {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_KIRBY_STATUS_KIND_RYU_SPECIAL_N_COMMAND, false);
                    }
                    if boma.is_cat_flag( Cat4::SpecialN2Command) {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_KIRBY_STATUS_KIND_RYU_SPECIAL_N2_COMMAND, false);
                    }
                }
                // Ken
                if WorkModule::get_int(boma, *FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_COPY_CHARA) == *FIGHTER_KIND_KEN {
                    WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N_COMMAND);
                    if boma.is_cat_flag( Cat4::AttackCommand1) {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_KIRBY_STATUS_KIND_KEN_SPECIAL_N_COMMAND, false);
                    }
                }
                //----------------------------------------------------------------------------------------------------------------------

            }
        }

        // Level 2: Tilt Cancels
        if [*FIGHTER_STATUS_KIND_ATTACK_S3,
            *FIGHTER_STATUS_KIND_ATTACK_HI3,
            *FIGHTER_STATUS_KIND_ATTACK_LW3].contains(&status_kind) {
            if (AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) && !boma.is_in_hitlag())
                || (AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_SHIELD) && !boma.is_in_hitlag()) {
                // Check for smash attack inputs
                if boma.is_cat_flag(Cat1::AttackS4) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_S4_START,true);
                }
                if boma.is_cat_flag(Cat1::AttackHi4) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_HI4_START,true);
                }
                if boma.is_cat_flag(Cat1::AttackLw4) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_LW4_START,true);
                }

                // Check for special attack inputs
                if boma.is_cat_flag(Cat1::SpecialN) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_N,false);
                }
                if boma.is_cat_flag(Cat1::SpecialS) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_S,false);
                }
                if boma.is_cat_flag(Cat1::SpecialHi) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_HI,false);
                }
                if boma.is_cat_flag(Cat1::SpecialLw) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_LW,false);
                }

                // Check for jump inputs during utilt
                if status_kind == *FIGHTER_STATUS_KIND_ATTACK_HI3
                && (AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT)
                && !boma.is_in_hitlag()) {
                    boma.check_jump_cancel(false);
                }

                //----------------------------------------------------------------------------------------------------------------------
                // Check for command inputs
                // Ryu
                if WorkModule::get_int(boma, *FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_COPY_CHARA) == *FIGHTER_KIND_RYU {
                    WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N_COMMAND);
                    WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N2_COMMAND);
                    if boma.is_cat_flag( Cat4::SpecialNCommand) {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_KIRBY_STATUS_KIND_RYU_SPECIAL_N_COMMAND, false);
                    }
                    if boma.is_cat_flag( Cat4::SpecialN2Command) {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_KIRBY_STATUS_KIND_RYU_SPECIAL_N2_COMMAND, false);
                    }
                }
                // Ken
                if WorkModule::get_int(boma, *FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_COPY_CHARA) == *FIGHTER_KIND_KEN {
                    WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N_COMMAND);
                    if boma.is_cat_flag( Cat4::AttackCommand1) {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_KIRBY_STATUS_KIND_KEN_SPECIAL_N_COMMAND, false);
                    }
                }
                //----------------------------------------------------------------------------------------------------------------------

            }
        }

        // Smash Cancels
        if [*FIGHTER_STATUS_KIND_ATTACK_S4,
            *FIGHTER_STATUS_KIND_ATTACK_HI4,
            *FIGHTER_STATUS_KIND_ATTACK_LW4].contains(&status_kind) {
            if (AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) && !boma.is_in_hitlag())
                || (AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_SHIELD) && !boma.is_in_hitlag()) {

                // Check for special attack inputs
                if boma.is_cat_flag(Cat1::SpecialN) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_N,false);
                }
                if boma.is_cat_flag(Cat1::SpecialS) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_S,false);
                }
                if boma.is_cat_flag(Cat1::SpecialHi) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_HI,false);
                }
                if boma.is_cat_flag(Cat1::SpecialLw) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_LW,false);
                }

                // Check for jump inputs
                if status_kind == *FIGHTER_STATUS_KIND_ATTACK_HI4
                && (AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT)
                && !boma.is_in_hitlag()) {
                    boma.check_jump_cancel(false);
                }

                //----------------------------------------------------------------------------------------------------------------------
                // Check for command inputs
                // Ryu
                if WorkModule::get_int(boma, *FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_COPY_CHARA) == *FIGHTER_KIND_RYU {
                    WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N_COMMAND);
                    WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N2_COMMAND);
                    if boma.is_cat_flag( Cat4::SpecialNCommand) {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_KIRBY_STATUS_KIND_RYU_SPECIAL_N_COMMAND, false);
                    }
                    if boma.is_cat_flag( Cat4::SpecialN2Command) {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_KIRBY_STATUS_KIND_RYU_SPECIAL_N2_COMMAND, false);
                    }
                }
                // Ken
                if WorkModule::get_int(boma, *FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_COPY_CHARA) == *FIGHTER_KIND_KEN {
                    WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N_COMMAND);
                    if boma.is_cat_flag( Cat4::AttackCommand1) {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_KIRBY_STATUS_KIND_KEN_SPECIAL_N_COMMAND, false);
                    }
                }
                //----------------------------------------------------------------------------------------------------------------------
            }
        }

        // Aerial Cancels
        if status_kind == *FIGHTER_STATUS_KIND_ATTACK_AIR {
            if (AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) && !boma.is_in_hitlag())
                || (AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_SHIELD) && !boma.is_in_hitlag()) {
                // Check for jump inputs
                if (AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) && !boma.is_in_hitlag()) {
                    boma.check_jump_cancel(false);
                }
                // Check for special attack inputs
                if boma.is_cat_flag(Cat1::SpecialN) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_N,false);
                }
                if boma.is_cat_flag(Cat1::SpecialS) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_S,false);
                }
                if boma.is_cat_flag(Cat1::SpecialHi) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_HI,false);
                }
                if boma.is_cat_flag(Cat1::SpecialLw) {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_LW,false);
                }

                //----------------------------------------------------------------------------------------------------------------------
                // Check for command inputs
                // Ryu
                if WorkModule::get_int(boma, *FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_COPY_CHARA) == *FIGHTER_KIND_RYU {
                    WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N_COMMAND);
                    WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N2_COMMAND);
                    if boma.is_cat_flag( Cat4::SpecialNCommand) {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_KIRBY_STATUS_KIND_RYU_SPECIAL_N_COMMAND, false);
                    }
                    if boma.is_cat_flag( Cat4::SpecialN2Command) {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_KIRBY_STATUS_KIND_RYU_SPECIAL_N2_COMMAND, false);
                    }
                }
                // Ken
                if WorkModule::get_int(boma, *FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_COPY_CHARA) == *FIGHTER_KIND_KEN {
                    WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N_COMMAND);
                    if boma.is_cat_flag( Cat4::AttackCommand1) {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_KIRBY_STATUS_KIND_KEN_SPECIAL_N_COMMAND, false);
                    }
                }
                //----------------------------------------------------------------------------------------------------------------------

            }
        }

    }
}

unsafe fn fastfall_specials(fighter: &mut L2CFighterCommon) {
    if !fighter.is_in_hitlag()
    && !StatusModule::is_changing(fighter.module_accessor)
    && fighter.is_status_one_of(&[
        *FIGHTER_STATUS_KIND_SPECIAL_N,
        *FIGHTER_STATUS_KIND_SPECIAL_S,
        *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_N_LOOP,
        *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_N_SPIT,
        *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_N_DRINK,
        *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_N_END,
        *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_N_SWALLOW,
        *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_N_EAT_FALL,
        *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_N_EAT_JUMP1,
        *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_N_EAT_JUMP2,
        *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_N_EAT_TURN_AIR,
        *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_N_EAT_WAIT_FALL,
        *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_N_EAT_WAIT_JUMP,
        *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_S_FALL,
        *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_S_JUMP,
        *FIGHTER_KIRBY_STATUS_KIND_SPECIAL_S_ATTACK,
        ])
    && fighter.is_situation(*SITUATION_KIND_AIR) {
        fighter.sub_air_check_dive();
        if fighter.is_flag(*FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE) {
            if [*FIGHTER_KINETIC_TYPE_MOTION_AIR, *FIGHTER_KINETIC_TYPE_MOTION_AIR_ANGLE].contains(&KineticModule::get_kinetic_type(fighter.module_accessor)) {
                fighter.clear_lua_stack();
                lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION);
                let speed_y = app::sv_kinetic_energy::get_speed_y(fighter.lua_state_agent);

                fighter.clear_lua_stack();
                lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, speed_y, 0.0, 0.0, 0.0);
                app::sv_kinetic_energy::reset_energy(fighter.lua_state_agent);

                fighter.clear_lua_stack();
                lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
                app::sv_kinetic_energy::enable(fighter.lua_state_agent);

                KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_MOTION, fighter.module_accessor);
            }
        }
    }
}

// Copy Abilities
// Fox Laser Land Cancel
unsafe fn fox_laser_fastfall_landcancel(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat2: i32, stick_y: f32) {
    if status_kind == *FIGHTER_KIRBY_STATUS_KIND_FOX_SPECIAL_N {
        if situation_kind == *SITUATION_KIND_GROUND && StatusModule::prev_situation_kind(boma) == *SITUATION_KIND_AIR {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_LANDING, false);
        } else if situation_kind == *SITUATION_KIND_AIR {
            if boma.is_cat_flag(Cat2::FallJump) && stick_y < -0.66 && KineticModule::get_sum_speed_y(boma, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY) <= 0.0 {
                WorkModule::set_flag(boma, true, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE);
            }
        }
    }
}

// Falco Laser Land Cancel
unsafe fn falco_laser_fastfall_landcancel(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat2: i32, stick_y: f32) {
    if status_kind == *FIGHTER_KIRBY_STATUS_KIND_FALCO_SPECIAL_N {
        if situation_kind == *SITUATION_KIND_GROUND && StatusModule::prev_situation_kind(boma) == *SITUATION_KIND_AIR {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_LANDING, false);
        } else if situation_kind == *SITUATION_KIND_AIR {
            if boma.is_cat_flag(Cat2::FallJump) && stick_y < -0.66 && KineticModule::get_sum_speed_y(boma, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY) <= 0.0 {
                WorkModule::set_flag(boma, true, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE);
            }
        }
    }
}

// Laser Airdodge Cancel
unsafe fn airdodge_cancel(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat1: i32, frame: f32) {
    if status_kind == *FIGHTER_KIRBY_STATUS_KIND_WOLF_SPECIAL_N {
        if frame > 17.0 {
            boma.check_airdodge_cancel();
        }
    }
}

// Water Shuriken Max Dash Cancel
unsafe fn max_water_shuriken_dc(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat1: i32, frame: f32) {
    if status_kind == *FIGHTER_KIRBY_STATUS_KIND_GEKKOUGA_SPECIAL_N_MAX_SHOT {
        if frame > 12.0 {
            boma.check_dash_cancel();
        }
    }
}

// Firaga Airdodge Cancel
unsafe fn magic_cancels(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat1: i32, frame: f32) {
    if status_kind == *FIGHTER_KIRBY_STATUS_KIND_TRAIL_SPECIAL_N1_SHOOT {
        if frame > 2.0 {
            boma.check_airdodge_cancel();
        }
    }
    // Thundaga Land Cancel
    if boma.is_status(*FIGHTER_KIRBY_STATUS_KIND_TRAIL_SPECIAL_N3)
    && boma.is_situation(*SITUATION_KIND_GROUND)
    && boma.is_prev_situation(*SITUATION_KIND_AIR)
    {
        let special_n_fire_cancel_frame_ground = 69.0;
        let landing_lag = 12.0;
        if MotionModule::frame(boma) < (special_n_fire_cancel_frame_ground - landing_lag) {
            MotionModule::set_frame_sync_anim_cmd(boma, special_n_fire_cancel_frame_ground - landing_lag, true, true, true);
        }
    }
}


// Bite Early Throw and Turnaround
unsafe fn bite_early_throw_turnaround(boma: &mut BattleObjectModuleAccessor, status_kind: i32, stick_x: f32, facing: f32, frame: f32) {
    if StatusModule::is_changing(boma) {
        return;
    }
    if status_kind == *FIGHTER_KIRBY_STATUS_KIND_WARIO_SPECIAL_N_BITE {
        if compare_mask(ControlModule::get_pad_flag(boma), *FIGHTER_PAD_FLAG_SPECIAL_TRIGGER) {
            boma.change_status_req(*FIGHTER_KIRBY_STATUS_KIND_WARIO_SPECIAL_N_BITE_END, false);
        }
    }
    if status_kind == *FIGHTER_KIRBY_STATUS_KIND_WARIO_SPECIAL_N_BITE_END {
        if frame < 7.0 {
            if facing * stick_x < 0.0 {
                PostureModule::reverse_lr(boma);
                PostureModule::update_rot_y_lr(boma);
            }
        }
    }
}

// Chef Land Cancel
unsafe fn ff_chef_land_cancel(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat2: i32, stick_y: f32) {
    if status_kind == *FIGHTER_KIRBY_STATUS_KIND_GAMEWATCH_SPECIAL_N {
        if situation_kind == *SITUATION_KIND_GROUND && StatusModule::prev_situation_kind(boma) == *SITUATION_KIND_AIR {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_LANDING, false);
        }
        if situation_kind == *SITUATION_KIND_AIR {
            if boma.is_cat_flag(Cat2::FallJump) && stick_y < -0.66
                && KineticModule::get_sum_speed_y(boma, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY) <= 0.0 {
                WorkModule::set_flag(boma, true, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE);
            }
        }
        if MotionModule::frame(boma) <= 1.0 {
            let nspec_halt = Vector3f{x: 0.9, y: 1.0, z: 1.0};
            KineticModule::mul_speed(boma, &nspec_halt, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        }
    }
}

unsafe fn frame_data(boma: &mut BattleObjectModuleAccessor, status_kind: i32, motion_kind: u64, frame: f32) {
    if StatusModule::is_changing(boma) {
        return;
    }
    if status_kind == *FIGHTER_KIRBY_STATUS_KIND_GAMEWATCH_SPECIAL_N {
        if frame <= 19.0 {
            MotionModule::set_rate(boma, 2.0);
        }
        if frame > 19.0 {
            MotionModule::set_rate(boma, 1.0);
        }
    }
}

// Nayru's Love Land Cancel and Fast Fall
unsafe fn nayru_fastfall_land_cancel(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat2: i32, stick_y: f32, frame: f32) {
    if status_kind == *FIGHTER_KIRBY_STATUS_KIND_ZELDA_SPECIAL_N {
        if situation_kind == *SITUATION_KIND_GROUND {
            if StatusModule::prev_situation_kind(boma) == *SITUATION_KIND_AIR && frame < 55.0 {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_LANDING, false);
                MotionModule::set_frame_sync_anim_cmd(boma, 56.0, true, true, false);
            }
        }
        else if situation_kind == *SITUATION_KIND_AIR {
            if frame >= 31.0 {
                KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_FALL);
                if boma.is_cat_flag(Cat2::FallJump) && stick_y < -0.66 && KineticModule::get_sum_speed_y(boma, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY) <= 0.0 {
                    WorkModule::set_flag(boma, true, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE);
                }
            }
        }
    }
}

pub unsafe fn moveset(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    final_cutter_landing_bugfix(fighter);
    horizontal_cutter(fighter);
    //disable_dash_attack_slideoff(fighter);
    //stone_control(fighter);
    fastfall_specials(fighter);

    // Magic Series
    magic_series(boma, id, cat, status_kind, situation_kind, motion_kind, stick_x, stick_y, facing, frame);

    // Fox Laser Land Cancel and Fast Fall
    fox_laser_fastfall_landcancel(boma, status_kind, situation_kind, cat[1], stick_y);

    // Falco Laser Land Cancel and Fast Fall
    falco_laser_fastfall_landcancel(boma, status_kind, situation_kind, cat[1], stick_y);

    // Water Shuriken Max Dash Cancel
    max_water_shuriken_dc(boma, status_kind, situation_kind, cat[0], frame);

    // Firaga and Thundaga Cancels
    magic_cancels(boma, status_kind, situation_kind, cat[0], frame);

    // Bite Early Throw and Turnaround
    bite_early_throw_turnaround(boma, status_kind, stick_x, facing, frame);

    // Chef Land Cancel
    ff_chef_land_cancel(boma, status_kind, situation_kind, cat[1], stick_y);
    frame_data(boma, status_kind, motion_kind, frame);

    // Nayru's Love Land Cancel and Fast Fall
    nayru_fastfall_land_cancel(boma, status_kind, situation_kind, cat[2], stick_y, frame);

    // Laser Airdodge Cancel
    airdodge_cancel(boma, status_kind, situation_kind, cat[0], frame);

}

#[utils::macros::opff(FIGHTER_KIND_KIRBY )]
pub fn kirby_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		kirby_frame(fighter)
    }
}

pub unsafe fn kirby_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}