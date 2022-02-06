use std::fs::File;

use crate::dispatcher::EventDispatcher;
use crate::processor::NewBookEventProcessor;
use crate::provider::DebouncedEventAdapter;
use crate::settings::{config_path, Settings};
use anyhow::{bail, Result};
use daemonize::Daemonize;
use notify::{watcher, RecursiveMode, Watcher};
use std::env;
use std::fs;
use std::path::Path;
use std::sync::mpsc::channel;
use std::time::Duration;

mod dispatcher;
mod processor;
mod provider;
mod settings;

fn main() -> Result<()> {
    do_checks()?;

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

fn do_checks() -> Result<()> {
    if !Path::new(&config_path()?).exists() {
        bail!("configuration file not found under {}", config_path()?);
    }
    if !is_program_in_path("bt-obex") {
        bail!("bt-obex not found in $PATH");
    }
    Ok(())
}

fn is_program_in_path(program: &str) -> bool {
    if let Ok(path) = env::var("PATH") {
        for p in path.split(':') {
            let p_str = format!("{}/{}", p, program);
            if fs::metadata(p_str).is_ok() {
                return true;
            }
        }
    }
    false
}

fn watch_for_added_books(settings: &Settings) -> Result<()> {
    println!("Success, daemonized");
    let (sender, receiver) = channel();

    let mut watcher = watcher(sender, Duration::from_secs(settings.interval()))?;
    watcher.watch(settings.books_dir(), RecursiveMode::Recursive)?;

    EventDispatcher::new(
        &DebouncedEventAdapter::new(receiver),
        &NewBookEventProcessor::new(settings),
    )
    .dispatch()?;

    Ok(())
}
