use notify::DebouncedEvent;
use std::path::PathBuf;
use std::sync::mpsc::{Receiver, RecvError};

pub(crate) trait EventProvider {
    fn next(&self) -> Result<FsEvent, RecvError>;
}

pub(crate) enum FsEvent {
    Stop,
    NewFile(PathBuf),
    Other,
}

pub(crate) struct DebouncedEventAdapter {
    receiver: Receiver<DebouncedEvent>,
}

impl DebouncedEventAdapter {
    pub(crate) fn new(receiver: Receiver<DebouncedEvent>) -> Self {
        DebouncedEventAdapter { receiver }
    }
}

impl EventProvider for DebouncedEventAdapter {
    fn next(&self) -> Result<FsEvent, RecvError> {
        match self.receiver.recv() {
            Ok(DebouncedEvent::Create(p)) => Ok(FsEvent::NewFile(p)),
            Ok(_) => Ok(FsEvent::Other),
            Err(e) => Err(e),
        }
    }
}
