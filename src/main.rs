use std::fs::File;

use crate::dispatcher::FsEventDispatcher;
use crate::processor::NewBookEventProcessor;
use crate::provider::DebouncedEventAdapter;
use daemonize::Daemonize;
use notify::{watcher, RecursiveMode, Watcher};
use std::sync::mpsc::channel;
use std::time::Duration;

mod dispatcher;
mod processor;
mod provider;

fn main() {
    let stdout = File::create("/tmp/books-daemon.out").unwrap();
    let stderr = File::create("/tmp/books-daemon.err").unwrap();

    let daemonize = Daemonize::new()
        .pid_file("/tmp/books-daemon.pid")
        .working_directory("/home/zbychu/books")
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
        .watch("/home/zbychu/books", RecursiveMode::Recursive)
        .unwrap();

    FsEventDispatcher::new(
        &DebouncedEventAdapter::new(receiver),
        &NewBookEventProcessor::new(),
    )
    .handle()
    .expect("failed to dispatch events");
}
