// threads3.rs
//
// Execute `rustlings hint threads3` or use the `hint` watch subcommand for a
// hint.

use std::sync::mpsc;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

struct Queue {
    length: u32,
    first_half: Vec<u32>,
    second_half: Vec<u32>,
}

impl Queue {
    fn new() -> Self {
        Queue {
            length: 10,
            first_half: vec![1, 2, 3, 4, 5],
            second_half: vec![6, 7, 8, 9, 10],
        }
    }
}

fn send_tx(q: Queue, tx: mpsc::Sender<u32>) -> () {
    let txs = Arc::new(tx);
    {
        let tx = Arc::clone(&txs);
    thread::spawn(move || {
        // Note the partial move of the q first_half
        for val in q.first_half {
            println!("sending {:?}", val);
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    }

    {
        let tx = Arc::clone(&txs);
    thread::spawn(move || {
        // Partial move of the q second_half
        for val in q.second_half {
            println!("sending {:?}", val);
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
}
}

#[test]
fn main() {
    let (tx, rx) = mpsc::channel();
    let queue = Queue::new();
    let queue_length = queue.length;

    send_tx(queue, tx);

    let mut total_received: u32 = 0;
    for received in rx {
        println!("Got: {}", received);
        total_received += 1;
    }

    println!("total numbers received: {}", total_received);
    assert_eq!(total_received, queue_length)
}
