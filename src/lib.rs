//!
//! A simple program commandline arguments handler
//!
//! HanishKVC, 2022
//!

use std::collections::HashMap;
use std::env;

use loggerk::{log_w, log_d};


///
/// The ArgHandler takes
/// * index of current argument being handled
/// * a vector of cmdline arguments
/// It needs to return has to how many additional args
/// the handler has handled/consumed, if any.
///
type ArgHandler<'a> = &'a mut dyn FnMut(usize, &Vec<String>) -> usize;
pub struct ArgsCmdLineSimpleManager<'a> {
    handlers: HashMap<String, ArgHandler<'a>>
}

#[allow(non_snake_case)]
impl<'a> ArgsCmdLineSimpleManager<'a> {

    pub fn new() -> ArgsCmdLineSimpleManager<'a> {
        ArgsCmdLineSimpleManager {
            handlers: HashMap::new(),
        }
    }

    ///
    /// Specify
    /// * the named arg to process in the command line arguments list
    /// * the handler that should be called to process the same
    ///
    /// Handler is defined as a closure to allow any variable in the
    /// enclosing context in the caller to be manipulated.
    ///
    pub fn add_handler(&mut self, key: &str, ah: ArgHandler<'a>) {
        self.handlers.insert(key.to_string(), ah);
    }

    ///
    /// Once all the required named arguments and their handlers have
    /// been setup using add_handler, call this to process the cmdline
    /// arguments of the program / process calling this.
    ///
    pub fn process_args(&mut self) {
        let theArgs: Vec<String> = env::args().collect();
        let totalArgs = theArgs.len();
        let mut iArg = 0usize;
        loop {
            iArg += 1;
            if iArg >= totalArgs {
                break;
            }
            let ah = self.handlers.get_mut(&theArgs[iArg]);
            if ah.is_none() {
                log_w(&format!("WARN:SimpleCmdLineManager:ProcessArgs:Unknown arg:{}", theArgs[iArg]));
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
