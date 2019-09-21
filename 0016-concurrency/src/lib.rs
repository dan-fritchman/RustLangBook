use std::thread;
use std::time::Duration;
use std::sync::mpsc;
use std::sync::Mutex;
use std::sync::Arc;


pub fn run() {
    println!("!!!");
    th();
    ch();
    sh();
}

fn sh() {
    //! Share Memory
    let m = Mutex::new(5);
    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }
    println!("m = {:?}", m);

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!("Result: {}", *counter.lock().unwrap());
}

fn ch() {
    let (tx, rx) = mpsc::channel();

    let tx1 = mpsc::Sender::clone(&tx);
    thread::spawn(move || {
        let val = String::from("hi");
        tx1.send(val).unwrap();
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        for v in vals {
            tx1.send(v).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
        let vals = vec![
            String::from("more"),
            String::from("msgs"),
            String::from("for"),
            String::from("you"),
        ];
        for v in vals {
            tx.send(v).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}

fn th() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi #{} from sub-thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    for i in 1..5 {
        println!("hi #{} from main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
    handle.join().unwrap();
}
