use crate::inf::{
    journal::{Configuration, Output},
    tracker::{OperationResult, E},
};
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use std::{collections::HashMap, time::Instant};

pub struct Progress {
    mp: MultiProgress,
    bars: HashMap<
        usize,
        (
            Option<ProgressBar>,
            Instant,
            String,
            Option<OperationResult>,
        ),
    >,
    sequence: usize,
    cfg: Configuration,
}

impl Progress {
    fn offset(num: usize, total: usize) -> String {
        " ".repeat(format!("[{total}/{total}]").len() - format!("[{num}/{total}]").len())
            .to_string()
    }
    pub fn new(cfg: Configuration) -> Self {
        Progress {
            sequence: 0,
            mp: MultiProgress::new(),
            bars: HashMap::new(),
            cfg,
        }
    }
    pub fn create<'a, T>(&mut self, alias: T, len: Option<u64>) -> Result<usize, E>
    where
        T: 'a + ToOwned + ToString,
    {
        self.sequence += 1;
        if !matches!(self.cfg.output, Output::Progress) {
            self.bars.insert(
                self.sequence,
                (None, Instant::now(), alias.to_string(), None),
            );
            Ok(self.sequence)
        } else {
            let spinner_style =
                ProgressStyle::with_template("{spinner} {prefix:.bold.dim} {wide_msg}")?
                    .tick_chars("▂▃▅▆▇▆▅▃▂ ");
            let bar = self.mp.add(ProgressBar::new(len.unwrap_or(u64::MAX)));
            bar.set_style(spinner_style.clone());
            self.bars.insert(
                self.sequence,
                (Some(bar), Instant::now(), alias.to_string(), None),
            );
            self.reprint();
            Ok(self.sequence)
        }
    }
    pub fn set_message<'a, T>(&mut self, sequence: usize, msg: T)
    where
        T: 'a + ToOwned + ToString,
    {
        if let Some((Some(bar), _, _alias, _)) = self.bars.get(&sequence) {
            bar.set_message(msg.to_string());
            self.reprint();
        }
    }
    pub fn inc(&mut self, sequence: usize, position: Option<u64>) {
        if let Some((Some(bar), _, _, _)) = self.bars.get(&sequence) {
            if let Some(pos) = position {
                bar.set_position(pos);
            } else {
                bar.inc(1);
            }
            self.reprint();
        }
    }
    pub fn finish(&mut self, sequence: usize, result: OperationResult) {
        if let Some((Some(bar), instant, alias, res)) = self.bars.get_mut(&sequence) {
            bar.set_prefix(format!(
                "[{sequence}/{sequence}]{}[{result}][{alias}]",
                Progress::offset(sequence, sequence)
            ));
            bar.finish_with_message(format!("{alias} done in {}s.", instant.elapsed().as_secs()));
            res.replace(result);
            self.reprint();
        }
    }
    pub fn destroy(&mut self) {
        self.bars
            .iter_mut()
            .for_each(|(_, (bar, instant, alias, _))| {
                if let Some(bar) = bar {
                    if !bar.is_finished() {
                        bar.finish_with_message(format!(
                            "{alias} shutdown in {}s.",
                            instant.elapsed().as_secs()
                        ));
                    }
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
                if let Some(bar) = bar {
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
                }
            });
    }
}
