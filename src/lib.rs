//!
//! A simple program commandline arguments handler
//!
//! HanishKVC, 2022
//!

use std::collections::HashMap;
use std::env;

use loggerk::{log_e, log_d};


///
/// The ArgHandler takes
/// * a vector of cmdline arguments
/// * index of current argument being handled
/// It needs to return has to how many additional args
/// the handler has handled/consumed, if any.
///
type ArgHandler = fn(usize, &Vec<String>) -> usize;
pub struct SimpCmdLineManager {
    handlers: HashMap<String, ArgHandler>
}

#[allow(non_snake_case)]
impl SimpCmdLineManager {

    pub fn new() -> SimpCmdLineManager {
        SimpCmdLineManager {
            handlers: HashMap::new(),
        }
    }

    pub fn add_handler(&mut self, key: &str, ah: ArgHandler) {
        self.handlers.insert(key.to_string(), ah);
    }

    pub fn process_args(&self) {
        let theArgs: Vec<String> = env::args().collect();
        let totalArgs = theArgs.len();
        let mut iArg = 0usize;
        loop {
            iArg += 1;
            if iArg >= totalArgs {
                break;
            }
            let ah = self.handlers.get(&theArgs[iArg]);
            if ah.is_none() {
                log_e(&format!("ERRR:SimpleCmdLineManager:ProcessArgs:Unknown arg {}", theArgs[iArg]));
                continue;
            }
            let ah = ah.unwrap();
            let consumed = ah(iArg, &theArgs);
            iArg += consumed;
            log_d(&format!("DBUG:SimpleCmdLineManager:ProcessArgs:Consumed {} and following {} args", theArgs[iArg], consumed));
        }
    }

}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
