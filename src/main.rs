use std::path::{Path, PathBuf};
use std::time::Instant;
use std::time::UNIX_EPOCH;

use log_roller;

/*
fn make_temp_file_name<P>(file: P) -> PathBuf
where
    P: AsRef<Path>,
{
    let mut n = std::time::SystemTime::now()
        .duration_since(std::time::SystemTime::UNIX_EPOCH)
        .unwrap_or_else(|_| std::time::Duration::from_secs(0))
        .as_secs();
    let mut temp = file.as_ref().to_path_buf();
    temp.set_extension(format!("{}", n));
    while temp.exists() {
        n += 1;
        temp.set_extension(format!("{}", n));
    }
    temp
}
*/

const ROLL_PATTERN: &str = "./tmp/daemon.log.{}.gz";
//const ROLL_PATTERN: &str = "daemon.log.{}.gz";
const LOG_FILE_PATH: &str = "./tmp/daemon.log";

struct RollPath(PathBuf);

impl RollPath {
    fn new<P>(path: P) -> Self
    where
        P: AsRef<Path>,
    {
        Self(path.as_ref().to_path_buf())
    }
}

#[derive(Debug)]
struct RollDirectory(PathBuf);

impl RollDirectory {
    fn new<P>(path: P) -> Self
    where
        P: AsRef<Path>,
    {
        Self(path.as_ref().to_path_buf())
    }
}

impl From<&RollPath> for RollDirectory {
    fn from(value: &RollPath) -> Self {
        Self(value.0
            .parent()
            .map(|p| p.to_path_buf())
            .unwrap_or_else(||PathBuf::from(".")))
    }
}

struct Phase1Finder {
    roll_directory: RollDirectory,
}

impl Phase1Finder {
    fn new(roll_directory: RollDirectory) -> Self {
        println!("roll_directory = {:?}", roll_directory);
        Self {
            roll_directory,
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let anchor = std::time::Instant::now();
    let roll_path = RollPath::new(ROLL_PATTERN);
    let finder = Phase1Finder::new((&roll_path).into());

    let roll_path = PathBuf::from(ROLL_PATTERN);
    let roll_directory = roll_path
        .parent()
        .map(|p| p.to_path_buf())
        .unwrap_or_else(||PathBuf::from("."));

    let _ = std::fs::remove_dir_all("./tmp");

    let /*RollingFileAppender::*/ path = PathBuf::from(LOG_FILE_PATH);

    if let Some(parent) = path.parent() {
        println!("parent = {:?}", parent);
        std::fs::create_dir_all(parent)?;
    }

// Write some stuff to daemon.log

    // file: &Path
// https://github.com/estk/log4rs/blob/master/src/append/rolling_file/policy/compound/roll/fixed_window.rs#L125-L127
    // rename the file
//    let temp = make_temp_file_name(file);
//    move_file(file, &temp)?;

    Ok(())
}
