#![allow(dead_code)]
#![allow(unused_variables)]

use chrono::Local;
use colored::*;

pub fn warn(message: &str) {
    out("WARN", message);
}
pub fn info(message: &str) {
    out("INFO", message);
}
pub fn err(message: &str) {
    out("ERR", message);
}
pub fn ok(message: &str) {
    out("SUCCESS", message);
}

fn out (logtype: &str, message: &str) {
    let msg = message;
    let now = Local::now();
    let append = match logtype {
        "WARN" => "WARNING".yellow(),
        "INFO" => "INFO   ".blue(),
        "ERR"  => "ERROR  ".red(),
        "SUCCESS" => "SUCCESS".green(),
        _ => "UNDEFINED".cyan()
    };
    println!("{:5} {} {}", now.format("[%Y-%m-%d][%H:%M:%S]"), append, msg);
}