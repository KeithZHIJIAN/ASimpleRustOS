# ASimpleRustOS
Building a minimal operating system with Rust.

# ch1 - finished by Dec 16
## Intro
Here we implemented a colorized logging function. The system now allow users to set its log level via environment variable "LOG".
The default log level is "info", so the system will only print info logs or logs with higher level. Use "make run LOG=xxx" to change the level.
## Call flow
info macro => log macro, where log level is checked => log function => print_in_color macro => write function in MyStdout.

