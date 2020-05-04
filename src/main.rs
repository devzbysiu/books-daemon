use std::fs::File;

use crate::dispatcher::FsEventDispatcher;
use crate::processor::NewBookEventProcessor;
use crate::provider::DebouncedEventAdapter;
use crate::settings::Settings;
use daemonize::Daemonize;
use notify::{watcher, RecursiveMode, Watcher};
use std::sync::mpsc::channel;
use std::time::Duration;

mod dispatcher;
mod processor;
mod provider;
mod settings;

fn main() {
    let stdout = File::create("/tmp/books-daemon.out").unwrap();
    let stderr = File::create("/tmp/books-daemon.err").unwrap();
    let settings = Settings::load().expect("failed to load settings");

    let daemonize = Daemonize::new()
        .pid_file("/tmp/books-daemon.pid")
        .working_directory(settings.books_dir())
        .stdout(stdout)
        .stderr(stderr)
        .exit_action(|| println!("Executed before master process exits"))
        .privileged_action(|| "Executed before drop privileges");

    match daemonize.start() {
        Ok(_) => watch_for_added_books(&settings),
        Err(e) => eprintln!("Error, {}", e),
    }
}

fn watch_for_added_books(settings: &Settings) {
    println!("Success, daemonized");
    let (sender, receiver) = channel();

    let mut watcher = watcher(sender, Duration::from_secs(settings.interval()))
        .expect("failed to create watcher");
    watcher
        .watch(settings.books_dir(), RecursiveMode::Recursive)
        .unwrap();

    FsEventDispatcher::new(
        &DebouncedEventAdapter::new(receiver),
        &NewBookEventProcessor::new(&settings),
    )
    .handle()
    .expect("failed to dispatch events");
}
