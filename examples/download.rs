extern crate indicatif;

use std::thread;
use std::cmp::min;
use std::time::Duration;

use indicatif::{ProgressBar, ProgressStyle};

fn main() {
    let mut downloaded = 0;
    let total_size = 231_231_231;

    let pb = ProgressBar::new(total_size);
    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")
        .progress_chars("#>-"));

    while downloaded < total_size {
        let new = min(downloaded + 223_211, total_size);
        downloaded = new;
        pb.set_position(new);
        thread::sleep(Duration::from_millis(12));
    }

    pb.finish_with_message("downloaded");
}
