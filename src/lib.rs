#![warn(missing_docs)]
#![warn(rustdoc::missing_doc_code_examples)]
//! A small library with some convenient string helpers.
//!
//! ```
//! use lngh_strings::{WriteStringExt,WriteLnStringExt,WriteDebugStringExt,WriteLnDebugStringExt};
//!
//! let pink = (255, 27, 141);
//! let yellow = (255, 218, 0);
//! let blue = (27, 179, 255);
//!
//! let mut string = String::new();
//! string.writeln("hello world!")
//!     .writeln("The pan flag's colours are (in rgb):")
//!     .write("pink: ").writeln_debug(pink)
//!     .write("yellow: ").writeln_debug(yellow)
//!     .write("blue: ").write_debug(blue);
//!
//!let expected =
//!r#"hello world!
//!The pan flag's colours are (in rgb):
//!pink: (255, 27, 141)
//!yellow: (255, 218, 0)
//!blue: (27, 179, 255)"#;
//! assert_eq!(expected, string);
//! ```
use std::fmt::{Debug, Write};

///This an extension trait specifically for appending &str's to strings conveniently.
pub trait WriteStringExt
{
    ///Writes a str to a string. Can be chained.
    ///Will panic in the circumstance circumstance that write! fails.
    /// Example:
    /// ```
    /// use lngh_strings::WriteStringExt;
    ///
    /// let mut string = String::new();
    /// string.write("hello")
    ///     .write(" world!");
    /// assert_eq!("hello world!".to_string(), string);
    /// ```
    fn write(&mut self, str:&str) -> &mut Self;
}


impl WriteStringExt for String{
    fn write(&mut self ,str: &str) ->&mut  String {
        write!(self, "{}", str).expect("failed to push to string!");
        self
    }
}

///This an extension trait specifically for appending to strings conveniently.
pub trait WriteLnStringExt{
    ///Writes a str to a string. Can be chained.
    ///Will panic in the circumstance circumstance that writeln! fails.
    /// Example:
    /// ```
    /// use lngh_strings::WriteLnStringExt;
    ///
    /// let mut string = String::new();
    /// string.writeln("hello")
    ///     .writeln("world!");
    /// assert_eq!("hello\nworld!\n".to_string(), string);
    /// ```
    fn writeln(&mut self, str:&str) -> &mut Self;
}

impl WriteLnStringExt for String{
    fn writeln(&mut self, str: &str) ->&mut  String {
        writeln!(self, "{}", str).expect("failed to push to string!");
        self
    }
}


///This an extension trait specifically for appending items with the Debug trait to strings conveniently.
pub trait WriteDebugStringExt<T>
    where T: Debug
{
    ///Gets the debug formatting for an item and appends it to the string. Can be chained.
    /// Will panic in the circumstance circumstance that write! fails.
    /// Example:
    /// ```
    /// use lngh_strings::WriteDebugStringExt;
    ///
    /// let mut string = String::new();
    /// let item  = (1,'a');
    /// string.write_debug(item)
    /// .write_debug(item);
    /// assert_eq!("(1, 'a')(1, 'a')".to_string(), string);
    ///
    /// ```
    fn write_debug(&mut self, t: T) -> &mut String;
}

impl<T> WriteDebugStringExt<T> for String
    where T: Debug
{
    fn write_debug(&mut self, t: T) -> &mut String {
        write!(self, "{:?}",t).expect("failed to push to string!");
        self
    }
}


///This an extension trait specifically for appending items with the Debug trait and a newline to strings conveniently.
pub trait WriteLnDebugStringExt<T> where T: Debug{
    ///Gets the debug formatting for an item and appends it to the string and then a new line. Can be chained.
    /// Will panic in the circumstance circumstance that write! fails.
    /// Example:
    /// ```
    /// use lngh_strings::WriteLnDebugStringExt;
    ///
    /// let mut string = String::new();
    /// let item  = (1,'a');
    /// string.writeln_debug(item)
    /// .writeln_debug(item);
    /// assert_eq!("(1, 'a')\n(1, 'a')\n".to_string(), string);
    ///
    /// ```
    fn writeln_debug(&mut self, t: T) -> &mut String;
}

impl<T> WriteLnDebugStringExt<T> for String
    where T: Debug
{
    fn writeln_debug(&mut self, t: T) -> &mut String {
        writeln!(self, "{:?}",t).expect("failed to push to string!");
        self
    }
}



//TODO: hide this away in a better place!
#[cfg(test)]
mod tests{
    use crate::WriteStringExt;
    use crate::WriteLnStringExt;
    use crate::WriteDebugStringExt;
    use crate::WriteLnDebugStringExt;

    #[test]
    pub fn write_test(){
        let mut string = String::new();
        string.write("hello")
            .write(" world!");
        assert_eq!("hello world!".to_string(), string);

    }


    #[test]
    pub fn writeln_test(){
        let mut string = String::new();
        string.writeln("hello")
            .writeln("world!");
        assert_eq!("hello\nworld!\n".to_string(), string);
    }

    #[test]
    pub fn write_debug(){
        let mut string = String::new();
        let item  = (1,'a');
        string.write_debug(item)
            .write_debug(item);

        assert_eq!("(1, 'a')(1, 'a')".to_string(), string);
    }

    #[test]
    pub fn writeln_debug(){

        let mut string = String::new();
        let item  = (1,'a');
        string.writeln_debug(item)
        .writeln_debug(item);
        assert_eq!("(1, 'a')\n(1, 'a')\n".to_string(), string);
    }

    #[test]
    pub fn multiple(){
        let pink = (255, 27, 141);
        let yellow = (255, 218, 0);
        let blue = (27, 179, 255);

        let mut string = String::new();
        string.writeln("hello world!")
            .writeln("The pan flag's colours are (in rgb): ")
            .write("pink: ").writeln_debug(pink)
            .write("yellow: ").writeln_debug(yellow)
            .write("blue: ").write_debug(blue);
        assert_eq!("hello world!\nThe pan flag's colours are (in rgb): \npink: (255, 27, 141)\nyellow: (255, 218, 0)\nblue: (27, 179, 255)", string);
    }
}