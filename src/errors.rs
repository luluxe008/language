#![allow(dead_code)]

use std::fmt::Display;

pub type CompilerResult<T> = Result<T, Vec<Error>>;


/// Represent all the type of Error: Error, Warning and Note
/// Only Error stop the compilation
#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum ErrorType{
    Error,
    Warning,
    Note
}

impl ToString for ErrorType{
    fn to_string(&self) -> String {
        match self {
            ErrorType::Error => String::from("error"),
            ErrorType::Warning => String::from("warning"),
            ErrorType::Note => String::from("note"),
        }
    }
}


/// Indicate the filename and the line which is beging compiled.
/// It is not necessary use in error. 
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct PartialLocation{ 
    filename: String,
    line: u64
}

impl PartialLocation{
    /// create a new PartialLocation
    pub fn new(filename: impl Into<String>, line: u64) -> Self{
        PartialLocation {
            filename: filename.into(),
            line,
        }
    }

    /// create a PartialLocation with stdin settings
    pub fn stdin(line: u64) -> Self{
        Self { filename: "stdin".into(), line}
    }

    /// create a PartialLocation with not-specified settings
    pub fn not_specified(line: u64) -> Self{
        Self { filename: "not specified".into(), line}
    }

    /// create a PartialLocation with testing settings
    pub fn testing(line: u64) -> Self{
        Self { filename: "test".into(), line}
    }
}

/// Indicate the filename, the line and the char of an error.
/// Its main use is for error
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Location{
    filename: String, //no need to mutate this string
    line: u64,
    char_pos: u32
}

impl Location{
    /// create a new Location
    pub fn new(filename: impl Into<String>, line: u64, char: u32) -> Self{
        Location{
            filename: filename.into(),
            line,
            char_pos: char
        }
    }

    /// edit line
    /// can be chained
    pub fn line(mut self, line: u64) -> Self{
        self.line = line;
        self
    }

    /// edit char position
    /// can be chained
    pub fn char_pos(mut self, char_pos: u32) -> Self{
        self.char_pos = char_pos;
        self
    }

    /// edit filename
    /// can be chained
    pub fn filename(mut self, filename: impl Into<String>) -> Self{
        self.filename = filename.into();
        self
    }
}


impl From<PartialLocation> for Location{
    ///Convert PartialLocation to Location
    fn from(value: PartialLocation) -> Self {
        //that way, we can easely convert PartialLocation to Location
        Location {
            filename: value.filename,
            line: value.line ,
            char_pos:0,
         }
    }
}


/// An error. Can be an Error, a Warning or a Note
/// In all case it has the location of the problem, the general name of the problem, a description of the problem and the line of code where the problem
/// happened.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Error{
    err_type: ErrorType,
    location: Location,
    name: String,
    desc: String,
    line: String
}

impl Error{
    /// create a new Error.
    /// 
    /// Don't use this method (that's why it is not public)
    /// Use one of the following method to create better error
    fn new<S>(err_type: ErrorType, location:Location, name: S, desc: S, line: S) -> Self
    where S: Into<String> 
    {
        Error { 
            err_type, 
            location, 
            name: name.into(), 
            desc: desc.into(), 
            line: line.into() 
        }
    }

    /// create a new (general) syntax error.
    pub fn syntax_error<S>(location:Location, line: S) -> Self
    where S: Into<String> 
    {

        Self::new(ErrorType::Error, location, "SyntaxError", "Uncorrect syntax", line.into().as_str())
    }


    /// create a string closing error. It indicates that a string was not closed.
    pub fn string_closing<S>(location:Location, line: S) -> Self
    where S: Into<String> 
    {

        Self::new(ErrorType::Error, location, "StringClosingError", "A String litteral was not closed", line.into().as_str())
    }

    /// create a illegal character error. It indicates that a illegal charactrer was encountred
    pub fn illegal_character<S>(location:Location, line: S, char: char) -> Self
    where S: Into<String> 
    {

        Self::new(ErrorType::Error, location, "IllegalCharacter",
         &format!("An illegal character of code [{}] was encountred", char as u64),
          line.into().as_str())
    }

    /// create a excepted token error. It indicates that an excepted token was not found
    pub fn excepted_token<S>(location:Location, line: S, excepted: S) -> Self
    where S: Into<String> {
        Self::new(ErrorType::Error, location, "ExceptedToken", format!("Excepted Token [{}]", excepted.into()).as_str(), line.into().as_str())
    }

    /// create a unexcepted token error. It indicates that an unexcepted token was found
    pub fn unexcepted_token<S>(location:Location, line: S, token: S) -> Self
    where S: Into<String> {
        Self::new(ErrorType::Error, location, "UnexceptedToken", format!("Unexpected Token [{}]", token.into()).as_str(), line.into().as_str())
    }


    /// create a floating number error. The compiler does not support floating number.
    pub fn floating_numer<S>(location:Location, line: S) -> Self
    where S: Into<String> 
    {

        Self::new(ErrorType::Error, location, "FloatingNumber", "Floating Number are not supported.", line.into().as_str())
    }

}



impl Display for Error{
    /// basic error output
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {} in {} at {}:{}\n|\t{}\n|\t{}\n{}", 
        self.err_type.to_string(),
        self.name,
        self.location.filename,
        self.location.line,
        self.location.char_pos,
        self.line,
        " ".repeat(self.location.char_pos as usize) + "^",
        self.desc
     )
    }
}


/// print errors
pub fn display_errors(errs: Vec<Error>){
    for err in errs{
        println!("{err}\n")
    }
}
