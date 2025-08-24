use bytes::Bytes;
use mini_redis::client;
use tokio::sync::{mpsc, oneshot};

type Responder<T> = oneshot::Sender<mini_redis::Result<T>>;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(32);
    let tx2 = tx.clone();

    let handle01 = tokio::spawn(async move {
        let (resp_tx, resp_rx) = oneshot::channel();

        let cmd = Command::Get {
            key: "foo".to_string(),
            resp_tx,
        };

        tx.send(cmd).await.unwrap();

        let result = resp_rx.await;
        println!("GOT: {:?}", result)
    });

    let handle02 = tokio::spawn(async move {
        let (resp_tx, resp_rx) = oneshot::channel();

        let cmd = Command::Set {
            key: "foo".to_string(),
            val: "bar".into(),
            resp_tx,
        };

        tx2.send(cmd).await.unwrap();

        let result = resp_rx.await;
        println!("GOT: {:?}", result)
    });

    let manager = tokio::spawn(async move {
        let mut client = client::connect("127.0.0.1:6379").await.unwrap();

        while let Some(cmd) = rx.recv().await {
            use Command::*;

            match cmd {
                Get { key, resp_tx } => {
                    let res = client.get(&key).await;

                    let _ = resp_tx.send(res);
                }
                Set { key, val, resp_tx } => {
                    let res = client.set(&key, val).await;

                    let _ = resp_tx.send(res);
                }
            }
        }
    });

    handle01.await.unwrap();
    handle02.await.unwrap();
    manager.await.unwrap();
}

#[derive(Debug)]
enum Command {
    Get {
        key: String,
        resp_tx: Responder<Option<Bytes>>,
    },
    Set {
        key: String,
        val: Bytes,
        resp_tx: Responder<()>,
    },
}
