use crate::{cli::{args::{Argument, Description}, error::E, reporter::{Reporter, Display}, location::Location}, reader::entry::Component};

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
    fn action(&mut self, components: &[Component], reporter: &mut Reporter, location: &Location) -> Result<(), E> {
        reporter.bold("SCENARIO:\n");
        reporter.step_right();
        reporter.print(&format!(
            "{}{}\n\n",
            reporter.offset(),
            location.filename.to_str().unwrap()
        ));
        reporter.step_left();
        let with_context = components
            .iter()
            .filter(|comp| comp.cwd.is_some())
            .map(|comp| {
                (
                    comp.name.clone(),
                    comp.meta
                        .as_ref()
                        .map(|meta| meta.as_string())
                        .unwrap_or_default(),
                )
            })
            .collect::<Vec<(String, String)>>();
        if !with_context.is_empty() {
            reporter.bold("COMPONENTS:\n");
            reporter.step_right();
            reporter.pairs(with_context);
            reporter.step_left();
        }
        if components.iter().any(|comp| comp.cwd.is_none()) {
            reporter.bold("\nCOMMANDS:\n");
        }
        reporter.step_right();
        components
            .iter()
            .filter(|comp| comp.cwd.is_none())
            .for_each(|comp| {
                comp.tasks.iter().filter(|t| t.has_meta()).for_each(|task| {
                    task.display(reporter);
                });
            });
        reporter.step_left();
        Ok(())
    }
}
