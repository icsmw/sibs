use crate::{
    cli::{
        args::{Argument, Description}, error::E 
    },
    inf::{
        term::Display,
        context::Context,
    }, 
    entry::Component
};

const ARGS: [&str; 2] = ["--help", "-h"];

#[derive(Debug)]
pub struct Help {
    component: Option<String>,
}

impl Argument<Help> for Help {
    fn read(args: &mut Vec<String>) -> Result<Option<Help>, E> {
        Self::find_prev_to_opt(args, &ARGS).map(|component| component.map(|component| Self {
            component,
        }))
    }
    fn desc() -> Description {
        Description { 
            key: ARGS.iter().map(|s|s.to_string()).collect::<Vec<String>>(),
            desc: String::from("shows help. Global cx - shows available options and components. To get help for component use: component --help."),            pairs: vec![],

        }
    }
    fn action(&mut self, components: &[Component], cx: &mut Context) -> Result<(), E> {
        fn list_components(components: &[Component], cx: &mut Context) {
            let with_context = components
            .iter()
            .filter(|comp| comp.cwd.is_some())
            .map(|comp| {
                (
                    comp.name.to_string(),
                    comp.get_meta().iter()
                        .map(|meta| meta.as_string())
                        .collect::<Vec<String>>().join("\n"),
                )
            })
            .collect::<Vec<(String, String)>>();
            if !with_context.is_empty() {
                cx.term.bold("COMPONENTS:\n");
                cx.term.step_right();
                cx.term.pairs(with_context);
                cx.term.step_left();
            }
        }
        fn list_commands(components: &[Component], cx: &mut Context) {
            if components.iter().any(|comp| comp.cwd.is_none()) {
                cx.term.bold("\nCOMMANDS:\n");
            }
            cx.term.step_right();
            components
                .iter()
                .filter(|comp| comp.cwd.is_none())
                .for_each(|comp| {
                    comp.get_tasks().iter().filter(|t| t.has_meta()).for_each(|task| {
                        task.display(&mut cx.term);
                    });
                });
                cx.term.step_left();
        }
        cx.term.bold("SCENARIO:\n");
        cx.term.step_right();
        cx.term.print(format!(
            "{}{}\n\n",
            cx.term.offset(),
            cx.scenario.filename.to_str().unwrap()
        ));
        cx.term.step_left();
        if let Some(component) = self.component.as_ref() {
            if let Some(component) = components.iter().find(|c| &c.name.to_string() == component) {
                component.display(&mut cx.term);
            } else {
                cx.term.err(format!("Component \"{component}\" isn't found.\n\n"));
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
