use crate::inf::tracker::{OperationResult, Storage, E};
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use std::{collections::HashMap, time::Instant};

pub struct Progress {
    mp: MultiProgress,
    bars: HashMap<usize, (ProgressBar, Instant, String, Option<OperationResult>)>,
    sequence: usize,
}

impl Progress {
    fn offset(num: usize, total: usize) -> String {
        " ".repeat(format!("[{total}/{total}]").len() - format!("[{num}/{total}]").len())
            .to_string()
    }
    pub fn new() -> Self {
        Progress {
            sequence: 0,
            mp: MultiProgress::new(),
            bars: HashMap::new(),
        }
    }
    pub fn create<'a, T>(&mut self, alias: T, len: Option<u64>) -> Result<usize, E>
    where
        T: 'a + ToOwned + ToString,
    {
        let spinner_style = ProgressStyle::with_template("{spinner} {prefix:.bold.dim} {wide_msg}")
            .map_err(|e| E::ProgressBarError(e.to_string()))?
            .tick_chars("▂▃▅▆▇▆▅▃▂ ");
        self.sequence += 1;
        let bar = self.mp.add(ProgressBar::new(len.unwrap_or(u64::MAX)));
        bar.set_style(spinner_style.clone());
        self.bars.insert(
            self.sequence,
            (bar, Instant::now(), alias.to_string(), None),
        );
        self.reprint();
        Ok(self.sequence)
    }
    pub fn set_message<'a, T>(&mut self, sequence: usize, msg: T)
    where
        T: 'a + ToOwned + ToString,
    {
        if let Some((bar, _, alias, _)) = self.bars.get(&sequence) {
            bar.set_message(msg.to_string());
            self.reprint();
        }
    }
    pub fn inc(&mut self, sequence: usize, position: Option<u64>) {
        if let Some((bar, _, _, _)) = self.bars.get(&sequence) {
            if let Some(pos) = position {
                bar.set_position(pos);
            } else {
                bar.inc(1);
            }
            self.reprint();
        }
    }
    pub fn finish(&mut self, sequence: usize, result: OperationResult) {
        if let Some((bar, instant, alias, res)) = self.bars.get_mut(&sequence) {
            bar.set_prefix(format!(
                "[{sequence}/{sequence}]{}[{result}][{alias}]",
                Progress::offset(sequence, sequence)
            ));
            bar.finish_with_message(format!("{alias} done in {}s.", instant.elapsed().as_secs()));
            res.replace(result);
            self.reprint();
        }
    }
    pub fn shutdown(&mut self) {
        self.bars
            .iter_mut()
            .for_each(|(_, (bar, instant, alias, _))| {
                if !bar.is_finished() {
                    bar.finish_with_message(format!(
                        "{alias} shutdown in {}s.",
                        instant.elapsed().as_secs()
                    ));
                }
            });
        self.reprint();
        self.bars.clear();
    }

    fn reprint(&mut self) {
        let sequence = self.sequence;
        self.bars
            .iter_mut()
            .for_each(|(k, (bar, _, alias, result))| {
                let msg = if let Some(result) = result.as_ref() {
                    format!(
                        "[{k}/{sequence}]{}[{result}][{alias}]",
                        Progress::offset(*k, sequence)
                    )
                } else {
                    format!(
                        "[{k}/{sequence}]{}[....][{alias}]",
                        Progress::offset(*k, sequence)
                    )
                };
                bar.set_prefix(msg);
            });
    }
}
