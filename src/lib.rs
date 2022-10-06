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

///
/// The Core Entity for managing commandline arguments of a process
/// in a simple mannger.
/// Call its
/// * new method to create a new instance of this.
/// * add_handler method to specify the named arguments to process and
///   the handler to call wrt same.
/// * set_remaining_marker method to optionally set a marker wrt remaining
///   arguments towards the end. All arguments after this marker in the
///   commandline will be collected into the remaining member.
/// * process_args method to process the cmdline arguments as needed.
/// Access its
/// * unhandled member after process_args to get the list of unhandled args.
///   This doesnt include any remaining arguments.
/// * remaining member after process_args to get the list of remaining args,
///   provided a remaining_marker was setup.
///   One can check found_remainingmarker member 1st to check if marker was
///   found among the cmdline arguments.
///
pub struct ArgsCmdLineSimpleManager<'a> {
    handlers: HashMap<String, ArgHandler<'a>>,
    pub unhandled: Vec<String>,
    remaining_marker: String,
    pub found_remainingmarker: bool,
    pub remaining: Vec<String>,
}

#[allow(non_snake_case)]
impl<'a> ArgsCmdLineSimpleManager<'a> {

    pub fn new() -> ArgsCmdLineSimpleManager<'a> {
        ArgsCmdLineSimpleManager {
            handlers: HashMap::new(),
            unhandled: Vec::new(),
            remaining_marker: String::new(),
            found_remainingmarker: false,
            remaining: Vec::new(),
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
    /// Call this optionally to set a remaining args marker wrt the cmdline.
    /// Any arguments after this marker, will not be handled by this library.
    /// User can get the list of such arguments by accessing remaining member
    /// after call to process_args.
    ///
    pub fn set_remaining_marker(&mut self, marker: &str) {
        self.remaining_marker = marker.to_string().clone();
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
            if self.found_remainingmarker {
                self.remaining.push(theArgs[iArg].clone());
                continue;
            }
            if theArgs[iArg] == self.remaining_marker {
                self.found_remainingmarker = true;
                log_d(&format!("DBUG:ArgsCmdLineSimpleManager:ProcessArgs:Found remaining marker[{}], remaining arguments if any will be skipped", theArgs[iArg]));
                continue
            }
            let ah = self.handlers.get_mut(&theArgs[iArg]);
            if ah.is_none() {
                if theArgs[iArg] == "--help" {
                    for k in self.handlers.keys() {
                        log_d(&format!("\t{}\n", k));
                    }
                }
                log_w(&format!("WARN:ArgsCmdLineSimpleManager:ProcessArgs:Unknown unhandled arg:{}", theArgs[iArg]));
                self.unhandled.push(theArgs[iArg].clone());
                continue;
            }
            let ah = ah.unwrap();
            let consumed = ah(iArg, &theArgs);
            log_d(&format!("DBUG:ArgsCmdLineSimpleManager:ProcessArgs:Consumed {} and {} following args", theArgs[iArg], consumed));
            iArg += consumed;
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
