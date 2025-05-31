// TODO: switch to Driver

use asttree::*;
use interpreter::*;
use parser::*;
use runtime::*;
use semantic::*;

use uuid::Uuid;

use crate::*;

pub struct Script {
    component: Option<String>,
    task: Option<String>,
    args: Option<Vec<String>>,
    scenario: Scenario,
    anchor: Anchor,
    scx: Option<SemanticCx>,
    parser: Parser,
}

impl Script {
    pub fn new(
        scenario: Scenario,
        component: Option<String>,
        task: Option<String>,
        args: Option<Vec<String>>,
    ) -> Result<Self, E> {
        let mut parser = Parser::new(&scenario.filepath, false)?;
        let anchor = Anchor::read(&mut parser);
        if let Err(err) = &anchor {
            eprintln!("{}", parser.report_err(err)?);
        }
        let anchor = anchor?.ok_or(E::FailExtractAnchorNodeFrom(
            scenario.filepath.to_string_lossy().to_string(),
        ))?;
        let mut scx = SemanticCx::new(false);
        functions::register(&mut scx.fns.efns)?;
        if let Err(err) = anchor.initialize(&mut scx) {
            eprintln!("{}", parser.report_err(&err)?);
            return Err(err.into());
        }
        if let Err(err) = anchor.infer_type(&mut scx) {
            eprintln!("{}", parser.report_err(&err)?);
            return Err(err.into());
        }
        if let Err(err) = anchor.finalize(&mut scx) {
            eprintln!("{}", parser.report_err(&err)?);
            return Err(err.into());
        }
        Ok(Self {
            scenario,
            anchor,
            scx: Some(scx),
            parser,
            component,
            task,
            args,
        })
    }

    pub async fn run(&mut self) -> Result<RtValue, E> {
        let component = self.component.take().ok_or(E::ScriptAlreadyExecuted)?;
        let task = self.task.take().ok_or(E::ScriptAlreadyExecuted)?;
        let scx = self.scx.take().ok_or(E::ScriptAlreadyExecuted)?;
        let args = self.args.take().ok_or(E::ScriptAlreadyExecuted)?;
        let params = RtParameters::new(component.clone(), task.clone(), args, self.scenario.cwd()?);
        let rt = interpreter::runtime(params, scx)?;
        let cx = rt
            .create_cx(Uuid::new_v4(), format!("{component}:{task}"), None)
            .await?;
        let vl = self.anchor.interpret(rt.clone(), cx).await;
        let _ = rt.destroy().await;
        match vl {
            Ok(vl) => Ok(vl),
            Err(err) => {
                eprintln!(
                    "{}",
                    self.parser
                        .report_err(&err)
                        .map_err(|err| RtError::Other(err.to_string()))?
                );
                Err(err.into())
            }
        }
    }
    pub fn print(&self) -> Result<(), E> {
        if self.component.is_some() {
            self.print_tasks()
        } else {
            self.print_components()
        }
    }

    fn print_components(&self) -> Result<(), E> {
        let mut lines = Vec::new();
        self.anchor
            .get_components_md()
            .iter()
            .for_each(|(component, (md, tasks))| {
                lines.push(format!("- [b]{component}[/b][>>]"));
                lines.extend(md.lines().into_iter().map(|ln| format!("  {ln}")));
                tasks.iter().for_each(|(task, meta)| {
                    lines.push(format!("[>>]- [b]{task}[/b]"));
                    lines.extend(meta.lines().into_iter().map(|ln| format!("[>>]{ln}")));
                });
            });
        term::print(lines.join("\n"));
        Ok(())
    }

    fn print_tasks(&self) -> Result<(), E> {
        let Some(component) = self.component.clone() else {
            return Err(E::NoComponentParameter);
        };
        let mut lines = vec![format!("[b]{component}[/b]")];
        let Some(component) = self.anchor.get_component(&component) else {
            return Err(E::ComponentNotFound(component));
        };
        lines.extend(component.get_md().lines());
        if let Node::Root(Root::Component(component)) = component.get_node() {
            component.get_tasks_md().iter().for_each(|(task, meta)| {
                lines.push(format!(" - [b]{task}[/b][>>]"));
                lines.extend(meta.lines().into_iter().map(|ln| format!("[>>]{ln}")));
            });
        }
        term::print(lines.join("\n"));
        Ok(())
    }
}
