use crate::cli::{args::{Argument, Description}, error::E};
const ARGS: [&str; 2] = ["--help", "-h"];

#[derive(Debug)]
pub struct Help {
    component: Option<String>,
}

impl Help {
    pub fn context(&self) -> Option<&String> {
        self.component.as_ref()
    }
}


impl Argument<Help> for Help {
    fn read(args: &mut Vec<String>) -> Result<Option<Help>, E> {
        for (i, arg) in args.iter().enumerate() {
            if ARGS.contains(&arg.as_str()) {
                if i <= 1 {
                    args.drain(0..=i);
                    return Ok(Some(Self {
                        component: if i == 0 { None } else { Some(args.remove(0)) },
                    }));
                } else {
                    return Err(E::InvalidHelpRequest);
                }
            }
        }
        Ok(None)
    }
    fn desc() -> Description {
        Description { 
            key: ARGS.iter().map(|s|s.to_string()).collect::<Vec<String>>(),
            desc: String::from("shows help. Global context - shows available options and components. To get help for component use: component --help.")
        }
    }
}
