use crate::*;
use indexmap::IndexMap;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};

struct ProgressRef {
    pub alias: String,
    pub uuid: Uuid,
    pub childs: IndexMap<Uuid, ProgressRef>,
    pub bar: ProgressBar,
    pub state: ProgressState,
}

impl ProgressRef {
    pub fn set_state(&mut self, state: ProgressState) {
        self.state = state;
    }
    pub fn find(&mut self, uuid: &Uuid) -> Option<&mut ProgressRef> {
        if &self.uuid == uuid {
            Some(self)
        } else if self.childs.contains_key(uuid) {
            self.childs.get_mut(uuid)
        } else {
            self.childs.values_mut().find_map(|pref| pref.find(uuid))
        }
    }
    pub fn add(&mut self, child: ProgressRef) {
        self.childs.insert(child.uuid, child);
    }
    pub fn mount(&mut self, mp: &mut MultiProgress, st: &Styles) {
        self.bar = mp.add(st.get(&self.state));
        self.childs.values_mut().for_each(|chld| chld.mount(mp, st));
    }
    pub fn print(&self, index: usize, total: usize, deep: usize) {
        let filler = " "
            .repeat(format!("[{total}/{total}]").len() - format!("[{index}/{total}]").len())
            .to_string();
        let offset = if deep == 0 {
            " ".repeat(deep * 4)
        } else if index == total - 1 {
            format!("{}└", " ".repeat(deep * 4 - 1))
        } else {
            format!("{}├", " ".repeat(deep * 4 - 1))
        };
        self.bar.set_prefix(format!(
            "{offset}[{index}/{total}]{filler}[{}][{}]",
            self.state, self.alias
        ));
        if let Some(msg) = self.state.get_msg() {
            self.bar.set_message(msg);
        }
        if matches!(
            self.state,
            ProgressState::Working(..) | ProgressState::Pending(..)
        ) {
            self.bar.inc(1);
        }
        self.childs.values().enumerate().for_each(|(n, chld)| {
            chld.print(n, self.childs.len(), deep + 1);
        });
    }
}

struct Styles {
    progress: ProgressStyle,
    pending: ProgressStyle,
    none: ProgressStyle,
}

impl Styles {
    fn new() -> Result<Self, E> {
        Ok(Self {
            progress: ProgressStyle::with_template("[{spinner}]{prefix:.bold.dim} {wide_msg}")?
                .tick_chars("▁▂▃▅▆▇▆▅▃▂▁"),
            pending: ProgressStyle::with_template("[{spinner}]{prefix:.bold.dim} {wide_msg}")?
                .tick_chars("←↖↑↗→↘↓↙"),
            none: ProgressStyle::with_template("{prefix:.bold.dim} {wide_msg}")?,
        })
    }
    fn get(&self, state: &ProgressState) -> ProgressBar {
        let bar = ProgressBar::no_length();
        bar.set_style(match state {
            ProgressState::Working(..) | ProgressState::Progress(..) => self.progress.clone(),
            ProgressState::Pending(..) => self.pending.clone(),
            ProgressState::Success(..)
            | ProgressState::Failed(..)
            | ProgressState::Cancelled(..) => self.none.clone(),
        });
        bar
    }
}

pub struct ProgressRender {
    mp: MultiProgress,
    tree: IndexMap<Uuid, ProgressRef>,
    styles: Styles,
}

impl ProgressRender {
    pub fn new() -> Result<Self, E> {
        Ok(ProgressRender {
            mp: MultiProgress::new(),
            tree: IndexMap::new(),
            styles: Styles::new()?,
        })
    }
    pub fn add(&mut self, progress: &Progress) -> Result<(), E> {
        let state = ProgressState::default();
        let pref = ProgressRef {
            alias: progress.alias.clone(),
            uuid: progress.owner,
            childs: IndexMap::new(),
            bar: self.styles.get(&state),
            state,
        };
        if let Some(parent) = progress.parent.as_ref() {
            let Some(parent) = self.tree.values_mut().find_map(|pref| pref.find(parent)) else {
                return Err(E::NoProgressForTask(*parent));
            };
            parent.add(pref);
        } else {
            self.tree.insert(pref.uuid, pref);
        };
        self.mount();
        Ok(())
    }

    pub fn set_state(&mut self, uuid: Uuid, state: ProgressState) {
        let Some(pref) = self.tree.values_mut().find_map(|pref| pref.find(&uuid)) else {
            tracing::error!("Fail to find progress for job: {uuid}");
            return;
        };
        pref.set_state(state);
        self.mount();
    }

    pub fn set_msg(&mut self, uuid: Uuid, msg: String) {
        let Some(pref) = self.tree.values_mut().find_map(|pref| pref.find(&uuid)) else {
            tracing::error!("Fail to find progress for job: {uuid}");
            return;
        };
        pref.state.set_msg(msg);
        self.print();
    }

    pub fn print(&self) {
        self.tree
            .values()
            .enumerate()
            .for_each(|(n, pref)| pref.print(n, self.tree.len(), 0));
    }

    pub fn destroy(&mut self) {
        if let Err(err) = self.mp.clear() {
            tracing::error!("Fail clear progress bars: {err}");
        }
        self.tree.clear();
    }

    fn mount(&mut self) {
        if let Err(err) = self.mp.clear() {
            tracing::error!("Fail drop current progress bars: {err}. Will recreate it.");
            self.mp = MultiProgress::new();
        }
        self.tree
            .values_mut()
            .for_each(|bar| bar.mount(&mut self.mp, &self.styles));
        self.print();
    }
}
