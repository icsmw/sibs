mod actions;
mod error;
mod params;
mod processing;
mod scenario;

pub(crate) use actions::*;
pub(crate) use boxed::boxed;
pub(crate) use error::*;
pub(crate) use params::*;
pub(crate) use processing::*;
pub(crate) use scenario::*;

#[tokio::main]
async fn main() -> Result<(), E> {
    // Get all actions from parameters
    let actions = Parameters::actions()?;
    // Validate actions
    actions
        .iter()
        .map(|act| act.validate(&actions))
        .collect::<Result<Vec<()>, _>>()?;
    // Collect artifacts
    let artifacts: Vec<ActionArtifact> = actions
        .iter()
        .map(|act| act.artifact(&actions))
        .collect::<Result<Vec<Vec<_>>, _>>()?
        .into_iter()
        .flatten()
        .collect();
    // Run actions without context
    actions
        .iter()
        .map(|act| act.no_context_run(&artifacts))
        .collect::<Result<Vec<()>, _>>()?;
    // Run actions with context
    for act in actions.into_iter() {
        act.context_run(artifacts.clone()).await?;
    }
    Ok(())
}
