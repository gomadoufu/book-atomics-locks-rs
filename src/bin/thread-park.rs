use std::{collections::VecDeque, sync::Mutex, thread, time::Duration};

fn main() {
    let queue: Mutex<VecDeque<i32>> = Mutex::new(VecDeque::new());

    thread::scope(|s| {
        // 消費スレッド
        let t: thread::ScopedJoinHandle<_> = s.spawn(|| loop {
            let item: Option<i32> = queue.lock().unwrap().pop_front();
            if let Some(item) = item {
                dbg!(item);
            } else {
                thread::park();
            }
        });

        // 生成スレッド
        for i in 0.. {
            queue.lock().unwrap().push_back(i);
            t.thread().unpark();
            thread::sleep(Duration::from_secs(1));
        }
    });
}
