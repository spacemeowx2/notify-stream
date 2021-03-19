use std::{
    path::Path,
    pin::Pin,
    task::{Context, Poll},
};

pub use notify;

use futures_channel::mpsc;
use futures_core::stream::Stream;
use notify::{RecommendedWatcher, RecursiveMode, Watcher};

type StreamItem = notify::Result<notify::Event>;
pub struct NotifyStream(mpsc::UnboundedReceiver<StreamItem>, RecommendedWatcher);

impl Stream for NotifyStream {
    type Item = StreamItem;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        Pin::new(&mut self.0).poll_next(cx)
    }
}

pub fn notify_stream<P: AsRef<Path>>(
    path: P,
    recursive_mode: RecursiveMode,
) -> notify::Result<NotifyStream> {
    let (tx, rx) = mpsc::unbounded();

    let mut watcher: RecommendedWatcher =
        Watcher::new_immediate(move |i: notify::Result<notify::Event>| {
            // ignore error
            let _ = tx.unbounded_send(i).ok();
        })?;

    watcher.watch(path, recursive_mode)?;

    Ok(NotifyStream(rx, watcher))
}
