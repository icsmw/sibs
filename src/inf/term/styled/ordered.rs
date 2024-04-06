use crate::inf::term::styled::{striped_len, Styled};
use console::{strip_ansi_codes, Style};
use regex::{Captures, Regex};
use std::collections::HashMap;
use uuid::Uuid;

const SPLITTER: &str = "[>>]";

pub struct Ordered {
    lens: HashMap<Uuid, HashMap<usize, usize>>,
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
        if parts.len() > 1 {
            let mut lens = self.lens.remove(&uuid).unwrap_or_default();
            parts.iter().enumerate().for_each(|(i, part)| {
                if i == parts.len() - 1 {
                    return;
                }
                lens.entry(i)
                    .and_modify(|len| {
                        *len = *len.max(&mut striped_len(part));
                    })
                    .or_insert(striped_len(part));
            });
            self.lens.insert(uuid, lens);
            self.current = Some(uuid);
            str.replace(SPLITTER, &format!("[{uuid}]"))
        } else if !str.is_empty() {
            self.current = None;
            str.to_owned()
        } else {
            str.to_owned()
        }
    }
    fn finalize(&mut self, str: &str) -> String {
        for (uuid, lens) in self.lens.iter() {
            let parts = str.split(&format!("[{uuid}]")).collect::<Vec<&str>>();
            if parts.len() > 1 {
                let mut output = String::new();
                parts.iter().enumerate().for_each(|(i, part)| {
                    if i < parts.len() - 1 {
                        let len = lens.get(&i).unwrap_or(&0);
                        output.push_str(&format!(
                            "{part}{}",
                            " ".repeat(if *len > striped_len(part) {
                                len - striped_len(part)
                            } else {
                                0
                            })
                        ));
                    } else {
                        let offset = striped_len(&output);
                        let mut cursor = offset;
                        part.split_ascii_whitespace().for_each(|w| {
                            if cursor + striped_len(w) + 1 >= self.width {
                                output.push_str(&format!("\n{}{w} ", " ".repeat(offset)));
                                cursor = offset + striped_len(w) + 1;
                            } else {
                                output.push_str(&format!("{w} "));
                                cursor += striped_len(w) + 1;
                            }
                        });
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
