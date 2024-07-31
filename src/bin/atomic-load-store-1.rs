use std::{
    io,
    sync::atomic::{AtomicBool, Ordering::Relaxed},
    thread,
};
fn main() {
    static STOP: AtomicBool = AtomicBool::new(false);

    // スレッドを起動
    let background_thread: thread::JoinHandle<()> = thread::spawn(|| {
        while !STOP.load(Relaxed) {
            some_work();
        }
    });

    // メインスレッドを使って入力を受け付ける
    for line in io::stdin().lines() {
        match line.unwrap().as_str() {
            "help" => println!("commands: help, stop"),
            "stop" => break,
            cmd => println!("unknown command: {}", cmd),
        }
    }

    // バックグラウンドスレッドを停止
    STOP.store(true, Relaxed);

    // バックグラウンドスレッドの終了を待つ
    background_thread.join().unwrap();
}

fn some_work() {
    println!("doing some work...");
    thread::sleep(std::time::Duration::from_secs(5));
}
