use std::{
    sync::{atomic, Arc},
    thread,
    time::Duration,
};

#[derive(Debug)]
pub struct State {
    cur: atomic::AtomicUsize,
    max: usize,
}

impl State {
    pub fn bar(&self, frame: usize) -> String {
        let bar_width = 48;

        let cur = self.cur.load(atomic::Ordering::Relaxed);
        let max = self.max;
        let percent = cur as f64 / max as f64;
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

        let cur = format_number(cur);
        let max = format_number(max);
        let digits = max.len();
        let cur = format!("{:>width$}", cur, width = digits);
        format!("{spinner}{percent} {filled}{empty} {cur} / {max} ")
    }

    pub fn print(&self, frame: usize) {
        // carriage return and clear line from cursor to end
        eprint!("\r\x1b[K{}", self.bar(frame));
    }
}

#[derive(Debug)]
pub struct Progress {
    state: Arc<State>,
    join_handle: Option<thread::JoinHandle<()>>,
}

impl Drop for Progress {
    fn drop(&mut self) {
        if let Some(join_handle) = self.join_handle.take() {
            join_handle.join().unwrap();
        }

        self.state.print(0);
        eprintln!();
    }
}

impl Progress {
    pub fn new(max: usize) -> Self {
        let state = Arc::new(State {
            cur: atomic::AtomicUsize::new(0),
            max,
        });

        let join_handle = None;

        Progress { state, join_handle }
    }

    pub fn inc(&self) -> usize {
        self.state.cur.fetch_add(1, atomic::Ordering::Relaxed) + 1
    }

    /// Spawn thread that draws at consistent fps
    pub fn render(mut self, fps: u64) -> Self {
        let frame_duration = Duration::from_millis(1000 / fps);
        let state = self.state.clone();

        let join_handle = thread::spawn(move || {
            let mut frame = 0;

            loop {
                let cur = state.cur.load(atomic::Ordering::Relaxed);
                let max = state.max;
                let done = cur >= max;

                // carriage return and clear line from cursor to end
                state.print(frame);

                if done {
                    break;
                }

                thread::sleep(frame_duration);
                frame += 1;
            }
        });

        self.join_handle = Some(join_handle);
        self
    }
}

const SPINNER_CHARS: [&str; 10] = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];

fn format_number(value: usize) -> String {
    let value_str = value.to_string();

    let chars: Vec<char> = value_str.chars().rev().collect();

    chars
        .chunks(3)
        .map(|chunk| chunk.iter().rev().collect::<String>())
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .collect::<Vec<_>>()
        .join(",")
}
