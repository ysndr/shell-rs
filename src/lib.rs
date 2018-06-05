extern crate atty;
extern crate termcolor;
extern crate libc;
#[macro_use]
extern crate error_chain;

pub mod shell;


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        ::shell::Shell::new();
    }
}
