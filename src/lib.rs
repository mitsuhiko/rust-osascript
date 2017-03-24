//! This library implements limited functionality for the OSA System on macOS.
//! In particular it allows you to execute JavaScript via the OSA system to
//! script applications.  It's particularly useful if you need to tell other
//! applications to execute certain functionality.
//!
//! Currently only JavaScript is supported.  Parameters passed to it show up
//! as `$params` and the return value from the script (as returned with the
//! `return` keyword) is deserialized later.
//!
//! # Example
//!
//! ```
//! extern crate osascript;
//! #[macro_use] extern crate serde_derive;
//! 
//! #[derive(Serialize)]
//! struct AlertParams {
//!     title: String,
//!     message: String,
//!     alert_type: String,
//!     buttons: Vec<String>,
//! }
//! 
//! #[derive(Deserialize)]
//! struct AlertResult {
//!     #[serde(rename="buttonReturned")]
//!     button: String,
//! }
//! 
//! fn main() {
//!     let script = osascript::JavaScript::new("
//!         var App = Application('Finder');
//!         App.includeStandardAdditions = true;
//!         return App.displayAlert($params.title, {
//!             message: $params.message,
//!             'as': $params.alert_type,
//!             buttons: $params.buttons,
//!         });
//!     ");
//! 
//!     let rv: AlertResult = script.execute_with_params(AlertParams {
//!         title: "Shit is on fire!".into(),
//!         message: "What is happening".into(),
//!         alert_type: "critical".into(),
//!         buttons: vec![
//!             "Show details".into(),
//!             "Ignore".into(),
//!         ]
//!     }).unwrap();
//! 
//!     println!("You clicked '{}'", rv.button);
//! }
//! ```
use std::process;
use std::io;
use std::string::FromUtf8Error;
use std::io::Write;

extern crate serde;
extern crate serde_json;
#[macro_use] extern crate serde_derive;

use serde::{Serialize, Deserialize};

/// The error from the script system
#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    Json(serde_json::Error),
    Script(String),
}

/// Holds an apple flavoured JavaScript
pub struct JavaScript {
    code: String,
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Io(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Error {
        Error::Json(err)
    }
}

impl From<FromUtf8Error> for Error {
    fn from(err: FromUtf8Error) -> Error {
        Error::Script(format!("UTF-8 Error: {}", err))
    }
}

#[derive(Serialize)]
struct EmptyParams {}

fn wrap_code<S: Serialize>(code: &str, params: S) -> Result<String, Error> {
    let mut buf: Vec<u8> = vec![];
    write!(&mut buf, "var $params = ")?;
    serde_json::to_writer(&mut buf, &params)?;
    write!(&mut buf, ";JSON.stringify((function() {{{};return null;}})());", code)?;
    Ok(String::from_utf8(buf)?)
}

impl JavaScript {
    /// Creates a new script from the given code.
    pub fn new(code: &str) -> JavaScript {
        JavaScript {
            code: code.to_string(),
        }
    }

    /// Executes the script and does not pass any arguments.
    pub fn execute<D: Deserialize>(&self) -> Result<D, Error> {
        self.execute_with_params(EmptyParams {})
    }

    /// Executes the script and passes the provided arguments.
    pub fn execute_with_params<S: Serialize, D: Deserialize>(&self, params: S)
        -> Result<D, Error>
    {
        let wrapped_code = wrap_code(&self.code, params)?;
        let output = process::Command::new("osascript")
            .arg("-l")
            .arg("JavaScript")
            .arg("-e")
            .arg(&wrapped_code)
            .output()?;
        if output.status.success() {
            Ok(serde_json::from_slice(&output.stdout)?)
        } else {
            Err(Error::Script(String::from_utf8(output.stderr)?))
        }
    }
}
