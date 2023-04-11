use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        // send() takes ownership of val, receiver gets ownership
        tx.send(val).unwrap();
        // println!("val is {}", val);
    });

    // recv blocks main thread and waits for a message to be received
    // when the transmitter closes recv() will return Err
    //
    // try_recv() does not block, returns a Result<T,E> immediately
    let received = rx.recv().unwrap();

    println!("Got: {}", received);


    // Raul Rosas - 18 years old, twink, pro fighter, in the UFC and 
    // hes been beating people who are like 25 years old and 
    // he faced off against this guy who is from MKE. He trains at a gym that Austins gym is
    // affiliated with. Austin will meet this guy
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),

        ];

        for val in vals {
            tx1.send(val).unwrap();
            std::thread::sleep(Duration::from_secs(1));
            
        }
    });


    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}
