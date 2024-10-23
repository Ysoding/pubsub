use std::time::Duration;

use pubsub::*;

fn main() {
    let p = PubSub::new();

    let (tx1, rx1) = p.subscribe("topic1");
    let (_, rx2) = p.subscribe("topic2");

    p.publish("test1", "topic1");
    p.publish("test2", "topic2");

    println!("got {:?}", rx1.recv().unwrap());
    println!("got {:?}", rx2.recv().unwrap());

    p.add_subscription(&tx1, "topic2");
    p.publish("test3", "topic2");

    println!("got {:?}", rx1.recv().unwrap());

    p.remove_subscription(&tx1, "topic2");

    let result = match rx1.recv_timeout(Duration::from_secs(1)) {
        Ok(val) => format!("should no notification, got {:?}", val),
        Err(_) => "no notification".to_string(),
    };

    println!("result: {:?}", result);
}
