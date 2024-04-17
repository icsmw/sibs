use crate::error::{LinkedErr, LinkedErrSerialized};
use console::Style;
use std::fmt;

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
}

impl Report {
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
            Self::Trace { .. } => {
                panic!("Footprint: Not implemented!")
            }
        }
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
