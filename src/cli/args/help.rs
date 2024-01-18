use crate::{
    cli::{
        args::{Argument, Description}, error::E 
    },
    inf::{
        reporter::{Display},
        context::Context,
    }, 
    reader::entry::Component
};

const ARGS: [&str; 2] = ["--help", "-h"];

#[derive(Debug)]
pub struct Help {
    component: Option<String>,
}

impl Argument<Help> for Help {
    fn read(args: &mut Vec<String>) -> Result<Option<Help>, E> {
        for (i, arg) in args.iter().enumerate() {
            if ARGS.contains(&arg.as_str()) {
                if i <= 1 {
                    let component = if i == 0 { None } else { Some(args[i - 1].clone()) };
                    args.drain(0..=i);
                    return Ok(Some(Self {
                        component,
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
            desc: String::from("shows help. Global cx - shows available options and components. To get help for component use: component --help.")
        }
    }
    fn action(&mut self, components: &[Component], cx: &mut Context) -> Result<(), E> {
        fn list_components(components: &[Component], cx: &mut Context) {
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
                cx.reporter.bold("COMPONENTS:\n");
                cx.reporter.step_right();
                cx.reporter.pairs(with_context);
                cx.reporter.step_left();
            }
        }
        fn list_commands(components: &[Component], cx: &mut Context) {
            if components.iter().any(|comp| comp.cwd.is_none()) {
                cx.reporter.bold("\nCOMMANDS:\n");
            }
            cx.reporter.step_right();
            components
                .iter()
                .filter(|comp| comp.cwd.is_none())
                .for_each(|comp| {
                    comp.tasks.iter().filter(|t| t.has_meta()).for_each(|task| {
                        task.display(&mut cx.reporter);
                    });
                });
                cx.reporter.step_left();
        }
        cx.reporter.bold("SCENARIO:\n");
        cx.reporter.step_right();
        cx.reporter.print(format!(
            "{}{}\n\n",
            cx.reporter.offset(),
            cx.scenario.filename.to_str().unwrap()
        ));
        cx.reporter.step_left();
        if let Some(component) = self.component.as_ref() {
            if let Some(component) = components.iter().find(|c| &c.name == component) {
                component.display(&mut cx.reporter);
            } else {
                cx.reporter.err(format!("Component \"{component}\" isn't found.\n\n"));
                list_components(components, cx);
                return Err(E::ComponentNotExists(component.to_string()));
            }
        } else {
            list_components(components, cx);
            list_commands(components, cx);
        }
        Ok(())
    }
}
