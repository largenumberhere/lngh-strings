#![warn(missing_docs)]
#![warn(missing_doc_code_examples)]
//! A small library with some string helpers
use std::fmt::Write;

///Writes a str to a string.
///
/// Example:
/// ```
/// let mut string = String::new();
/// string.write("hello world!");
/// assert_eq!("hello world!".to_string(), string);
/// ```
pub trait WriteStringExt
{
    ///Please see trait documentation
    fn write(&mut self, str:&str) -> &mut Self;
}


impl WriteStringExt for String{
    fn write(&mut self, str: &str) ->&mut  String {
        write!(self, "{}", str).expect("failed to push to string!");
        self
    }
}
///Writes a str and \n to a string.
/// It is a very thin wrapper for the writeln! macro so it can be appended to strings
///
/// Example:
///```
/// let mut string = String::new();
/// string.writeln("hello world!");
/// assert_eq!("hello world!\n".to_string(), string);
/// ```
pub trait WriteLnStringExt{
    ///Please see trait documentation
    fn writeln(&mut self, str:&str) -> &mut Self;
}

impl WriteLnStringExt for String{

    fn writeln(&mut self, str: &str) ->&mut  String {
        writeln!(self, "{}", str).expect("failed to push to string!");
        self
    }
}



//TODO: hide this away in a better place!
#[cfg(test)]
mod tests{
    use crate::WriteStringExt;
    use crate::WriteLnStringExt;

    #[test]
    pub fn write_test(){
        let mut string = String::new();
        string.write("hello world!");
        assert_eq!("hello world!".to_string(), string);
    }


    #[test]
    pub fn writeln_test(){
        let mut string = String::new();
        string.writeln("hello world!");
        assert_eq!("hello world!\n".to_string(), string);
    }
}