use std::{
    sync::atomic::{AtomicUsize, Ordering::Relaxed},
    thread,
    time::Duration,
};
fn main() {
    let num_done = AtomicUsize::new(0);

    thread::scope(|s| {
        // 1つのバックグラウンドスレッドで100個のアイテムをすべて処理する
        s.spawn(|| {
            for i in 0..100 {
                process_item(i);
                num_done.store(i + 1, Relaxed);
            }
        });

        // メインスレッドは、毎秒アイテムの処理状況を表示する
        loop {
            let n = num_done.load(Relaxed);
            if n == 100 {
                break;
            }
            println!("Working.. {n}/100 done");
            thread::sleep(Duration::from_secs(1));
        }
    });

    println!("All done!");
}

fn process_item(i: usize) {
    println!("Processing item {}", i);
    thread::sleep(Duration::from_secs(1));
}
