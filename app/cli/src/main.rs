mod actions;
mod error;
mod params;
mod script;

pub(crate) use actions::*;
pub(crate) use error::*;
pub(crate) use params::*;
pub(crate) use scenario::*;
pub(crate) use script::*;

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
    // Run actions
    let mut post_actions = Vec::new();
    for act in actions.into_iter() {
        post_actions.push(act.run(artifacts.clone())?);
    }
    if post_actions
        .iter()
        .any(|art| matches!(art, RunArtifact::Lsp))
    {
        if post_actions
            .iter()
            .any(|art| !matches!(art, RunArtifact::Lsp) && !matches!(art, RunArtifact::Void))
        {
            return Err(E::SelfishLts);
        }
    }
    // Run post actions, if exists
    for artifact in post_actions.into_iter() {
        match artifact {
            RunArtifact::Script(mut script) => {
                let _ = script.run().await?;
            }
            RunArtifact::Lsp => {
                lsp::run().await;
            }
            RunArtifact::Void => {}
        }
    }
    Ok(())
}
