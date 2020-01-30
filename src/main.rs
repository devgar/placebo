use colored::*;
use rand::seq::SliceRandom;
use std::{thread, time};
use chrono::Local;
use rand::Rng;

fn main() {
    let mut xs = vec!["var", "std", "usr", "lib", "dev", "bin", "opt", "tmp"];
    let r = vec!["Err".red(), "Ok".green()];
    let mut rng = rand::thread_rng();
    loop {
        let i: usize = rng.gen_range(1, 9);
        let time = Local::now().format("%Y-%m-%d %H:%M:%S");
        let path = &xs[0..i].join("/").blue();
        let spaces = " ".repeat(4 * (8 - i));
        let status = match rng.gen_range(0, 16) { 0 => &r[0], _ => &r[1] };
        xs.shuffle(&mut rng);
        thread::sleep(time::Duration::from_millis((9 + (i as u64)) * 100));
        println!("{}\n  /{}/ {} {}", time, path, spaces, status);
    }
}
