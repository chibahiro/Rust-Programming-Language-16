use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (sender, receiver) = mpsc::channel();

    let sender_cloned = mpsc::Sender::clone(&sender);
    thread::spawn(move || {
        let vals = vec![
            String::from("ハイ"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            sender_cloned.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    
    
   thread::spawn(move || {
        let vals = vec![
            String::from("さらなる"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            sender.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    
    
    // recv() を呼ばなくても待機して取得してくれる
    for received in receiver {
        println!("取得： {}", received);
    }
}

