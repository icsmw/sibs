use chrono::{Local, TimeZone};
use runtime::scheme::SessionInfo;

pub enum Header {
    Uuid,
    Started,
    DoneInMs,
    Errors,
    Warnings,
    Debugs,
    Infos,
    Stdouts,
    Stderrs,
    Cwd,
}

impl Header {
    pub fn as_str(&self, opts: &TableOptions) -> &str {
        match self {
            Header::Uuid => "Uuid",
            Header::Started => "Started",
            Header::DoneInMs => {
                if opts.short_headers {
                    "ms"
                } else {
                    "Done in, ms"
                }
            }
            Header::Errors => {
                if opts.short_headers {
                    "E"
                } else {
                    "Errors"
                }
            }
            Header::Warnings => {
                if opts.short_headers {
                    "W"
                } else {
                    "Warnings"
                }
            }
            Header::Debugs => {
                if opts.short_headers {
                    "D"
                } else {
                    "Debugs"
                }
            }
            Header::Infos => {
                if opts.short_headers {
                    "I"
                } else {
                    "Infos"
                }
            }
            Header::Stdouts => {
                if opts.short_headers {
                    "SO"
                } else {
                    "Stdouts"
                }
            }
            Header::Stderrs => {
                if opts.short_headers {
                    "SE"
                } else {
                    "Stderrs"
                }
            }
            Header::Cwd => {
                if opts.short_headers {
                    "cwd"
                } else {
                    "Folder"
                }
            }
        }
    }

    pub fn row(&self, info: &SessionInfo, opts: &TableOptions) -> String {
        match self {
            Header::Uuid => {
                if opts.cut_uuid {
                    info.uuid.to_string()[..8].to_string()
                } else {
                    info.uuid.to_string()
                }
            }
            Header::Started => Local
                .timestamp_opt(info.open as i64, 0)
                .single()
                .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
                .unwrap_or_else(|| info.open.to_string()),
            Header::DoneInMs => info.close.saturating_sub(info.open).to_string(),
            Header::Errors => {
                if let Some(stat) = info.md.stat {
                    stat.errs.to_string()
                } else {
                    "-".to_string()
                }
            }
            Header::Warnings => {
                if let Some(stat) = info.md.stat {
                    stat.warns.to_string()
                } else {
                    "-".to_string()
                }
            }
            Header::Debugs => {
                if let Some(stat) = info.md.stat {
                    stat.debugs.to_string()
                } else {
                    "-".to_string()
                }
            }
            Header::Infos => {
                if let Some(stat) = info.md.stat {
                    stat.infos.to_string()
                } else {
                    "-".to_string()
                }
            }
            Header::Stdouts => {
                if let Some(stat) = info.md.stat {
                    stat.stdouts.to_string()
                } else {
                    "-".to_string()
                }
            }
            Header::Stderrs => {
                if let Some(stat) = info.md.stat {
                    stat.stderrs.to_string()
                } else {
                    "-".to_string()
                }
            }
            Header::Cwd => {
                let cwd = info.md.cwd.to_string_lossy().to_string();
                if opts.cut_cwd {
                    cwd.split('/').next_back().unwrap_or(&cwd).to_string()
                } else {
                    cwd
                }
            }
        }
    }
}

#[derive(Default)]
pub struct TableOptions {
    pub short_headers: bool,
    pub cut_cwd: bool,
    pub cut_uuid: bool,
}

impl TableOptions {
    pub fn analize<'a, I: Iterator<Item = &'a SessionInfo>>(mut self, data: I) -> Self {
        let mut cut_uuid = true;
        let mut cut_cwd = true;
        let mut prev_uuid = None;
        let mut prev_cwd = None;
        for info in data {
            if !cut_uuid {
                if let Some(prev) = prev_uuid {
                    cut_uuid = prev == &(info.uuid.as_bytes()[..8]);
                } else {
                    prev_uuid = Some(&info.uuid.as_bytes()[..8]);
                }
            }
            if !cut_cwd {
                if let Some(prev) = prev_cwd {
                    cut_cwd = prev == &info.md.cwd;
                } else {
                    prev_cwd = Some(&info.md.cwd);
                }
            }
            if !cut_uuid && !cut_cwd {
                break;
            }
        }
        self.cut_cwd = cut_cwd;
        self.cut_uuid = cut_uuid;
        self
    }
}

pub const HEADERS: &[Header] = &[
    Header::Uuid,
    Header::Started,
    Header::DoneInMs,
    Header::Errors,
    Header::Warnings,
    Header::Debugs,
    Header::Infos,
    Header::Stdouts,
    Header::Stderrs,
    Header::Cwd,
];
