use std::{
    collections::HashMap,
    sync::{
        mpsc::{self, Receiver, Sender},
        Arc, Mutex,
    },
};

pub struct PubSub {
    subscribers: Arc<Mutex<HashMap<String, Vec<Sender<String>>>>>,
}

impl Default for PubSub {
    fn default() -> Self {
        Self::new()
    }
}

impl PubSub {
    pub fn new() -> Self {
        PubSub {
            subscribers: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn subscribe(&self, topic: &str) -> (Sender<String>, Receiver<String>) {
        let (tx, rx) = mpsc::channel();
        let mut subscribers = self.subscribers.lock().unwrap();
        subscribers
            .entry(topic.to_string())
            .or_default()
            .push(tx.clone());
        (tx, rx)
    }

    pub fn publish(&self, msg: &str, topic: &str) {
        let subscribers = self.subscribers.lock().unwrap();
        if let Some(rx_list) = subscribers.get(topic) {
            for rx in rx_list {
                rx.send(msg.to_string()).unwrap();
            }
        };
    }

    pub fn add_subscription(&self, tx: &Sender<String>, topic: &str) {
        let mut subscribers = self.subscribers.lock().unwrap();
        subscribers
            .entry(topic.to_string())
            .or_default()
            .push(tx.clone());
    }

    pub fn remove_subscription(&self, tx: &Sender<String>, topic: &str) {
        let mut subscribers = self.subscribers.lock().unwrap();
        if let Some(sub_list) = subscribers.get_mut(topic) {
            sub_list.retain(|subscriber| !std::ptr::eq(subscriber, tx));
            if sub_list.is_empty() {
                subscribers.remove(topic);
            }
        }
    }
}
