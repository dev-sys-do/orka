use colored::*;

/// Display struct, used to handle displaying stuff back to the user
pub struct Display {}

impl Display {
    pub fn print_error(&self, line: &str) {
        self.print_line(line.red())
    }

    pub fn print_log(&self, line: &str) {
        self.print_line(line.white())
    }

    fn print_line(&self, line: ColoredString) {
        println!("{}", line);
    }
}
