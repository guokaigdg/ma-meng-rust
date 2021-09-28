/*
 * @Author: guokai05
 * @Date: 2021-09-27 15:05:33
 * @LastEditors: guokai05
 * @LastEditTime: 2021-09-27 15:35:19
 */

// fn main() {
//     println!("Hello, world!");
// }

use ferris_says::say; // from the previous step
use std::io::{stdout, BufWriter};

fn main() {
    let stdout = stdout();
    let message = String::from("Hello fellow Rustaceans!");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();
}