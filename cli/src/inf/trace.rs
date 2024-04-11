use std::{collections::HashMap, fmt};
use uuid::Uuid;

use crate::{
    error::LinkedErr,
    inf::{
        map::{Fragment, Map},
        AnyValue,
    },
    reader::{Maps, E},
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
pub struct Trace {
    footprints: Vec<Footprint>,
    reports: HashMap<Uuid, (Vec<Record>, String)>,
    deep: usize,
}

impl Trace {
    pub fn new(deep: Option<usize>) -> Self {
        Self {
            footprints: Vec::new(),
            reports: HashMap::new(),
            deep: deep.unwrap_or(TRACE_DEFAILT_DEEP),
        }
    }
    pub fn add(&mut self, token: &usize, value: &Option<AnyValue>) {
        if self.deep != 0 {
            self.footprints
                .push((*token, value.as_ref().map(|e| e.to_string())));
            if self.footprints.len() > self.deep && !self.footprints.is_empty() {
                self.footprints.remove(0);
            }
        }
    }
    pub fn add_report<T>(&mut self, maps: &Maps, token: &usize, err: &LinkedErr<T>) -> Result<(), E>
    where
        T: std::error::Error + fmt::Display + ToString,
    {
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
            Status::Error(err.to_string()),
        ));

        self.reports.insert(
            err.uuid,
            (
                records,
                maps.get(token)?.report_err(token, err.e.to_string())?,
            ),
        );
        Ok(())
    }
}
