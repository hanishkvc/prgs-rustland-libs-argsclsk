//!
//! A simple test logic for cross checking the cmdline args library
//!
//! HanishKVC, 2022
//!

use simpcmdlinek;
use loggerk::{log_init, log_d};

fn test_cmdline(key3magic: u32) {
    let mut sclm = simpcmdlinek::SimpCmdLineManager::new();
    sclm.add_handler("--key1", &|iarg: usize, theargs: &Vec<String>| -> usize {
        log_d(&format!("FoundArg:@{}:{}", iarg, theargs[iarg]));
        0
    });
    sclm.add_handler("--key2", &|iarg: usize, theargs: &Vec<String>| -> usize {
        log_d(&format!("FoundArg:@{}:{}", iarg, theargs[iarg]));
        1
    });
    let key3h = |iarg, theargs: &Vec<String>|-> usize {
        let localmagic = key3magic;
        log_d(&format!("FoundArg:@{}:{}: Also key3magic is {}", iarg, theargs[iarg], localmagic));
        0
    };
    sclm.add_handler("--key3", &key3h);
    sclm.process_args();
    drop(sclm);
}


fn main() {
    log_init();
    test_cmdline(123);
}
