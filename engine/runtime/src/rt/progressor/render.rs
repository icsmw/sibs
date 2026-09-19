use crate::*;
use indexmap::IndexMap;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};

struct ProgressRef {
    pub element: JobElement,
    pub childs: IndexMap<Uuid, ProgressRef>,
    pub bar: ProgressBar,
    pub state: Option<ProgressState>,
}

impl ProgressRef {
    pub fn set_state(&mut self, state: ProgressState) {
        self.state = Some(state);
    }
    pub fn find(&mut self, uuid: &Uuid) -> Option<&mut ProgressRef> {
        if &self.element.uuid == uuid {
            Some(self)
        } else if self.childs.contains_key(uuid) {
            self.childs.get_mut(uuid)
        } else {
            self.childs.values_mut().find_map(|pref| pref.find(uuid))
        }
    }
    pub fn mount(&mut self, mp: &mut MultiProgress, st: &Styles) {
        // Reorder existing bars instead of leaving old rows in MultiProgress.
        mp.remove(&self.bar);
        self.bar.set_style(st.style(self.state.as_ref()));
        mp.add(self.bar.clone());
        self.childs.values_mut().for_each(|chld| chld.mount(mp, st));
    }
    pub fn print(&self, prefix: &str, children_prefix: &str) {
        self.bar
            .set_prefix(format!("{prefix}{}", self.element.alias));
        let message = match &self.state {
            Some(state) => {
                let detail = state.get_msg().unwrap_or_default();
                if detail.is_empty() {
                    format!("[{state}]")
                } else {
                    format!("[{state}] {detail}")
                }
            }
            None => String::new(),
        };
        self.bar.set_message(message);
        if matches!(
            self.state,
            Some(ProgressState::Working(..) | ProgressState::Pending(..))
        ) {
            self.bar.inc(1);
        }
        for (index, child) in self.childs.values().enumerate() {
            let last = index + 1 == self.childs.len();
            let branch = if last { "└── " } else { "├── " };
            let continuation = if last { "    " } else { "│   " };
            child.print(
                &format!("{children_prefix}{branch}"),
                &format!("{children_prefix}{continuation}"),
            );
        }
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
            progress: ProgressStyle::with_template("{prefix:.bold} [{spinner}] {wide_msg}")?
                .tick_chars("▁▂▃▅▆▇▆▅▃▂▁"),
            pending: ProgressStyle::with_template("{prefix:.bold} [{spinner}] {wide_msg}")?
                .tick_chars("←↖↑↗→↘↓↙"),
            none: ProgressStyle::with_template("{prefix:.bold} {wide_msg}")?,
        })
    }
    fn get(&self, state: Option<&ProgressState>) -> ProgressBar {
        let bar = ProgressBar::no_length();
        bar.set_style(self.style(state));
        bar
    }
    fn style(&self, state: Option<&ProgressState>) -> ProgressStyle {
        match state {
            Some(ProgressState::Working(..) | ProgressState::Progress(..)) => self.progress.clone(),
            Some(ProgressState::Pending(..)) => self.pending.clone(),
            Some(
                ProgressState::Success(..)
                | ProgressState::Failed(..)
                | ProgressState::Cancelled(..),
            )
            | None => self.none.clone(),
        }
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
    pub fn add(&mut self, path: Vec<JobElement>) -> Result<(), E> {
        if path.is_empty() {
            return Err(E::Other(
                "Cannot register progress with an empty path".into(),
            ));
        }
        let last = path.len() - 1;
        let mut branch = &mut self.tree;
        for (index, element) in path.into_iter().enumerate() {
            let pref = branch.entry(element.uuid).or_insert_with(|| ProgressRef {
                element,
                childs: IndexMap::new(),
                bar: self.styles.get(None),
                state: None,
            });
            // Ancestors are labels until their own progress is requested.
            // Repeated requests retain the current state and existing children.
            if index == last && pref.state.is_none() {
                pref.state = Some(ProgressState::default());
            }
            branch = &mut pref.childs;
        }
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
        if let Some(state) = pref.state.as_mut() {
            state.set_msg(msg);
        }
        self.print();
    }

    pub fn print(&self) {
        self.tree.values().for_each(|pref| pref.print("", ""));
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

#[cfg(test)]
mod tests {
    use super::*;

    fn element(alias: &str) -> JobElement {
        JobElement {
            uuid: Uuid::new_v4(),
            alias: alias.into(),
        }
    }

    #[test]
    fn tree_keeps_ancestor_lines_and_alignment_when_state_changes() {
        let mut render = ProgressRender::new().unwrap();
        let root = element("root");
        let first = element("task A");
        let second = element("task B");
        let command = element("command");
        let spawn = element("spawn");
        let other = element("other root");
        render
            .add(vec![
                root.clone(),
                first.clone(),
                command.clone(),
                spawn.clone(),
            ])
            .unwrap();
        render.add(vec![root.clone(), second.clone()]).unwrap();
        render.add(vec![other.clone()]).unwrap();
        let lines = |render: &ProgressRender| {
            fn collect(node: &ProgressRef, lines: &mut Vec<String>) {
                lines.push(node.bar.prefix());
                for child in node.childs.values() {
                    collect(child, lines);
                }
            }
            let mut lines = Vec::new();
            for node in render.tree.values() {
                collect(node, &mut lines);
            }
            lines
        };
        let expected = vec![
            "root",
            "├── task A",
            "│   └── command",
            "│       └── spawn",
            "└── task B",
            "other root",
        ];
        assert_eq!(lines(&render), expected);
        render.set_state(spawn.uuid, ProgressState::Success(Some("finished".into())));
        render.add(vec![root.clone(), first.clone()]).unwrap();
        assert_eq!(lines(&render), expected);
        let node = render
            .tree
            .get_mut(&root.uuid)
            .unwrap()
            .find(&spawn.uuid)
            .unwrap();
        assert!(node.bar.message().ends_with("finished"));
    }

    #[test]
    fn child_request_creates_ancestor_labels_and_only_one_active_bar() {
        let mut render = ProgressRender::new().unwrap();
        let root = element("root");
        let task = element("task");
        let child = element("command");
        render
            .add(vec![root.clone(), task.clone(), child.clone()])
            .unwrap();
        let root_ref = &render.tree[&root.uuid];
        assert!(root_ref.state.is_none());
        let task_ref = &root_ref.childs[&task.uuid];
        assert!(task_ref.state.is_none());
        assert!(matches!(
            task_ref.childs[&child.uuid].state,
            Some(ProgressState::Working(_))
        ));
    }

    #[test]
    fn parent_request_and_repeat_requests_preserve_children_and_state() {
        let mut render = ProgressRender::new().unwrap();
        let parent = element("parent");
        let child = element("child");
        let sibling = element("sibling");
        let path = vec![parent.clone(), child.clone()];
        render.add(path.clone()).unwrap();
        render.set_state(
            child.uuid,
            ProgressState::Progress(Some("half".into()), 5, 10),
        );
        render.add(vec![parent.clone(), sibling.clone()]).unwrap();
        render.add(vec![parent.clone()]).unwrap();
        render.set_state(parent.uuid, ProgressState::Pending(Some("waiting".into())));
        render.add(vec![parent.clone()]).unwrap();
        render.add(path).unwrap();
        assert_eq!(render.tree.len(), 1);
        let parent_ref = &render.tree[&parent.uuid];
        assert!(matches!(parent_ref.state, Some(ProgressState::Pending(_))));
        assert_eq!(parent_ref.childs.len(), 2);
        let child_ref = &parent_ref.childs[&child.uuid];
        assert!(matches!(
            child_ref.state,
            Some(ProgressState::Progress(_, 5, 10))
        ));
        assert_eq!(
            child_ref.state.as_ref().unwrap().get_msg().as_deref(),
            Some("half")
        );
    }

    #[test]
    fn empty_path_is_rejected_without_creating_rows() {
        let mut render = ProgressRender::new().unwrap();
        assert!(render.add(vec![]).is_err());
        assert!(render.tree.is_empty());
    }
}
