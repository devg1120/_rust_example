use std::{collections::HashSet, time::Duration};

use futures::{future::pending, StreamExt};
use tiny_actor::*;

#[tokio::test]
async fn spawn_and_abort() {
    let (mut child, _address) = spawn_process(Config::default(), |_: Inbox<()>| async move {
        let () = pending().await;
    });
    child.abort();
    assert!(child.await.unwrap_err().is_abort());
}

#[tokio::test]
async fn spawn_await_address() {
    let (mut child, address) = spawn_process(Config::default(), |_: Inbox<()>| async move {
        let () = pending().await;
    });
    child.abort();
    address.await;
}

#[tokio::test]
async fn spawn_and_panic() {
    let (child, _address) = spawn_process(Config::default(), |_: Inbox<()>| async move { panic!() });
    assert!(child.await.unwrap_err().is_panic());
}

#[tokio::test]
async fn spawn_and_normal_exit() {
    let (child, _address) = spawn_process(Config::default(), |_: Inbox<()>| async move {});
    assert!(child.await.is_ok());
}

#[tokio::test]
async fn spawn_and_halt() {
    let (child, _address) = spawn_process(Config::default(), |mut inbox: Inbox<()>| async move {
        assert_eq!(inbox.recv().await.unwrap_err(), RecvError::Halted);
    });
    child.halt();
    assert!(child.await.is_ok());
}

#[tokio::test]
async fn spawn_and_drop() {
    let (child, address) = spawn_process(
        Config {
            link: Link::Attached(Duration::from_millis(10)),
            capacity: Capacity::Bounded(10),
        },
        |mut inbox: Inbox<()>| async move {
            assert_eq!(inbox.recv().await.unwrap_err(), RecvError::Halted);
            let () = pending().await;
        },
    );
    drop(child);
    address.await;
}

#[tokio::test]
async fn spawn_and_drop_detached() {
    let (child, address) = spawn_process(
        Config::new(Link::Detached, Capacity::Unbounded(BackPressure::default())),
        |mut inbox: Inbox<()>| async move {
            assert_eq!(inbox.recv().await.unwrap(), ());
        },
    );
    drop(child);
    tokio::time::sleep(Duration::from_millis(10)).await;
    address.send(()).await.unwrap();
    address.await;
}

#[tokio::test]
async fn base_counts() {
    let (mut child, address) = spawn_process(Config::default(), |inbox: Inbox<()>| async move {
        pending::<()>().await;
        drop(inbox);
    });
    assert_eq!(child.address_count(), 1);
    assert_eq!(child.process_count(), 1);
    assert_eq!(address.msg_count(), 0);
    child.abort();
}

#[tokio::test]
async fn address_counts() {
    let (mut child, address) = spawn_process(Config::default(), |inbox: Inbox<()>| async move {
        pending::<()>().await;
        drop(inbox);
    });
    assert_eq!(child.address_count(), 1);
    let address2 = address.clone();
    assert_eq!(child.address_count(), 2);
    drop(address2);
    assert_eq!(child.address_count(), 1);
    child.abort();
}

#[tokio::test]
async fn inbox_counts() {
    let (pool, _address) = spawn_many_processes(
        0..4,
        Config::default(),
        |_, mut inbox: Inbox<()>| async move {
            inbox.recv().await.unwrap_err();
        },
    );
    let mut pool = pool.into_dyn();
    assert_eq!(pool.process_count(), 4);

    pool.halt_some(1);
    tokio::time::sleep(Duration::from_millis(10)).await;
    assert_eq!(pool.process_count(), 3);

    pool.try_spawn(|mut inbox: Inbox<()>| async move {
        inbox.recv().await.unwrap_err();
    })
    .unwrap();
    assert_eq!(pool.process_count(), 4);

    pool.halt_some(2);
    tokio::time::sleep(Duration::from_millis(10)).await;
    assert_eq!(pool.process_count(), 2);

    pool.halt();
    tokio::time::sleep(Duration::from_millis(10)).await;
    assert_eq!(pool.process_count(), 0);
}

#[tokio::test]
async fn pooled_messaging_split() {
    let (pool, address) = spawn_many_processes(0..3, Config::bounded(5), |_, mut inbox| async move {
        let mut numbers = Vec::new();
        loop {
            match inbox.recv().await {
                Ok(msg) => {
                    tokio::time::sleep(Duration::from_millis(1)).await;
                    numbers.push(msg);
                }
                Err(signal) => match signal {
                    RecvError::Halted => break Ok(numbers),
                    RecvError::ClosedAndEmpty => break Err(()),
                },
            }
        }
    });

    for i in 0..30 {
        address.send(i).await.unwrap();
    }

    tokio::time::sleep(Duration::from_millis(20)).await;
    address.halt();

    let res = pool
        .map(|e| e.unwrap().unwrap())
        .fold(HashSet::new(), |mut acc, vals| async move {
            assert!(vals.len() >= 9);
            assert!(vals.len() <= 11);
            for val in vals {
                assert!(acc.insert(val));
            }
            acc
        })
        .await;
    assert_eq!(res.len(), 30)
}
