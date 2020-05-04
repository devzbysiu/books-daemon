use notify::DebouncedEvent;
use std::path::PathBuf;
use std::sync::mpsc::{Receiver, RecvError};

pub(crate) trait EventProvider {
    fn next(&self) -> Result<FsEvent, RecvError>;
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum FsEvent {
    // for testing purposes
    #[allow(dead_code)]
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

#[cfg(test)]
mod test {
    use crate::provider::{DebouncedEventAdapter, EventProvider, FsEvent};
    use notify::DebouncedEvent;
    use std::path::PathBuf;
    use std::sync::mpsc::channel;

    #[test]
    fn test_create_event_is_correctly_adapted_to_new_file_event() {
        // given
        let (sender, receiver) = channel();
        let provider = DebouncedEventAdapter::new(receiver);

        // when
        sender
            .send(DebouncedEvent::Create(PathBuf::from("/test")))
            .unwrap();

        // then
        assert_eq!(
            provider.next().unwrap(),
            FsEvent::NewFile(PathBuf::from("/test")),
        );
    }

    #[test]
    fn test_any_other_event_is_converted_to_other_event() {
        // given
        let (sender, receiver) = channel();
        let provider = DebouncedEventAdapter::new(receiver);

        // when
        sender
            .send(DebouncedEvent::Chmod(PathBuf::from("/test")))
            .unwrap();
        sender
            .send(DebouncedEvent::Write(PathBuf::from("/test")))
            .unwrap();
        sender
            .send(DebouncedEvent::Remove(PathBuf::from("/test")))
            .unwrap();
        sender
            .send(DebouncedEvent::Rename(
                PathBuf::from("/test"),
                PathBuf::from("/test1"),
            ))
            .unwrap();
        sender
            .send(DebouncedEvent::NoticeWrite(PathBuf::from("/test")))
            .unwrap();
        sender
            .send(DebouncedEvent::NoticeRemove(PathBuf::from("/test")))
            .unwrap();
        sender.send(DebouncedEvent::Rescan).unwrap();

        // then
        assert_eq!(FsEvent::Other, provider.next().unwrap());
        assert_eq!(FsEvent::Other, provider.next().unwrap());
        assert_eq!(FsEvent::Other, provider.next().unwrap());
        assert_eq!(FsEvent::Other, provider.next().unwrap());
        assert_eq!(FsEvent::Other, provider.next().unwrap());
        assert_eq!(FsEvent::Other, provider.next().unwrap());
        assert_eq!(FsEvent::Other, provider.next().unwrap());
    }
}
