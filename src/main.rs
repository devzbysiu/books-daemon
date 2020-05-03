use std::fs::File;

use crate::handler::FsEventHandler;
use crate::processor::NewBookEventProcessor;
use daemonize::Daemonize;
use notify::{watcher, DebouncedEvent, RecursiveMode, Watcher};
use std::path::Path;
use std::sync::mpsc::{channel, Receiver};
use std::time::Duration;

mod handler;
mod processor;

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

    FsEventHandler::new(receiver, &NewBookEventProcessor::new()).handle();
}
