use std::fs::OpenOptions;
use std::io::Write;

const LOG_PATH: &str = "/tmp/goose-debug.log";

pub fn log(msg: &str) {
    if let Ok(mut f) = OpenOptions::new().create(true).append(true).open(LOG_PATH) {
        let _ = writeln!(f, "{msg}");
        let _ = f.flush();
    }
}
