use trpl::{Html, Either};
use std::{
    pin::{pin, Pin},
    future::Future,
    time::{Duration, Instant},
};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    // trpl::run(async {
    //     let url = &args[1];
    //     match page_title(url).await {
    //         Some(title) => println!("Title: {}", title),
    //         None => println!("Failed to fetch the title."),
    //     }
    // })
    
    // trpl::run(async {
    //     let title_fut_1 = page_title2(&args[1]);
    //     let title_fut_2 = page_title2(&args[2]);

    //     let (url, maybe_title) = 
    //         match trpl::race(title_fut_1, title_fut_2).await {
    //             Either::Left((url, maybe_title)) => (url, maybe_title),
    //             Either::Right((url, maybe_title)) => (url, maybe_title),
    //         };

    //     println!("URL: {}", url);
    //     match maybe_title {
    //         Some(title) => println!("Title: {}", title),
    //         None => println!("Failed to fetch the title."),
    //     }
    // })

    trpl::run(async {
        trpl::spawn_task(async {
            for i in 0..10 {
                println!("hi number {i} from the first task");
                trpl::sleep(Duration::from_millis(100)).await;
            }
        });

        for i in 1..5 {
            println!("hi number {i} from the second task");
            trpl::sleep(Duration::from_millis(200)).await;
        }
    });

    println!("-----------------");
    trpl::run(async {
        let handle = trpl::spawn_task(async {
            for i in 0..10 {
                println!("hi number {i} from the first task");
                trpl::sleep(Duration::from_millis(100)).await;
            }
        });
    
        for i in 1..5 {
            println!("hi number {i} from the second task");
            trpl::sleep(Duration::from_millis(200)).await;
        }
    
        handle.await.unwrap();
    });


    println!("-----------------");
    trpl::run(async {
        let fut1 = async {
            for i in 0..10 {
                println!("hi number {i} from the first task");
                trpl::sleep(Duration::from_millis(100)).await;
            }
        };

        let fut2 = async {
            for i in 1..5 {
                println!("hi number {i} from the second task");
                trpl::sleep(Duration::from_millis(200)).await;
            }
        };

        trpl::join(fut1, fut2).await;
    });

    println!("-----------------");
    trpl::run(async {
        let (tx, mut rx) = trpl::channel();

        let val = String::from("hello");
        tx.send(val).unwrap();

        let received = rx.recv().await.unwrap();
        println!("Received: {}", received);
    });

    println!("-----------------");
    trpl::run(async {
        let (tx, mut rx) = trpl::channel();

        let tx_fut = async move {
            let vals = vec!["hello", "world", "from", "trpl"];

            for val in vals {
                tx.send(val).unwrap();
                trpl::sleep(Duration::from_millis(100)).await;
            }
        };

        let rx_fut = async {
            while let Some(value) = rx.recv().await {
                println!("Received: {}", value);
            }
        };

        trpl::join(tx_fut, rx_fut).await;
    });

    println!("-----------------");
    trpl::run(async {
        let (tx, mut rx) = trpl::channel();

        let tx1 = tx.clone();
        let tx1_fut = pin!(async move {
            let vals = vec!["hello", "world", "from", "trpl"];

            for val in vals {
                tx1.send(val).unwrap();
                trpl::sleep(Duration::from_millis(100)).await;
            }
        });

        let rx_fut = pin!(async {
            while let Some(value) = rx.recv().await {
                println!("Received: {}", value);
            }
        });

        let tx_fut = pin!(async move {
            let vals = vec!["hello", "world", "from", "trpl"];

            for val in vals {
                tx.send(val).unwrap();
                trpl::sleep(Duration::from_millis(100)).await;
            }
        });

        let futures: Vec<Pin<Box<dyn Future<Output = ()>>>> = 
            vec![Box::pin(tx1_fut), Box::pin(rx_fut), Box::pin(tx_fut)];

        // trpl::join_all for same type
        trpl::join_all(futures).await;
    });

    println!("-----------------");
    trpl::run(async {
        let a = async { 1u32 };
        let b = async { "Hello, world!" };
        let c = async { true };

        // trpl::join! for different types
        let (a_res, b_res, c_res) = trpl::join!(a, b, c);
        println!("a: {}, b: {}, c: {}", a_res, b_res, c_res);
    });

    println!("-----------------");
    trpl::run(async {
        let slow = async {
            trpl::sleep(Duration::from_secs(2)).await;
            println!("slow");
        };

        let fast = async {
            println!("fast");
        };

        trpl::race(slow, fast).await;
    });

    println!("-----------------");
    trpl::run(async {
        let one_ns = Duration::from_nanos(1);
        let start = Instant::now();
        async {
            for _ in 1..1000 {
                trpl::sleep(one_ns).await;
            }
        }.await;
        let time = Instant::now() - start;
        println!("Time taken: {:?}", time.as_secs_f32());

        let start = Instant::now();
        async {
            for _ in 1..1000 {
                trpl::yield_now().await;
            }
        }.await;
        let time = Instant::now() - start;
        println!("Time taken: {:?}", time.as_secs_f32());
    });

    println!("-----------------");
    trpl::run(async {
        let slow = async {
            trpl::sleep(Duration::from_secs(2)).await;
            println!("First task finished");
        };

        match timeout(slow, Duration::from_secs(1)).await {
            Ok(message) => println!("Task finished: {:?}", message),
            Err(duration) => println!("Task timed out after {:?}", duration.as_secs()),
        }
    });
}

async fn timeout<F: Future>(
    future_to_try: F,
    max_time: Duration,
) -> Result<F::Output, Duration> {
    match trpl::race(future_to_try, trpl::sleep(max_time)).await {
        Either::Left(result) => Ok(result),
        Either::Right(_) => Err(max_time),
    }
}

async fn page_title(url: &str) -> Option<String> {
    let res_text = trpl::get(url).await.text().await;
    Html::parse(&res_text)
        .select_first("title")
        .map(|title| title.inner_html())
}

async fn page_title2(url: &str) -> (&str, Option<String>) {
    let text = trpl::get(url).await.text().await;
    let title = Html::parse(&text)
        .select_first("title")
        .map(|title| title.inner_html());
    (url, title)
}

enum PageTitleFuture<'a> {
    Initial { url: &'a str },
    GetAwaitPoint { url: &'a str },
    TextAwaitPoint { response: trpl::Response },
}

// enum Either<A, B> {
//     Left(A),
//     Right(B),
// }