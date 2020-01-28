use colored::*;
use rand::seq::SliceRandom;
use std::{thread, time};
use rand::Rng;

fn main() {
    let mut xs = vec!["var", "std", "usr", "lib", "dev", "bin", "opt", "tmp"];
    let r = vec!["Fail".red(), "Ok".green()];
    let mut rng = rand::thread_rng();
    loop {
        let i: usize = rng.gen_range(1, 9);
        let spaces = " ".repeat(4 * (8 - i));
        let status = match rng.gen_range(0, 16) { 0 => &r[0], _ => &r[1] };
        xs.shuffle(&mut rng);
        thread::sleep(time::Duration::from_millis((i as u64) * 100));
        println!("  /{}/ {} {}", &xs[0..i].join("/"), spaces, status);
    }
}
