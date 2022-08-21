//!
//! A simple test logic for cross checking the cmdline args library
//!
//! HanishKVC, 2022
//!

use simpcmdlinek;
use loggerk::{log_init, log_d};

fn main() {
    log_init();
    let mut sclm = simpcmdlinek::SimpCmdLineManager::new();
    sclm.add_handler("--key1", |iarg: usize, theargs: &Vec<String>| -> usize {
        log_d(&format!("FoundArg:@{}:{}", iarg, theargs[iarg]));
        0
    });
    sclm.process_args();
}
