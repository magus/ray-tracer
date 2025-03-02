use std::{
    sync::{atomic, Arc},
    thread,
    time::Duration,
};

#[derive(Debug)]
pub struct State {
    cur: atomic::AtomicU32,
    max: u32,
}

#[derive(Clone, Debug)]
pub struct Progress {
    state: Arc<State>,
}

pub struct ProgressThread {
    join_handle: Option<thread::JoinHandle<()>>,
}

impl Drop for ProgressThread {
    fn drop(&mut self) {
        if let Some(join_handle) = self.join_handle.take() {
            join_handle.join().unwrap();
        }
    }
}

impl Progress {
    pub fn new(max: u32) -> Self {
        let state = Arc::new(State {
            cur: atomic::AtomicU32::new(0),
            max,
        });

        Progress { state }
    }

    pub fn inc(&self) -> u32 {
        self.state.cur.fetch_add(1, atomic::Ordering::Relaxed) + 1
    }

    pub fn bar(&self, frame: usize) -> String {
        let bar_width = 48;

        let current = self.state.cur.load(atomic::Ordering::Relaxed);
        let percent = current as f64 / self.state.max as f64;
        let filled = (percent * bar_width as f64).round() as usize;
        let empty = format!("\x1b[90m{}\x1b[0m", "─".repeat(bar_width - filled));
        let filled = "━".repeat(filled);

        let spinner_frame = SPINNER_CHARS[frame % SPINNER_CHARS.len()];
        let percent = (percent * 100.0) as u32;

        let spinner = if percent == 100 {
            format!("")
        } else {
            format!("\x1b[1m\x1b[36m{spinner_frame}\x1b[0m")
        };

        let percent = format!("{percent:>3}%");

        format!("{spinner} {percent} {filled}{empty}  ")
    }

    /// Spawn thread that draws at consistent fps
    pub fn render(&self, fps: u64) -> ProgressThread {
        let frame_duration = Duration::from_millis(1000 / fps);
        let progress = self.clone();

        let join_handle = thread::spawn(move || {
            let mut frame = 0;

            loop {
                let cur = progress.state.cur.load(atomic::Ordering::Relaxed);
                let max = progress.state.max;
                let done = cur >= max;

                eprint!("\r{}", progress.bar(frame));

                if done {
                    break;
                }

                thread::sleep(frame_duration);
                frame += 1;
            }
        });

        let join_handle = Some(join_handle);
        ProgressThread { join_handle }
    }
}

const SPINNER_CHARS: [&str; 10] = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
