use crate::*;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use std::time::Instant;

pub struct ProgressRender {
    mp: MultiProgress,
    bars: HashMap<usize, (Option<ProgressBar>, Instant, String, Option<JobResultExt>)>,
    sequence: usize,
}

impl ProgressRender {
    fn offset(num: usize, total: usize) -> String {
        " ".repeat(format!("[{total}/{total}]").len() - format!("[{num}/{total}]").len())
            .to_string()
    }
    pub fn new() -> Self {
        ProgressRender {
            sequence: 0,
            mp: MultiProgress::new(),
            bars: HashMap::new(),
        }
    }
    pub fn add_job<T: AsRef<str>>(&mut self, alias: T, len: Option<u64>) -> Result<usize, E> {
        self.sequence += 1;
        let spinner_style =
            ProgressStyle::with_template("{spinner} {prefix:.bold.dim} {wide_msg}")?
                .tick_chars("▂▃▅▆▇▆▅▃▂ ");
        let bar = self.mp.add(ProgressBar::new(len.unwrap_or(u64::MAX)));
        bar.set_style(spinner_style.clone());
        self.bars.insert(
            self.sequence,
            (Some(bar), Instant::now(), alias.as_ref().to_string(), None),
        );
        self.reprint();
        Ok(self.sequence)
    }
    pub fn msg<T: AsRef<str>>(&mut self, sequence: usize, msg: T) {
        if let Some((Some(bar), _, _alias, _)) = self.bars.get(&sequence) {
            bar.set_message(msg.as_ref().to_string());
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
    pub fn len(&mut self, sequence: usize, len: u64) {
        if let Some((Some(bar), _, _, _)) = self.bars.get(&sequence) {
            bar.set_length(len);
            self.reprint();
        }
    }
    pub fn finish(&mut self, sequence: usize, result: JobResultExt) {
        if let Some((Some(bar), instant, alias, res)) = self.bars.get_mut(&sequence) {
            bar.set_prefix(format!(
                "[{sequence}/{sequence}]{}[{result}][{alias}]",
                ProgressRender::offset(sequence, sequence)
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
                            ProgressRender::offset(*k, sequence)
                        )
                    } else {
                        format!(
                            "[{k}/{sequence}]{}[....][{alias}]",
                            ProgressRender::offset(*k, sequence)
                        )
                    };
                    bar.set_prefix(msg);
                }
            });
    }
}
