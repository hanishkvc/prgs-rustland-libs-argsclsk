//!
//! A simple test logic for cross checking the cmdline args library
//!
//! HanishKVC, 2022
//!

use simpcmdlinek;
use loggerk::{log_init, log_d};

fn test_cmdline(mut sharedvar: u32) {
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


    log_d(&format!("DBUG:TestCmdLine:SharedVar:B4DefiningClosure:{}", sharedvar));
    let mut key3h = |iarg, theargs: &Vec<String>|-> usize {
        let localmagic = sharedvar;
        sharedvar += 3;
        log_d(&format!("FoundArg:@{}:{}: Also key3magic is {}", iarg, theargs[iarg], localmagic));
        0
    };
    sclm.add_handler("--key3", &mut key3h);
    //log_d(&format!("DBUG:TestCmdLine:SharedVar:B4ProcessArgs:{}", sharedvar));
    sclm.process_args();
    log_d(&format!("DBUG:TestCmdLine:SharedVar:AfterProcessArgs:{}", sharedvar));
    //drop(sclm); // By commenting out this drop, we allow compiler to auto drop it immidiately after process_args automatically
}


fn main() {
    log_init();
    test_cmdline(123);
}
