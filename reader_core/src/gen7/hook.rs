use crate::{pnp, utils};
use chrono::NaiveDateTime;
static mut MAIN_RNG_SEED_TICKS: u32 = 0;
static mut MAIN_RNG_DATE_TIME: NaiveDateTime = NaiveDateTime::MIN;
static mut MAIN_RNG_TIME_OFFSET_MS: u64 = 0;
static mut SOS_SEED: u32 = 0;

pub fn sos_seed() -> u32 {
    unsafe { SOS_SEED }
}

fn init_sfmt_hook(regs: &[u32], _stack_pointer: *mut u32) {
    if regs[0] == 0x30038e30 {
        unsafe { SOS_SEED = regs[1] };
    }
}

pub struct RngSeedContext {
    pub init_datetime: NaiveDateTime,
    pub time_offset_ms: u64,
    pub ticks: u32,
}

pub fn main_rng_seed_context() -> RngSeedContext {
    unsafe {
        RngSeedContext {
            init_datetime: MAIN_RNG_DATE_TIME,
            time_offset_ms: MAIN_RNG_TIME_OFFSET_MS,
            ticks: MAIN_RNG_SEED_TICKS,
        }
    }
}

fn init_main_rng_hook(_regs: &[u32], stack_pointer: *mut u32) {
    let ticks = unsafe { stack_pointer.add(4).read() };
    let console_ms_epoch_low = unsafe { stack_pointer.add(12).read() };
    let console_ms_epoch_high = unsafe { stack_pointer.add(13).read() };
    let console_ms_epoch = ((console_ms_epoch_high as u64) << 32) | (console_ms_epoch_low as u64);
    let current_time = pnp::datetime_from_console_ms(console_ms_epoch);
    // Assume we didn't start with any milliseconds and any extra milliseconds was spent loading
    let start_ms = (pnp::game_start_ms() / 1000) * 1000;
    let time_offset_ms = console_ms_epoch.saturating_sub(start_ms);

    unsafe {
        MAIN_RNG_SEED_TICKS = ticks;
        MAIN_RNG_DATE_TIME = current_time;
        MAIN_RNG_TIME_OFFSET_MS = time_offset_ms;
    }
}

pub fn init_um() {
    utils::hook_game_branch! {
        hook_router_addr = 0x5bbfdc,
        init_sfmt_hook = 0x361e60,
        init_main_rng_hook = 0x3fcbc0,
    }
}

pub fn init_us() {
    utils::hook_game_branch! {
        hook_router_addr = 0x5bbfdc,
        init_sfmt_hook = 0x361e60,
        init_main_rng_hook = 0x3fcbbc,
    }
}

pub fn init_sm() {
    utils::hook_game_branch! {
        hook_router_addr = 0x597fdc,
        init_sfmt_hook = 0x359784,
        init_main_rng_hook = 0x3eab60,
    }
}
