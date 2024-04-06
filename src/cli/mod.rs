mod args;
pub mod error;

use crate::{
    elements::{Component, Element},
    inf::{context::Context, operator::Operator, scenario::Scenario, term, tracker},
    reader,
};
use args::Arguments;
use error::E;
use std::{
    env::{self, current_dir},
    path::PathBuf,
};

use self::args::Argument;

fn get_arguments() -> Result<(Vec<String>, Arguments), E> {
    let mut income = env::args().collect::<Vec<String>>();
    if !income.is_empty() {
        income.remove(0);
    }
    let args = Arguments::new(&mut income)?;
    Ok((income, args))
}

pub async fn get_tracker_configuration() -> Result<tracker::Configuration, E> {
    let (_, arguments) = get_arguments()?;
    Ok(tracker::Configuration {
        output: arguments
            .get_value_no_cx::<args::exertion::Output, tracker::Output>()
            .await?
            .unwrap_or(tracker::Output::Progress),
        log_file: arguments
            .get_value_no_cx::<args::exertion::LogFile, PathBuf>()
            .await?,
        trace: arguments
            .get_value_no_cx::<args::exertion::Trace, bool>()
            .await?
            .unwrap_or(false),
    })
}

pub async fn read(cx: &mut Context) -> Result<(), E> {
    let (mut income, arguments) = get_arguments()?;
    if arguments.all_without_context().await? {
        if !income.is_empty() {
            term::print(&format!(
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
    cx.set_scenario(scenario);
    let scenario = cx.scenario.filename.to_owned();
    let elements = match reader::read_file(cx, scenario, true).await {
        Ok(elements) => elements,
        Err(err) => {
            cx.sources.report_error(&err)?;
            return Err(E::ReaderError(err.e));
        }
    };
    let no_actions = arguments.has::<args::exertion::Help>() || income.is_empty();
    arguments.run::<args::exertion::Help>(&elements, cx).await?;
    if no_actions {
        return Ok(());
    }
    let components = elements
        .into_iter()
        .filter_map(|el| {
            if let Element::Component(c, _) = el {
                Some(c)
            } else {
                None
            }
        })
        .collect::<Vec<Component>>();
    if let Some(component) = if components.is_empty() {
        None
    } else {
        Some(income.remove(0))
    } {
        if let Some(component) = components
            .iter()
            .find(|comp| comp.name.to_string() == component)
        {
            component
                .execute(Some(component), &components, &income, cx)
                .await
                .map_err(|e| {
                    // cx.sources.report_error(&0, &e);
                    e.e
                })?;
            Ok(())
        } else {
            Err(E::ComponentNotExists(component.to_string()))
        }
    } else {
        Err(E::NoArguments)
    }
}
