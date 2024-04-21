use std::collections::HashSet;
use uuid::Uuid;

use crate::{
    error::LinkedErrSerialized,
    inf::{
        context::atlas::{Maps, E},
        map::Mapping,
        Footprint, Journal, Status,
    },
};

pub const TRACE_DEFAILT_DEEP: usize = 5;

type FootprintRef = (usize, Option<String>);

#[derive(Debug)]
pub struct Footprints {
    footprints: Vec<FootprintRef>,
    reported: HashSet<Uuid>,
    journal: Journal,
    deep: usize,
}

impl Footprints {
    pub fn new(journal: &Journal, deep: Option<usize>) -> Self {
        Self {
            footprints: Vec::new(),
            reported: HashSet::new(),
            journal: journal.clone(),
            deep: deep.unwrap_or(TRACE_DEFAILT_DEEP),
        }
    }
    pub fn add(&mut self, token: &usize, value: Option<String>) {
        if self.deep != 0 {
            self.footprints.push((*token, value));
            if self.footprints.len() > self.deep && !self.footprints.is_empty() {
                self.footprints.remove(0);
            }
        }
    }
    pub fn report_err(&mut self, maps: &mut Maps, err: LinkedErrSerialized) -> Result<(), E> {
        if self.reported.contains(&err.uuid) {
            return Ok(());
        }
        let mut records: Vec<Footprint> = Vec::new();
        for (token, value) in self.footprints.iter() {
            records.push((
                // TODO: return here good formated fragment
                maps.get(token)?.get_fragment(token)?.content,
                Status::Output(value.clone()),
            ));
        }
        let Some(token) = err.token else {
            self.journal.report((records, None, err).into());
            return Ok(());
        };
        records.push((
            maps.get(&token)?.get_fragment(&token)?.content,
            Status::Error(err.e.clone()),
        ));
        self.journal.report(
            (
                records,
                Some(maps.get(&token)?.report_err(&token, &err.e)?),
                err,
            )
                .into(),
        );
        Ok(())
    }
}
