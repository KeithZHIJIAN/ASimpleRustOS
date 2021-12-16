# ASimpleRustOS
Building a minimal operating system with Rust.

# ch1 - finished by Dec 16
## Intro
Here we implemented a colorized logging function. The system now allow users to set its log level via environment variable "LOG".
The default log level is "info", so the system will only print info logs or logs with higher level. Use "make run LOG=xxx" to change the level.
## Work flow
info macro => log macro, where log level is checked => log function => print_in_color macro => write function in MyStdout.
## Problem
Here the log macro will call get_max_level() function and do the matching every time it is called. It is possible to store the result as a static or const variable to optimize the program. However, there is a problem with using option_env macro to assign a const value, "calls in constant functions are limited to constant functions, tuple structs and tuple variants". So the question now becomes how to store an environment variable with a const rust variable.
