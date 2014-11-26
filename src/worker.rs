use std::collections::hash_map::{mod, HashMap};
use std::sync::{Arc, Mutex};

pub struct Worker {
    queue: Arc<Mutex<Queue>>,
}

// The key is a crate name, and the value is a list of senders that must be notified
// when the crate has finished being built.
type Queue = HashMap<String, Vec<Sender<()>>>;

impl Worker {
    pub fn new(tmp_dir: Path) -> Worker {
        let queue = Arc::new(Mutex::new(HashMap::new()));

        let queue2 = queue.clone();
        spawn(proc() { background_thread(queue2, tmp_dir); });

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

fn background_thread(queue: Arc<Mutex<Queue>>, tmp_dir: Path) {
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

        generate_docs(next_crate[], &tmp_dir);

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

fn generate_docs(crate_name: &str, tmp_dir: &Path) {
    use cargo;
    use std::io;
    use std::io::util::NullWriter;
    use std::io::fs::{mod, File};
    use std::os;

    fs::mkdir(tmp_dir, io::USER_RWX).ok();      // ignoring error if already exists

    let mut manifest = File::create(&tmp_dir.join("Cargo.toml")).unwrap();
    (writeln!(&mut manifest, r#"[package]"#)).unwrap();
    (writeln!(&mut manifest, r#"name = "autocrate_tmp""#)).unwrap();
    (writeln!(&mut manifest, r#"version = "0.0.1""#)).unwrap();
    (writeln!(&mut manifest, r#"authors = []"#)).unwrap();
    (writeln!(&mut manifest, r#""#)).unwrap();
    (writeln!(&mut manifest, r#"[dependencies]"#)).unwrap();
    (writeln!(&mut manifest, r#"{} = "*""#, crate_name)).unwrap();

    fs::mkdir(&tmp_dir.join("src"), io::USER_RWX).ok();      // ignoring error if already exists
    File::create(&tmp_dir.join("src").join("lib.rs")).unwrap();

    // building the cargo shell
    let out = cargo::core::shell::Shell::create(box NullWriter, cargo::core::shell::ShellConfig {
        color: false,
        verbose: true,
        tty: true,
    });

    let err = cargo::core::shell::Shell::create(box NullWriter, cargo::core::shell::ShellConfig {
        color: false,
        verbose: true,
        tty: true,
    });

    let mut multishell = cargo::core::shell::MultiShell::new(out, err, true);

    // generating docs
    let mut doc_opts = cargo::ops::DocOptions {
        all: true,
        open_result: false,
        compile_opts: cargo::ops::CompileOptions {
            env: "doc-all",
            shell: &mut multishell,
            jobs: Some(1),
            target: None,
            dev_deps: false,
            features: &[],      // TODO: all features should be enabled
            no_default_features: false,
            spec: None,
            lib_only: false,
        },
    };

    cargo::ops::doc(&os::make_absolute(tmp_dir).unwrap(), &mut doc_opts).unwrap();
}
