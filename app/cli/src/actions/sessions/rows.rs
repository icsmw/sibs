use std::{collections::HashMap, fmt};

use runtime::{
    scheme::{EventTy, RecordTy},
    JournalReader,
};
use uuid::Uuid;

const CHUNK: u8 = 100;
const TY_FILLER: u8 = 7;
const TS_FILLER: u8 = 16;
const OFFSET_FILLER: u16 = 4;
const COLOR_MIN: u8 = 31;
const COLOR_MAX: u8 = 36;

enum Marker {
    JobOpen,
    JobClose,
    Child,
}

impl fmt::Display for Marker {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Marker::JobOpen => "•",
                Marker::JobClose => "×",
                Marker::Child => "·",
            }
        )
    }
}

impl From<&EventTy> for Marker {
    fn from(event: &EventTy) -> Self {
        match event {
            EventTy::JobOpened => Marker::JobOpen,
            EventTy::JobClosed => Marker::JobClose,
            _ => Marker::Child,
        }
    }
}

struct MarkerInfo {
    inner: Marker,
    color: u8,
}

impl MarkerInfo {
    fn new(event: &EventTy, color: u8) -> Self {
        Self {
            inner: event.into(),
            color,
        }
    }
}

impl fmt::Display for MarkerInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\x1b[{}m{}\x1b[0m", self.color, self.inner)
    }
}

struct Colors {
    color: u8,
}

impl Default for Colors {
    fn default() -> Self {
        Self { color: COLOR_MIN }
    }
}

impl Colors {
    fn next(&mut self) -> u8 {
        let color = self.color;
        self.color += 1;
        if self.color > COLOR_MAX {
            self.color = COLOR_MIN;
        }
        color
    }
}

pub fn render(reader: &mut JournalReader, session: &Uuid) {
    let mut from = 0;
    let mut prev_ty: Option<RecordTy> = None;
    let mut prev_ts: Option<u64> = None;
    let mut relations: HashMap<Uuid, (u16, u8)> = HashMap::new();
    let mut colors = Colors::default();
    loop {
        let rows = reader
            .read(session, from, CHUNK as usize)
            .unwrap_or_default();
        if rows.is_empty() {
            break;
        }
        from += rows.len();
        rows.into_iter().for_each(|record| {
            if let Some(parent) = record.parent.as_ref() {
                let (offset, _) = *relations.entry(*parent).or_insert((1, colors.next()));
                relations
                    .entry(record.owner)
                    .or_insert((offset + OFFSET_FILLER, colors.next()));
            }
            let (offset, color) = *relations.entry(record.owner).or_insert((1, colors.next()));
            let marker = MarkerInfo::new(&record.event, color);
            println!(
                "{}",
                record.to_string(
                    prev_ty
                        .as_ref()
                        .map(|ty| (!(ty == &record.ty), TY_FILLER))
                        .unwrap_or((true, TY_FILLER)),
                    prev_ts
                        .as_ref()
                        .map(|ts| (ts != &record.ts, TS_FILLER))
                        .unwrap_or((true, TS_FILLER)),
                    offset,
                    marker
                )
            );
            prev_ty = Some(record.ty);
            prev_ts = Some(record.ts);
        });
    }
}
