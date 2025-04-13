use trpl::Html;
use std::time::Duration;

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

enum Either<A, B> {
    Left(A),
    Right(B),
}