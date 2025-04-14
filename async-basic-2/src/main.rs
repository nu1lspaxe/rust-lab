use std::{pin::pin, time::Duration};
use trpl::{ReceiverStream, Stream, StreamExt};

fn main() {
    trpl::run(async {
        let values = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let iter = values.iter().map(|n| n * 2);
        let mut stream = trpl::stream_from_iter(iter);

        while let Some(value) = stream.next().await {
            println!("{}", value);
        }
    });

    println!("=====================");
    trpl::run(async {
        let values = 1..101;
        let iter = values.map(|n| n * 2);
        let stream = trpl::stream_from_iter(iter);

        let mut filtered = 
            stream.filter(|v| v % 3 == 0 || v % 5 == 0);

        while let Some(value) = filtered.next().await {
            println!("{}", value);
        }
    });

    println!("=====================");
    trpl::run(async {
        let mut messages = get_message();
        
        while let Some(message) = messages.next().await {
            println!("{message}");
        }
    });

    println!("=====================");
    trpl::run(async {
        let messages = get_message().timeout(Duration::from_secs(1));
        let intervals = get_intervals()
            .map(|coutn| format!("Interval: {}", coutn))
            .timeout(Duration::from_secs(1));

        let merged = messages.merge(intervals).take(20);
        let mut stream = pin!(merged);

        while let Some(result) = stream.next().await {
            match result {
                Ok(message) => println!("{message}"),
                Err(e) => println!("Error: {e}"),
            }
        }
    })
}

fn get_message() -> impl Stream<Item = String> {
    let (tx, rx) = trpl::channel();

    let messages = ["Hello", "World", "from", "Rust"];
    for message in messages {
        tx.send(format!("Message: {}", message)).unwrap();
    }

    ReceiverStream::new(rx)
}

fn get_intervals() -> impl Stream<Item = u32> {
    let (tx, rx) = trpl::channel();

    trpl::spawn_task(async move {
        let mut count = 0;
        loop {
            trpl::sleep(Duration::from_millis(1)).await;
            count += 1;
            tx.send(count).unwrap();
        }
    });

    ReceiverStream::new(rx)
}