use runtime::{scheme::RecordTy, JournalReader};
use uuid::Uuid;

const CHUNK: u8 = 100;
const TY_FILLER: u8 = 7;
const TS_FILLER: u8 = 16;

pub fn render(reader: &mut JournalReader, session: &Uuid) {
    let mut from = 0;
    let mut prev_ty: Option<RecordTy> = None;
    let mut prev_ts: Option<u64> = None;
    loop {
        let rows = reader
            .read(session, from, CHUNK as usize)
            .unwrap_or_default();
        if rows.is_empty() {
            break;
        }
        from += rows.len();

        rows.into_iter().for_each(|record| {
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
                        .unwrap_or((true, TS_FILLER))
                )
            );
            prev_ty = Some(record.ty);
            prev_ts = Some(record.ts);
        });
    }
}
