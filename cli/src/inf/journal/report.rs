use crate::error::{LinkedErr, LinkedErrSerialized};
use console::Style;
use std::fmt;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub enum Status {
    Output(Option<String>),
    Error(String),
}

/// Footprint of executing
///
/// - The `String` is fragment of executed code, which return a value.
/// - The `Status` returned result during executing.
pub type Footprint = (String, Status);

#[derive(Debug, Clone)]
pub enum Report {
    /// Report, which includes only error. In most cases reported during
    /// reading
    ///
    /// - The `LinkedErrSerialized` representation of `LindedErr`.
    LinkedErr(LinkedErrSerialized),
    /// Report, which includes map report and error. In most cases reported
    /// during reading
    ///
    /// # Fields
    ///
    /// * `report` is related map report as string.
    /// * `err` representation of `LindedErr`.
    #[allow(clippy::enum_variant_names)]
    Report {
        report: String,
        err: LinkedErrSerialized,
    },
    /// Report with multiple previous to error footprints of executing. This
    /// option is used only during executing.
    ///
    /// # Fields
    ///
    /// * `trace` list of previous to error footprints.
    /// * `report` is related map report as string. can be None in case if
    /// `LinkedErr` isn't bound to any token.
    /// * `err` representation of `LindedErr`.
    Trace {
        trace: Vec<Footprint>,
        report: Option<String>,
        err: LinkedErrSerialized,
    },
    /// Report, which includes only string message of error. As usual is used
    /// during initialization.
    ///
    /// - The `String` is error message.
    String(String),
}

impl Report {
    pub fn err_uuid(&self) -> Option<Uuid> {
        match self {
            Self::LinkedErr(err) => Some(err.uuid),
            Self::Report { report, err } => Some(err.uuid),
            Self::Trace { trace, report, err } => Some(err.uuid),
            _ => None,
        }
    }
    pub fn print(&self) {
        match self {
            Self::LinkedErr(err) => {
                eprintln!(
                    "{} {}",
                    Style::new().red().bold().apply_to("ERROR:"),
                    Style::new().white().apply_to(&err.e)
                );
            }
            Self::Report { report, err: _ } => eprintln!("{report}"),
            Self::Trace { trace, report, err } => {
                trace.iter().for_each(|(fragment, status)| {
                    eprintln!("{fragment}");
                });
                if let Some(report) = report {
                    eprintln!("{report}");
                }
                eprintln!(
                    "{} {}",
                    Style::new().red().bold().apply_to("ERROR:"),
                    Style::new().white().apply_to(&err.e)
                );
            }
            Self::String(err) => {
                eprintln!(
                    "{} {}",
                    Style::new().red().bold().apply_to("ERROR:"),
                    Style::new().white().apply_to(&err)
                );
            }
        }
    }
}

impl From<&str> for Report {
    fn from(val: &str) -> Self {
        Self::String(val.to_owned())
    }
}

impl From<&String> for Report {
    fn from(val: &String) -> Self {
        Self::String(val.to_owned())
    }
}

impl<T: Clone + fmt::Display> From<&LinkedErr<T>> for Report {
    fn from(val: &LinkedErr<T>) -> Self {
        Self::LinkedErr(val.serialize())
    }
}

impl<T: Clone + fmt::Display> From<(String, &LinkedErr<T>)> for Report {
    fn from(val: (String, &LinkedErr<T>)) -> Self {
        Self::Report {
            report: val.0,
            err: val.1.serialize(),
        }
    }
}

impl From<(Vec<Footprint>, Option<String>, LinkedErrSerialized)> for Report {
    fn from(val: (Vec<Footprint>, Option<String>, LinkedErrSerialized)) -> Self {
        Self::Trace {
            trace: val.0,
            report: val.1,
            err: val.2,
        }
    }
}

// impl<T: Clone + fmt::Display> From<(Vec<Footprint>, Option<String>, &LinkedErr<T>)> for Report {
//     fn from(val: (Vec<Footprint>, Option<String>, &LinkedErr<T>)) -> Self {
//         Self::Trace {
//             trace: val.0,
//             report: val.1,
//             err: val.2.serialize(),
//         }
//     }
// }
