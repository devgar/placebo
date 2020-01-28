use colored::*;
use rand::seq::SliceRandom;
use std::{thread, time};
use rand::Rng;

fn main() {
    let mut xs: [String; 8] = [
        "var".to_string(),
        "std".to_string(),
        "usr".to_string(),
        "lib".to_string(),
        "dev".to_string(),
        "bin".to_string(),
        "opt".to_string(),
        "tmp".to_string()
    ];
    let mut rng = rand::thread_rng();
    loop {
        let i: usize = rand::thread_rng().gen_range(1, 9);
        let hun_millis = time::Duration::from_millis((i as u64) * 100);
        thread::sleep(hun_millis);
        xs.shuffle(&mut rng);
        let status = {
            if rand::thread_rng().gen_range(0, 16) == 0 {
                "Fail".red()
            } else { "Ok".green() }
        };
        println!("  /{}/ {} {}", &xs[0..i].join("/"), " ".repeat(4 * (8 - i)), status);
    }
}
