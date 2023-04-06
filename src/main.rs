extern crate captrs;
extern crate shuteye;

use captrs::*;
use shuteye::sleep;
use std::time::Duration;
use x11cap::Bgr8;

fn main() {
    let mut capturer = Capturer::new(0).unwrap();

    let (w, h) = capturer.geometry();
    let size = w as u64 * h as u64;

    loop {
        let ps = capturer.capture_frame().unwrap();

        let (mut tot_r, mut tot_g, mut tot_b) = (0, 0, 0);

        for Bgr8 { r, g, b, .. } in ps.into_iter() {
            tot_r += r as u64;
            tot_g += g as u64;
            tot_b += b as u64;
        }

        println!("Avg: {:?}", (tot_r / size, tot_g / size, tot_b / size));

        sleep(Duration::from_millis(80));
    }
}
