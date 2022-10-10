#![cfg(test)]
use futures::{
    future::{BoxFuture, FutureExt},
    task::{waker_ref, ArcWake},
};
use std::{
    future::Future,
    pin::Pin,
    sync::{
        mpsc::{sync_channel, Receiver, SyncSender},
        Arc, Mutex,
    },
    task::{Context, Poll, Waker},
    thread,
    time::{Duration, Instant},
};

/// Shared state between the future and the waiting thread.
struct AsyncState {
    waker: Option<Waker>,
    result: Option<Duration>,
}

impl AsyncState {
    pub fn new() -> Self {
        AsyncState {
            waker: None,
            result: None,
        }
    }

    pub fn complete(&mut self, duration: Duration) {
        self.result = Some(duration);

        if let Some(waker) = self.waker.take() {
            waker.wake();
        }
    }
}

pub struct AsyncThreadSleeper {
    async_state: Arc<Mutex<AsyncState>>,
}

impl Future for AsyncThreadSleeper {
    type Output = Duration;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut async_state = self
            .async_state
            .lock()
            .expect("Failed to lock shared state mutex.");
        if let Some(duration) = async_state.result {
            return Poll::Ready(duration);
        }
        async_state.waker = Some(cx.waker().clone());
        Poll::Pending
    }
}

impl AsyncThreadSleeper {
    pub fn new(duration: Duration) -> Self {
        // The shared state is what is needed to coordinate the future.
        let async_state = Arc::new(Mutex::new(AsyncState::new()));
        let async_state2 = async_state.clone();

        thread::spawn(move || {
            let start = Instant::now();

            // Sleep for the duration specified. This makes the behavior async.
            thread::sleep(duration);

            // Mark the async state as completed.
            async_state2
                .lock()
                .expect("Failed to lock shared state mutex")
                .complete(start.elapsed());
        });

        AsyncThreadSleeper {
            async_state: async_state,
        }
    }
}

struct Executor {
    ready_queue: Receiver<Arc<Task>>,
}

impl Executor {
    fn new() -> (Self, Spawner) {
        let (task_sender, ready_queue) = sync_channel(10_000 /* max tasks */);
        (Executor { ready_queue }, Spawner { task_sender })
    }

    fn run(&self) {
        while let Ok(task) = self.ready_queue.recv() {
            let mut future_slot = task.future.lock().unwrap();
            if let Some(mut future) = future_slot.take() {
                let waker = waker_ref(&task);
                let context = &mut Context::from_waker(&*waker);
                if future.as_mut().poll(context).is_pending() {
                    *future_slot = Some(future);
                }
            }
        }
    }
}

/// Takes a future, and sends it to the [Executor] via the `task_sender`.
#[derive(Clone)]
struct Spawner {
    task_sender: SyncSender<Arc<Task>>,
}

impl Spawner {
    fn spawn(&self, future: impl Future<Output = ()> + 'static + Send) {
        let future = future.boxed();
        let task = Arc::new(Task {
            future: Mutex::new(Some(future)),
            task_sender: self.task_sender.clone(),
        });
        self.task_sender.send(task).expect("too many tasks queued");
    }
}

struct Task {
    future: Mutex<Option<BoxFuture<'static, ()>>>,
    task_sender: SyncSender<Arc<Task>>,
}

impl ArcWake for Task {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        let cloned = arc_self.clone();
        arc_self
            .task_sender
            .send(cloned)
            .expect("too many tasks queued");
    }
}

#[test]
fn test_futures() {
    let (executor, spawner) = Executor::new();

    // Schedules 10 tasks.
    for i in 0..10 {
        spawner.spawn(async move {
            println!("Queuing AsyncThreadSleeper #{}", i);
            let duration = AsyncThreadSleeper::new(Duration::from_millis(100)).await;
            println!(
                "AsyncThreadSleeper #{} ran in {}ms",
                i,
                duration.as_millis()
            );
        });
    }

    drop(spawner);

    executor.run();
}
