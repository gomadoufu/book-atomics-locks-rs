use std::{
    sync::atomic::AtomicBool,
    sync::atomic::Ordering::{Acquire, Relaxed, Release},
    thread,
};

static mut DATA: String = String::new();
static LOCKED: AtomicBool = AtomicBool::new(false);

fn f() {
    // Acquire lock
    // 同スレッド内なので、次のReleaseよりも必ず先行発生する
    if LOCKED
        .compare_exchange(false, true, Acquire, Relaxed)
        .is_ok()
    {
        unsafe { DATA.push('!') };
        // Release lock
        // Release/Acquireメモリオーダリングにより、次のAcquireよりも必ず先行発生する
        LOCKED.store(false, Release);
    }
}

fn main() {
    thread::scope(|s| {
        for _ in 0..100 {
            s.spawn(f);
        }
    })
}
