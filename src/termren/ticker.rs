use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;
use std::time::Instant;
pub struct Ticker {
    ticktime: Duration,
    last_tick_time: Instant,
    frame_count: u64,
    tickers: Vec<Arc<Mutex<dyn Tickable + Send + Sync>>>,
}

pub enum TickCode {
    Success,
    StopTicker,
}

pub trait Tickable {
    fn tick(&mut self, ticker: &Ticker, delta_time: &Duration) -> TickCode;
}

unsafe impl Send for Ticker {}

impl Ticker {
    pub fn new(framerate: f64) -> Ticker {
        Ticker {
            ticktime: Duration::from_secs_f64(1.0 / framerate),
            last_tick_time: Instant::now(),
            frame_count: 0,
            tickers: Vec::new(),
        }
    }

    pub fn start(mut self) -> JoinHandle<()> {
        let _test1 = thread::spawn(move || 'thread: loop {
            let current_time = Instant::now();
            let delta_time: Duration = current_time.duration_since(self.last_tick_time);

            /* Frame limiter */
            if delta_time < self.ticktime {
                continue;
            }

            for ticked in &self.tickers {
                let mut _test = ticked.lock().unwrap();
                let code = _test.tick(&self, &delta_time);
                match code {
                    TickCode::StopTicker => { break 'thread; }
                    TickCode::Success => {}
                }
            }
            self.last_tick_time = current_time;
            self.frame_count += 1;
        });
        _test1
    }
    //TODO: accept borrow variable; perhaps send in a mutex with it -> overload this function
    pub fn add_ticked(&mut self, ticker: impl Tickable + Send + Sync + 'static) {
        self.tickers.push(Arc::new(Mutex::new(ticker)));
    }

    pub fn add_ref_ticked(&mut self, ticker:&Arc<Mutex<impl Tickable + Send + Sync + 'static>>) {
        self.tickers.push(ticker.clone());
    }

    pub fn frame_count(&self) -> u64 {
        self.frame_count
    }
}
