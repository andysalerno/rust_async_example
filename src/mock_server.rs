use std::future::Future;
use std::pin::Pin;
use std::task::Context;
use std::task::Poll;
use std::time::Duration;
use std::task::Waker;

/// Not currently possible in Nightly
/// (trait functions cannot be async :( )
// trait SocketConn {
//     async fn read_to_end(&self) -> Response;
// }

pub struct MockServer;

impl MockServer {
    pub async fn read_from_address(addr: &str) -> String {
        MockSocketConn::connect().to_address(addr).await
    }
}

#[derive(Default)]
pub struct MockSocketConn {
    address: String,
    duration_remaining: Option<Duration>,
}

impl MockSocketConn {
    pub fn connect() -> Self {
        Self::default()
    }

    pub fn to_address(mut self, address: &str) -> Self {
        self.address = address.into();
        self
    }
}

impl Future for MockSocketConn {
    type Output = String;

    fn poll(mut self: Pin<&mut Self>, ctx: &mut Context<'_>) -> Poll<Self::Output> {
        use rand::Rng;

        // How much work is left to do in total for this Future?
        let duration_remaining = if let Some(duration) = self.duration_remaining {
            duration
        } else {
            let total_work_duration_ms = rand::thread_rng().gen_range(100, 1000);
            self.duration_remaining = Some(Duration::from_millis(total_work_duration_ms));
            self.duration_remaining.unwrap()
        };

        dbg!(duration_remaining);

        // How much work can we do right now, during this poll()?
        const POLL_WORK_TIME_MS: u64 = 10;
        let work_duration = Duration::from_millis(POLL_WORK_TIME_MS);

        // Do the "work" during this poll().
        do_some_work(work_duration);

        // Update the remaining duration.
        let new_duration_remaining = duration_remaining.checked_sub(work_duration);

        match new_duration_remaining {
            Some(_) => {
                // There's still more work, so let's pretend we have a reactor
                // that will wake the runtime up when we are ready to do more work.
                let waker = ctx.waker().clone();
                ToyReactor::register_waker(waker);

                self.duration_remaining = new_duration_remaining;
                Poll::Pending
            }
            None => Poll::Ready(format!("Received response from: {}", self.address)),
        }
    }
}

/// Do some "work".
fn do_some_work(work_duration: Duration) {
    std::thread::sleep(work_duration);
}

struct ToyReactor;

impl ToyReactor {
    fn register_waker(waker: Waker) {
        std::thread::spawn(move || {
            // In SLEEP_MS ms, the pretend "reactor" will learn that
            // we are ready to do more work
            // (perhaps a network response has arrived, or a disk IO completed)
            // and it will inform the executor to wake us up.
            const SLEEP_MS: u64 = 100;
            let sleep_duration = Duration::from_millis(SLEEP_MS);
            std::thread::sleep(sleep_duration);
            waker.wake();
        });
    }
}