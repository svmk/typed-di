use std::error::Error;
use std::fmt;

pub struct BuildError {
    display: String,
    debug: String,   
}

impl BuildError {
    pub fn from_error(error: impl fmt::Display + fmt::Debug) -> BuildError {
        let display = format!("{}", error);
        let debug = format!("{:?}", error);
        return BuildError {
            display,
            debug,
        }
    }
}

impl fmt::Display for BuildError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.display)
    }
}

impl fmt::Debug for BuildError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.debug)
    }
}

impl Error for BuildError {
    
}