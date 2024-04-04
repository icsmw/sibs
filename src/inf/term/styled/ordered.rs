use crate::inf::term::styled::Styled;
use console::Style;
use regex::{Captures, Regex};
use std::collections::HashMap;
use uuid::Uuid;

const SPLITTER: &str = "[>>]";

pub struct Ordered {
    lens: HashMap<Uuid, usize>,
    current: Option<Uuid>,
    width: usize,
}

impl Ordered {
    pub fn new(width: usize) -> Self {
        Self {
            lens: HashMap::new(),
            current: None,
            width,
        }
    }
}

impl Styled for Ordered {
    fn apply(&mut self, str: &str) -> String {
        let uuid = self.current.unwrap_or(Uuid::new_v4());
        let parts = str.split(SPLITTER).collect::<Vec<&str>>();
        if let (Some(before), true) = (parts.first(), parts.len() > 1) {
            self.lens
                .entry(uuid)
                .and_modify(|len| {
                    *len = *len.max(&mut before.len());
                })
                .or_insert(before.len());
            self.current = Some(uuid);
            str.replace(SPLITTER, &format!("[{uuid}]"))
        } else {
            self.current = None;
            str.to_owned()
        }
    }
    fn finalize(&mut self, str: &str) -> String {
        for (uuid, len) in self.lens.iter() {
            let mut parts = str.split(&format!("[{uuid}]")).collect::<Vec<&str>>();
            if parts.len() > 1 {
                let before = parts.remove(0);
                let mut output = format!(
                    "{before}{}",
                    " ".repeat(if *len > before.len() {
                        len - before.len()
                    } else {
                        0
                    }),
                );
                let offset = output.len();
                let right = parts.join("").to_string();
                let mut cursor = offset;
                right.split_ascii_whitespace().for_each(|w| {
                    if cursor + w.len() > self.width {
                        output = format!("{output}\n{}", " ".repeat(offset));
                        cursor = offset;
                    } else {
                        output = format!("{output}{w} ");
                        cursor += w.len() + 1;
                    }
                });
                return output;
            }
        }
        str.to_owned()
    }
}

#[test]
fn test() {
    let mut orderer = Ordered::new(70);
    let example = r#"_ [>>]_
__ [>>]_
___ [>>]_
____ [>>]_
___ [>>]_
__ [>>]_
_ [>>]_
no match
===== [>>]_
========== [>>]_
=============== [>>]_
==================== [>>]_
=============== [>>]==== ===== ====== ===== ==== ===== ====== ===== ==== ===== ====== ===== ==== ===== ====== ===== ==== ===== ====== ===== ===== ==== ===== ===== ==== ===== 
========== [>>]_
===== [>>]_"#;
    let lines = example
        .split('\n')
        .map(|s| orderer.apply(s))
        .collect::<Vec<String>>();
    let output = lines
        .iter()
        .map(|l| orderer.finalize(l))
        .collect::<Vec<String>>()
        .join("\n");
    assert_eq!(
        output,
        r#"_    _ 
__   _ 
___  _ 
____ _ 
___  _ 
__   _ 
_    _ 
no match
=====                _ 
==========           _ 
===============      _ 
==================== _ 
===============      ==== ===== ====== ===== ==== ===== ====== ===== 
                     ===== ====== ===== ==== ===== ====== ===== ==== 
                     ====== ===== ===== ==== ===== ===== ==== ===== 
==========           _ 
=====                _ "#
    );
}
