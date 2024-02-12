use actix::prelude::*;

/// Define `Ping` message
struct Ping(usize);

impl Message for Ping {
    type Result = usize;
}

/// Actor
struct MyActor {
    count: usize,
}

impl Actor for MyActor {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        ctx.set_mailbox_capacity(10);
    }
}

//let addr = MyActor.start();

impl Handler<Ping> for MyActor {
    type Result = usize;

    fn handle(&mut self, msg: Ping, ctx: &mut Context<Self>) -> Self::Result {
        self.count += msg.0;
        println!("recv");
        if self.count > 5 {
            println!("Shutting down ping receiver.");
            ctx.stop()
        }

        self.count
    }
}

#[actix_rt::main]
async fn main() {
    // start new actor
    let addr = MyActor { count: 0 }.start();

    // send message and get future for result
    let addr_2 = addr.clone();
    let _res = addr.send(Ping(1)).await;
    let _res = addr.send(Ping(1)).await;
    let _res = addr.send(Ping(1)).await;
    let _res = addr.send(Ping(1)).await;
    let _res = addr.send(Ping(1)).await;
    let _res = addr.send(Ping(1)).await;
    let _res = addr.send(Ping(1)).await;
    let _res = addr.send(Ping(1)).await;
    let res = addr.send(Ping(1)).await;

    match res {
        Ok(_) => assert!(addr_2.try_send(Ping(6)).is_err()),
        _ => {}
    }
}



/*
#[derive(Message)]
#[rtype(result = "()")]
struct OrderShipped(usize);

#[derive(Message)]
#[rtype(result = "()")]
struct Ship(usize);

/// Subscribe to order shipped event.
#[derive(Message)]
#[rtype(result = "()")]
struct Subscribe(pub Recipient<OrderShipped>);

/// Actor that provides order shipped event subscriptions
struct OrderEvents {
    subscribers: Vec<Recipient<OrderShipped>>,
}

impl OrderEvents {
    fn new() -> Self {
        OrderEvents {
            subscribers: vec![]
        }
    }
}

impl Actor for OrderEvents {
    type Context = Context<Self>;
}

impl OrderEvents {
    /// Send event to all subscribers
    fn notify(&mut self, order_id: usize) {
        for subscr in &self.subscribers {
           let _ = subscr.do_send(OrderShipped(order_id));
        }
    }
}

/// Subscribe to shipment event
impl Handler<Subscribe> for OrderEvents {
    type Result = ();

    fn handle(&mut self, msg: Subscribe, _: &mut Self::Context) {
        self.subscribers.push(msg.0);
    }
}

/// Subscribe to ship message
impl Handler<Ship> for OrderEvents {
    type Result = ();
    fn handle(&mut self, msg: Ship, _ctx: &mut Self::Context) -> Self::Result {
        self.notify(msg.0);
        System::current().stop();
    }
}

/// Email Subscriber
struct EmailSubscriber;
impl Actor for EmailSubscriber {
    type Context = Context<Self>;
}

impl Handler<OrderShipped> for EmailSubscriber {
    type Result = ();
    fn handle(&mut self, msg: OrderShipped, _ctx: &mut Self::Context) -> Self::Result {
        println!("Email sent for order {}", msg.0)
    }

}
struct SmsSubscriber;
impl Actor for SmsSubscriber {
    type Context = Context<Self>;
}

impl Handler<OrderShipped> for SmsSubscriber {
    type Result = ();
    fn handle(&mut self, msg: OrderShipped, _ctx: &mut Self::Context) -> Self::Result {
        println!("SMS sent for order {}", msg.0)
    }

}

#[actix_rt::main]
async fn main() -> Result<(), actix::MailboxError> {
    let email_subscriber = Subscribe(EmailSubscriber {}.start().recipient());
    let sms_subscriber = Subscribe(SmsSubscriber {}.start().recipient());
    let order_event = OrderEvents::new().start();
    order_event.send(email_subscriber).await?;
    order_event.send(sms_subscriber).await?;
    order_evenimpl Handler<Ping> for MyActor {
    type Result = usize;

    fn handle(&mut self, msg: Ping, ctx: &mut Context<Self>) -> Self::Result {
        self.count += msg.0;

        if self.count > 5 {
            println!("Shutting down ping receiver.");
            ctx.stop()
        }

        self.count
    }
}

#[actix_rt::main]
async fn main() {
    // start new actor
    let addr = MyActor { count: 10 }.start();

    // send message and get future for result
    let addr_2 = addr.clone();
    let res = addr.send(Ping(6)).await;

    match res {
        Ok(_) => assert!(addr_2.try_send(Ping(6)).is_err()),
        _ => {}
    }
}t.send(Ship(1)).await?;
    Ok(())
}
*/
