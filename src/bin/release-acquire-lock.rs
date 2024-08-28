use std::{
    sync::atomic::AtomicBool,
    sync::atomic::Ordering::{Acquire, Relaxed, Release},
    thread,
};

static mut DATA: String = String::new();
static LOCKED: AtomicBool = AtomicBool::new(false);

fn f() {
    if LOCKED
        .compare_exchange(false, true, Acquire, Relaxed)
        .is_ok()
    {
        // 安全性: Mutexを保持しているので誰かがDATAにアクセスすることはない
        unsafe { DATA.push('!') };
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
