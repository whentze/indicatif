extern crate indicatif;

use std::thread;
use std::time::Duration;

use indicatif::{ProgressBar, ProgressStyle};

pub static DANCING_KIRBY: [&'static str; 8] = [
"(>'-')>",
"<('-'<)",
"^('-')^",
"<('-'<)",
"(>'-')>",
"<('-'<)",
"^('-')^",
"<('-'<)"
];

fn main() {
    let pb = ProgressBar::new_spinner();
    pb.set_style(ProgressStyle::default_spinner()
        .tick_chars_multi(&DANCING_KIRBY));
    for _ in 0..1024 {
        pb.inc(1);
        thread::sleep(Duration::from_millis(5));
    }
    pb.finish_with_message("done");
}
