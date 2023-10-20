use super::*;
use globals::*;

// This file contains code for pummels

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            status_CatchAttack_Main
        );
    }
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_CatchAttack_Main)]
unsafe fn status_CatchAttack_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    // Your pummel FAF is equal to your animation's length, with a cap at this param's number of frames
    let pummel_max_cancel_frame = ParamModule::get_int(fighter.object(), ParamType::Common, "pummel_max_cancel_frame") as f32;
    if fighter.global_table[CURRENT_FRAME].get_i32() as f32 + 1.0 >= MotionModule::end_frame(fighter.module_accessor).min(pummel_max_cancel_frame)
    || CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.CatchCont().get_bool() {
            return 0.into();
        }
    }
    if fighter.global_table[SITUATION_KIND] != SITUATION_KIND_AIR {
        if MotionModule::is_end(fighter.module_accessor) { 
            fighter.change_status_req(*FIGHTER_STATUS_KIND_CATCH_WAIT, false);
        }
    }
    else {
        fighter.change_status_req(*FIGHTER_STATUS_KIND_CATCH_JUMP, false);
    }
    0.into()
}