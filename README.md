# ArgsClSK - A simple commandline arguments manager/handler

Auther: HanishKVC

Version: 20221006IST1210

License: GPL

## Overview

It allows a rust program to handle any commandline arguments passed to it
in a simple yet structured way.

It allows arguments that need to be processed to be identified and handlers
registered. The handler can decide how many adjacent arguments it will consume.

A remaining args marker can be set, to allow for any arguments that need to be
left as is towards the end to be achieved. A list of unhandled and remaining
arguments after arguments have been handled can be got.


## Usage flow

### Setup the logic

* Create a new instance of the ArgsCmdLineSimpleManager

* call add_handler method for each named command line argument, that one
  wants to process, and specify the handler to call wrt same.

  * the handler ideally needs to be defined as a closure which captures
    the required variable which will communicate the got data to the program,
    in a mutable way.

* one can call the set_remaining_marker method to optionally set a marker wrt
  any remaining arguments towards the end. All arguments after this marker in
  the commandline will be collected into the remaining member of the manager
  instance.

### Process the arguments

* call process_args method to process the cmdline arguments as needed.

  this will go through the commandline arguments of the process and inturn
  call the registered argument handlers, as well as put unhandled arguments
  into either unhandled or remaining vector, as the case may be.

### Access the extras

Access arguments that were not handled, and which appear before or after the
remaining marker, if any.

* use unhandled member after process_args to get the list of unhandled args.
  This doesnt include any remaining arguments.

* use remaining member after process_args to get the list of remaining args,
  provided a remaining_marker was setup.

  * One can check found_remainingmarker member 1st to check if marker was
    found among the cmdline arguments.

