use std::collections::hash_map::{mod, HashMap};
use std::sync::{Arc, Mutex};

pub struct Worker {
    queue: Arc<Mutex<Queue>>,
}

// The key is a crate name, and the value is a list of senders that must be notified
// when the crate has finished being built.
type Queue = HashMap<String, Vec<Sender<()>>>;

impl Worker {
    pub fn new() -> Worker {
        let queue = Arc::new(Mutex::new(HashMap::new()));

        let queue2 = queue.clone();
        spawn(proc() { background_thread(queue2); });

        Worker {
            queue: queue,
        }
    }

    pub fn submit(&self, crate_name: &str) -> Receiver<()> {
        let (tx, rx) = channel();

        let mut packages = self.queue.lock();
        match packages.entry(crate_name.to_string()) {
            hash_map::Entry::Occupied(mut entry) => {
                entry.get_mut().push(tx);
            },
            hash_map::Entry::Vacant(entry) => {
                entry.set(vec![tx]);
            },
        }

        packages.cond.signal();

        rx
    }
}

fn background_thread(queue: Arc<Mutex<Queue>>) {
    loop {
        // determining next crate to process
        // TODO: since we use a hashmap, packages don't keep the order in which they are added
        let next_crate = {
            let queue = queue.lock();
            let val;
            loop {
                if let Some(c) = queue.keys().next() {
                    val = c.clone();
                    break;
                } else {
                    queue.cond.wait();
                    continue;
                }
            }
            val
        };

        generate_docs(next_crate[]);

        // removing crate from queue and notifying
        {
            let mut queue = queue.lock();
            let queue = queue.remove(&next_crate).unwrap();
            for s in queue.into_iter() {
                s.send_opt(()).ok();
            }
        }
    }
}

fn generate_docs(crate_name: &str) {

}
