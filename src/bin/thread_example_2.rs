use std::{sync::mpsc, thread, time::Duration}; //Multi Producer, Single Consumer

fn main() {
    let (sender, receiever) = mpsc::channel();
    let sender_2 = sender.clone();

    let _join_handler = thread::spawn(move || {
        let messages = vec![
            "Sender1:Hi",
            "Sender1:Satya!",
            "Sender1:How",
            "Sender1:Are",
            "Sender1:You?",
        ];
        for message in messages {
            sender.send(message).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    let _join_handler = thread::spawn(move || {
        let messages = vec![
            "Sender2:Hi",
            "Sender2:Satya!",
            "Sender2:How",
            "Sender2:Are",
            "Sender2:You?",
        ];
        for message in messages {
            sender_2.send(message).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for message in receiever {
        println!("{message}");
    }
}
