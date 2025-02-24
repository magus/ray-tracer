pub struct Progress {
    pub cur: std::sync::Mutex<u32>,
    pub max: u32,
}

impl Progress {
    pub fn new(max: u32) -> Self {
        Progress {
            cur: std::sync::Mutex::new(0),
            max,
        }
    }

    pub fn inc(&self) -> u32 {
        let mut p_mut = self.cur.lock().unwrap();
        *p_mut += 1;
        *p_mut
    }

    pub fn bar(&self, current: u32) -> String {
        let percent = current as f64 / self.max as f64;
        let bar_width = 48;
        let filled = (percent * bar_width as f64).round() as usize;
        let empty = format!("\x1b[90m{}\x1b[0m", "─".repeat(bar_width - filled));
        let filled = "━".repeat(filled);
        let spinner_frame = SPINNER_CHARS[(current as usize) % SPINNER_CHARS.len()];
        let percent = (percent * 100.0) as u32;

        let spinner = if percent == 100 {
            format!("")
        } else {
            format!("\x1b[1m\x1b[36m{spinner_frame}\x1b[0m")
        };

        let percent = format!("{percent:>3}%");

        format!("{spinner} {percent} {filled}{empty}  ")
    }
}

const SPINNER_CHARS: [&str; 10] = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
