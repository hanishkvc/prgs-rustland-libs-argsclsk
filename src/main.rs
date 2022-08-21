//!
//! A simple test logic for cross checking the cmdline args library
//!
//! HanishKVC, 2022
//!

use simpcmdlinek;
use loggerk::{log_init, log_d};

fn test_cmdline(mut key3magic: u32) {
    let mut sclm = simpcmdlinek::SimpCmdLineManager::new();

    let mut key1h = |iarg: usize, theargs: &Vec<String>| -> usize {
        log_d(&format!("FoundArg:@{}:{}", iarg, theargs[iarg]));
        0
    };
    sclm.add_handler("--key1", &mut key1h);

    let mut key2h = |iarg: usize, theargs: &Vec<String>| -> usize {
        log_d(&format!("FoundArg:@{}:{}", iarg, theargs[iarg]));
        1
    };
    sclm.add_handler("--key2", &mut key2h);
    let mut key3h = |iarg, theargs: &Vec<String>|-> usize {
        let localmagic = key3magic;
        key3magic += 3;
        log_d(&format!("FoundArg:@{}:{}: Also key3magic is {}", iarg, theargs[iarg], localmagic));
        0
    };
    sclm.add_handler("--key3", &mut key3h);
    sclm.process_args();
    drop(sclm);
}


fn main() {
    log_init();
    test_cmdline(123);
}
