use std::fs::File;

use crate::dispatcher::EventDispatcher;
use crate::processor::NewBookEventProcessor;
use crate::provider::DebouncedEventAdapter;
use crate::settings::Settings;
use anyhow::Result;
use daemonize::Daemonize;
use notify::{watcher, RecursiveMode, Watcher};
use std::sync::mpsc::channel;
use std::time::Duration;

mod dispatcher;
mod processor;
mod provider;
mod settings;

fn main() -> Result<()> {
    let settings = Settings::load()?;

    let daemonize = Daemonize::new()
        .working_directory(settings.books_dir())
        .stdout(File::create(settings.stdout_file())?)
        .stderr(File::create(settings.stderr_file())?);

    match daemonize.start() {
        Ok(_) => watch_for_added_books(&settings)?,
        Err(e) => eprintln!("Error, {}", e),
    }
    Ok(())
}

fn watch_for_added_books(settings: &Settings) -> Result<()> {
    println!("Success, daemonized");
    let (sender, receiver) = channel();

    let mut watcher = watcher(sender, Duration::from_secs(settings.interval()))
        .expect("failed to create watcher");
    watcher.watch(settings.books_dir(), RecursiveMode::Recursive)?;

    EventDispatcher::new(
        &DebouncedEventAdapter::new(receiver),
        &NewBookEventProcessor::new(&settings),
    )
    .dispatch()?;

    Ok(())
}
