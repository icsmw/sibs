mod header;
mod table;

use crate::*;
use runtime::JournalReader;
use uuid::Uuid;

use header::*;
use table::*;

pub struct SessionsAction {
    session: Option<Uuid>,
}

impl SessionsAction {
    pub fn new(session: Option<String>) -> Result<Self, E> {
        let session = session
            .map(|s| Uuid::parse_str(&s).map_err(|e| E::InvalidUuid(e.to_string())))
            .transpose()?;
        Ok(Self { session })
    }
}

impl ActionMethods for SessionsAction {
    fn validate(&self, actions: &[Action]) -> Result<(), E> {
        if actions.len() != 1 {
            Err(E::StandaloneParameter(
                Parameters::Sessions.key().join(", ").to_string(),
            ))
        } else {
            Ok(())
        }
    }
    fn run(&self, artifacts: &mut Vec<ActionArtifact>) -> Result<RunArtifact, E> {
        let scenario = if let Some(ActionArtifact::Scenario(scenario)) = artifacts
            .iter()
            .find(|art| matches!(art, ActionArtifact::Scenario(..)))
            .cloned()
        {
            scenario
        } else {
            Scenario::new()?
        };
        if let Some(session) = self.session {
            // todo: implement session handling
        } else {
            let mut reader = JournalReader::new(&scenario.cwd()?)?;
            let sessions = reader.list();
            let opts = TableOptions::default().analize(sessions.values());
            let mut table = Table::default();
            HEADERS
                .iter()
                .for_each(|h| table.push_header(h.as_str(&opts)));
            for info in sessions.values() {
                table.push_row(HEADERS.iter().map(|h| h.row(info, &opts)));
            }
            table.print();
        }
        Ok(RunArtifact::Void)
    }
}
