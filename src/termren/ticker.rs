use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;
use std::time::Instant;

pub struct Ticker<T: Tickable<T>> {
    ticktime: Duration,
    last_tick_time: Instant,
    frame_count: u64,
    tickable: T,
}

pub enum TickCode {
    Success,
    StopTicker,
}

pub trait Tickable<T>: Send + Sync {
    fn tick(&mut self, _origin: Option<&mut T>, _delta_time: Duration) -> TickCode {
        TickCode::Success
    }
}

impl<T: Tickable<(Arc<Mutex<T>>, Arc<Mutex<K>>)>, K: Tickable<(Arc<Mutex<T>>, Arc<Mutex<K>>)>> Tickable<(Arc<Mutex<T>>, Arc<Mutex<K>>)> for (Arc<Mutex<T>>, Arc<Mutex<K>>) {
    fn tick(&mut self, _origin: Option<&mut (Arc<Mutex<T>>, Arc<Mutex<K>>)>, delta_time: Duration) -> TickCode {
        let (t0, t1) = self.clone();
        let mut u0 = t0.lock().unwrap();
        let mut u1 = t1.lock().unwrap();
        let c0 = u0.tick(Some(self), delta_time);
        let c1 = u1.tick(Some(self), delta_time);

        match c0 {
            TickCode::StopTicker => return TickCode::StopTicker,
            TickCode::Success => {}
        }
        c1
    }
}

impl<T: Tickable<T>> Tickable<Arc<Mutex<T>>> for Arc<Mutex<T>> {
    fn tick(&mut self, _origin: Option<&mut Arc<Mutex<T>>>, delta_time: Duration) -> TickCode {
        self.lock().unwrap().tick(None, delta_time);
        TickCode::Success
    }
}

//unsafe impl<T> Send for Ticker<T> {}

impl<T: Tickable<T> + 'static> Ticker<T> {
    pub fn new(framerate: f64, tickable: T) -> Ticker<T> {
        Ticker { ticktime: Duration::from_secs_f64(1.0 / framerate), last_tick_time: Instant::now(), frame_count: 0, tickable }
    }

    pub fn start(mut self) -> JoinHandle<()> {
        let thread = thread::spawn(move || 'thread: loop {
            let current_time = Instant::now();
            let delta_time: Duration = current_time.duration_since(self.last_tick_time);

            /* Frame limiter */
            if delta_time < self.ticktime {
                continue;
            }

            let code = self.tickable.tick(None, delta_time);

            match code {
                TickCode::StopTicker => {
                    break 'thread;
                }
                TickCode::Success => {}
            }

            self.last_tick_time = current_time;
            self.frame_count += 1;
        });
        thread
    }

    pub fn frame_count(&self) -> u64 {
        self.frame_count
    }

    pub fn set_tickable(&mut self, ticker: T) {
        self.tickable = ticker;
    }

    pub fn get_tickable(&mut self) -> &mut T {
        &mut self.tickable
    }
}
