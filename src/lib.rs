//!
//! A simple program commandline arguments handler
//!
//! HanishKVC, 2022
//!

use std::collections::HashMap;
use std::env;

///
/// The ArgHandler takes
/// * a vector of cmdline arguments
/// * index of current argument being handled
/// It is required to return has to how many args the handler has handled/consumed.
/// * at a minimum it should return 1 (ie the key arg itself)
///
type ArgHandler = fn(usize, &Vec<String>) -> usize;
struct SimpCmdLineManager {
    handlers: HashMap<String, ArgHandler>
}

#[allow(non_snake_case)]
impl SimpCmdLineManager {

    fn add_handler(&mut self, key: &str, ah: ArgHandler) {
        self.handlers.insert(key.to_string(), ah);
    }

    fn process_args(&mut self) {
        let theArgs: Vec<String> = env::args().collect();
        let totalArgs = theArgs.len();
        let mut iArg = 1usize;
        loop {
            if iArg >= totalArgs {
                break;
            }

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
