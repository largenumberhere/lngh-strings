use std::fmt::Write;

pub trait WriteStringExt{
    fn write(&mut self, str:&str) -> &mut Self;
}

impl WriteStringExt for String{
    ///Writes a str to a string.
    /// It is a very thin wrapper for the write! macro so it can be appended to strings
    fn write(&mut self, str: &str) ->&mut  String {
        write!(self, "{}", str).expect("failed to push to string!");
        self
    }
}

pub trait WriteLnStringExt{
    fn writeln(&mut self, str:&str) -> &mut Self;
}

impl WriteLnStringExt for String{
    ///Writes a str and \n to a string.
    /// It is a very thin wrapper for the writeln! macro so it can be appended to strings
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