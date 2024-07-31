use crate::{
    cli::{
        args::{Action, ActionPinnedResult, Argument, Description},
        error::E,
    },
    elements::Element,
    inf::{term, AnyValue},
};

const ARGS: [&str; 2] = ["--help", "-h"];

#[derive(Debug, Clone)]
pub struct Help {
    component: Option<String>,
}

impl Argument for Help {
    fn key() -> String {
        ARGS[0].to_owned()
    }
    fn read(args: &mut Vec<String>) -> Result<Option<Box<dyn Action>>, E> {
        if let (true, component) = Self::with_prev(args, &ARGS)? {
            Ok(Some(Box::new(Self { component })))
        } else {
            Ok(None)
        }
    }
    fn desc() -> Description {
        Description {
            key: ARGS.iter().map(|s| s.to_string()).collect::<Vec<String>>(),
            desc: "Shows help or available options and components. To get help for component use."
                .to_string(),
        }
    }
}

impl Action for Help {
    fn key(&self) -> String {
        ARGS[0].to_owned()
    }
    fn action<'a>(&'a self, components: &'a [Element]) -> ActionPinnedResult {
        Box::pin(async move {
            if let Some(component) = self.component.as_ref() {
                if let Some((el, md)) = components.iter().find_map(|el| {
                    if let Element::Component(el, md) = el {
                        if &el.name.value == component {
                            Some((el, md))
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                }) {
                    let mut output = format!(
                        "COMPONENT: [b]{}[/b]\n{}\n\nTASKS:\n",
                        el.name,
                        md.meta()
                            .iter()
                            .map(|m| m.as_string())
                            .collect::<Vec<String>>()
                            .join(" ")
                    );
                    el.elements.iter().for_each(|el| {
                        if let Element::Task(el, md) = el {
                            let mut meta = md.meta_as_lines();
                            let first = if meta.is_empty() {
                                String::new()
                            } else {
                                format!("{}\n", meta.remove(0))
                            };
                            let mut details = String::new();
                            let declarations = el
                                .declarations
                                .iter()
                                .filter_map(|el| {
                                    if let Element::VariableDeclaration(el, _) = el {
                                        details.push_str(&format!(
                                            "[>>][b][color:blue]{}[/color][/b]:[>>] {}\n",
                                            el.variable, el.declaration
                                        ));
                                        Some(el.variable.to_string())
                                    } else {
                                        None
                                    }
                                })
                                .collect::<Vec<String>>()
                                .join(" ");
                            let dependencies = el
                                .dependencies
                                .iter()
                                .filter_map(|el| {
                                    if let Element::Reference(el, _) = el {
                                        Some(format!("        {el}"))
                                    } else {
                                        None
                                    }
                                })
                                .collect::<Vec<String>>()
                                .join("\n");
                            output.push_str(&format!(
                                "    [b]{}{}[/b] [>>]{first}{}\n{}",
                                el.name,
                                if declarations.is_empty() {
                                    String::new()
                                } else {
                                    format!(" [color:blue]{declarations}[/color]")
                                },
                                meta.iter()
                                    .map(|m| format!("[>>]{m}",))
                                    .collect::<Vec<String>>()
                                    .join("\n"),
                                if meta.is_empty() {
                                    details.as_str()
                                } else {
                                    "\n"
                                }
                            ));
                            if !dependencies.is_empty() {
                                output.push_str(&format!("    Depend on:\n{dependencies}"));
                            }
                        }
                    });
                    term::print(&output);
                } else {
                    term::print(format!("Component [b]\"{component}\"[/b] isn't found."));
                    return Err(E::ComponentNotExists(component.to_string()));
                }
            } else {
                let mut output = String::from("COMPONENTS:\n");
                components.iter().for_each(|el| {
                    if let Element::Component(el, md) = el {
                        output.push_str(&format!(
                            "    [b]{}[/b] [>>]{}\n\n",
                            el.name,
                            md.meta()
                                .iter()
                                .map(|m| m.as_string())
                                .collect::<Vec<String>>()
                                .join(" ")
                        ));
                    }
                });
                term::print(&output);
            }
            Ok(AnyValue::empty())
        })
    }
}
