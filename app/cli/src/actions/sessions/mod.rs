mod header;
mod rows;
mod table;

use crate::*;
use runtime::JournalReader;
use uuid::Uuid;

use header::*;
use table::*;

pub struct SessionsAction {
    session: Option<String>,
}

impl SessionsAction {
    pub fn new(session: Option<String>) -> Result<Self, E> {
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
        let mut reader = JournalReader::new(&scenario.cwd()?)?;
        if let Some(session) = self.session.as_ref() {
            let uuid = match Uuid::parse_str(session) {
                Ok(uuid) => uuid,
                Err(err) => {
                    let sessions = reader.list();
                    sessions
                        .keys()
                        .find(|key| key.to_string().starts_with(session))
                        .cloned()
                        .ok_or(E::InvalidUuid(err.to_string()))?
                }
            };
            if let Some(count) = reader.open(&uuid)? {
                println!("Session {} has {} records", session, count);
                rows::render(&mut reader, &uuid);
            } else {
                println!("Session {} not found", session);
            }
        } else {
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
