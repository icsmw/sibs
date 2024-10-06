pub mod args;
pub mod error;
#[cfg(test)]
mod tests;

use crate::{
    elements::Element,
    inf::{
        context::Context,
        journal::{self, Journal},
        operator::Execute,
        scenario::Scenario,
        term, ExecuteContext, Value,
    },
    reader::{Reader, Sources},
};
use args::Arguments;
use error::E;
use std::{
    env::{self, current_dir},
    path::PathBuf,
};

fn get_arguments() -> Result<(Vec<String>, Arguments), E> {
    let mut income = env::args().collect::<Vec<String>>();
    if !income.is_empty() {
        income.remove(0);
    }
    let args = Arguments::new(&mut income)?;
    Ok((income, args))
}

pub async fn get_journal_configuration() -> Result<journal::Configuration, E> {
    let (_, arguments) = get_arguments()?;
    Ok(journal::Configuration {
        output: arguments
            .get_value_no_cx::<args::exertion::Output, journal::Output>()
            .await?
            .unwrap_or(journal::Output::Progress),
        log_file: arguments
            .get_value_no_cx::<args::exertion::LogFile, PathBuf>()
            .await?,
        trace: arguments
            .get_value_no_cx::<args::exertion::Trace, bool>()
            .await?
            .unwrap_or(false),
        writing: true,
    })
}

pub async fn process(journal: Journal) -> Result<(), E> {
    let (mut income, arguments) = get_arguments()?;
    if arguments.all_without_context().await? {
        if !income.is_empty() {
            term::print(format!(
                r#"[b]WARNING:[\b] Ingore next arguments: {}"#,
                income.join(", ")
            ));
        }
        return Ok(());
    }
    if arguments.len() == 1 && arguments.has::<args::exertion::Help>() {
        Arguments::print();
        return Ok(());
    }
    let scenario = if let Some(target) = arguments
        .get_value_no_cx::<args::exertion::Scenario, PathBuf>()
        .await?
    {
        Scenario::from(&current_dir()?.join(target).canonicalize()?)?
    } else {
        match Scenario::new() {
            Ok(scenario) => scenario,
            Err(_) => {
                term::print("[b]ERROR:[/b] Scenario file hasn't been found.");
                Arguments::print();
                return Ok(());
            }
        }
    };
    let mut src = Sources::new(&journal);
    let elements = match Reader::read_file(&scenario.filename, true, Some(&mut src), &journal).await
    {
        Ok(elements) => elements,
        Err(err) => {
            src.report_err(&err)?;
            return Err(E::ReaderError(err.e));
        }
    };
    let cx = Context::init(scenario, &src, &journal)?;
    let no_actions = arguments.has::<args::exertion::Help>() || income.is_empty();
    arguments.run::<args::exertion::Help>(&elements).await?;
    if no_actions {
        cx.destroy().await?;
        return Ok(());
    }
    let components = elements
        .into_iter()
        .filter(|el| matches!(el, Element::Component(..)))
        .collect::<Vec<Element>>();
    let result = if let Some(component) = if components.is_empty() {
        None
    } else {
        Some(income.remove(0))
    } {
        let Some(component) = components.iter().find(|comp| {
            if let Ok(comp) = comp.as_component() {
                comp.name.to_string() == component
            } else {
                false
            }
        }) else {
            return Err(E::ComponentNotExists(component.to_string()));
        };
        let sc = cx.scope.create("root", None).await?;
        let result = component
            .execute(
                ExecuteContext::unbound(cx.clone(), sc.clone())
                    .owner(Some(component))
                    .components(&components)
                    .args(
                        &income
                            .iter()
                            .map(|v| Value::String(v.to_string()))
                            .collect::<Vec<Value>>(),
                    )
                    .token(cx.aborting.clone()),
            )
            .await
            .map(|_| ())
            .map_err(|e| e.into());
        sc.destroy().await?;
        result
    } else {
        Err(E::NoArguments)
    };
    cx.destroy().await?;
    result
}
