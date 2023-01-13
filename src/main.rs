use std::thread;
use std::time::Duration;

fn main() {
    let v = vec![1, 2, 3];
    let v2 = vec![4, 5, 6];

    // move で所有権を強制的に奪わせる
    let handle = thread::spawn(move || {
        println!("メインスレッドのベクタ {:?}", v);
    });

    drop(v2);

    handle.join().unwrap();
}
