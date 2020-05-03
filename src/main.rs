use std::fs::File;

use daemonize::Daemonize;
use notify::{watcher, DebouncedEvent, RecursiveMode, Watcher};
use std::path::Path;
use std::sync::mpsc::{channel, Receiver};
use std::time::Duration;

fn main() {
    let stdout = File::create("/tmp/books-daemon.out").unwrap();
    let stderr = File::create("/tmp/books-daemon.err").unwrap();

    let daemonize = Daemonize::new()
        .pid_file("/tmp/books-daemon.pid")
        .working_directory("/tmp/test")
        .stdout(stdout)
        .stderr(stderr)
        .exit_action(|| println!("Executed before master process exits"))
        .privileged_action(|| "Executed before drop privileges");

    match daemonize.start() {
        Ok(_) => watch_for_added_books(),
        Err(e) => eprintln!("Error, {}", e),
    }
}

fn watch_for_added_books() {
    println!("Success, daemonized");
    let (sender, receiver) = channel();

    let mut watcher = watcher(sender, Duration::from_secs(2)).unwrap();
    watcher
        .watch("/tmp/test", RecursiveMode::Recursive)
        .unwrap();

    handle_file_events(receiver);
}

fn handle_file_events(rx: Receiver<DebouncedEvent>) {
    loop {
        match rx.recv() {
            Ok(DebouncedEvent::Create(p)) => process_new_book_event(p),
            Ok(_) => println!("different event"),
            Err(e) => eprint!("watch error: {:?}", e),
        }
    }
}

fn process_new_book_event<P: AsRef<Path>>(p: P) {
    println!("new file created: {:?}", p.as_ref());
}
