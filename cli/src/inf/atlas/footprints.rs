use std::collections::HashMap;
use uuid::Uuid;

use crate::{
    error::LinkedErrSerialized,
    inf::{
        atlas::{Maps, E},
        map::{Fragment, Map},
    },
};

pub const TRACE_DEFAILT_DEEP: usize = 5;

#[derive(Debug)]
enum Status {
    Output(Option<String>),
    Error(String),
}
type Footprint = (usize, Option<String>);
type Record = (usize, Fragment, Status);

#[derive(Debug)]
pub struct Footprints {
    footprints: Vec<Footprint>,
    reports: HashMap<Uuid, (Vec<Record>, String)>,
    deep: usize,
}

impl Footprints {
    pub fn new(deep: Option<usize>) -> Self {
        Self {
            footprints: Vec::new(),
            reports: HashMap::new(),
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
    pub fn report_err(
        &mut self,
        maps: &mut Maps,
        token: &usize,
        err: LinkedErrSerialized,
    ) -> Result<(), E> {
        if self.reports.contains_key(&err.uuid) {
            return Ok(());
        }
        let mut records: Vec<Record> = Vec::new();
        for (token, value) in self.footprints.iter() {
            records.push((
                *token,
                maps.get(token)?.get_fragment(token)?,
                Status::Output(value.clone()),
            ));
        }
        records.push((
            *token,
            maps.get(token)?.get_fragment(token)?,
            Status::Error(err.e.clone()),
        ));
        self.reports.insert(
            err.uuid,
            (records, maps.get(token)?.report_err(token, err.e)?),
        );
        Ok(())
    }
}
