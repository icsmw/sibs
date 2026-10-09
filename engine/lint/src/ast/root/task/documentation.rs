use std::{borrow::Cow, collections::HashSet};

use pulldown_cmark::{CowStr, Event, HeadingLevel, Parser, Tag, TagEnd};

#[derive(Default)]
struct ArgumentItem<'a> {
    name: Option<Cow<'a, str>>,
    started: bool,
    has_description: bool,
}

impl<'a> ArgumentItem<'a> {
    fn text(&mut self, text: &str) {
        if text.trim().is_empty() {
            return;
        }
        self.started = true;
        if self.name.is_some()
            && text
                .chars()
                .any(|ch| !ch.is_whitespace() && !matches!(ch, '-' | '–' | '—' | ':'))
        {
            self.has_description = true;
        }
    }

    fn code(&mut self, code: CowStr<'a>, starts_item: bool) {
        if !self.started {
            self.started = true;
            if starts_item && !code.trim().is_empty() {
                self.name = Some(match code {
                    CowStr::Borrowed(name) => Cow::Borrowed(name),
                    name => Cow::Owned(name.into_string()),
                });
            }
        } else {
            self.text(&code);
        }
    }
}

/// Reads top-level bullet items under an `Arguments` heading. Each item must
/// start with a code span containing the parameter name and include a nonempty
/// description. Markdown determines item boundaries, continuations and code
/// blocks; nested lists and examples never introduce additional parameters.
pub(super) fn documented_arguments(markdown: &str) -> HashSet<Cow<'_, str>> {
    let mut documented = HashSet::new();
    let mut parents = Vec::new();
    let mut section: Option<HeadingLevel> = None;
    let mut heading: Option<(HeadingLevel, String)> = None;
    let mut item: Option<ArgumentItem<'_>> = None;

    for event in Parser::new(markdown) {
        match event {
            Event::Start(tag) => {
                match &tag {
                    Tag::Heading { level, .. } if parents.is_empty() => {
                        heading = Some((*level, String::new()));
                    }
                    Tag::Item if section.is_some() && parents == [TagEnd::List(false)] => {
                        item = Some(ArgumentItem::default());
                    }
                    _ => {}
                }
                parents.push(tag.to_end());
            }
            Event::End(tag) => {
                if matches!(tag, TagEnd::Heading(_))
                    && parents.len() == 1
                    && let Some((level, title)) = heading.take()
                {
                    if title.trim() == "Arguments" {
                        section = Some(level);
                    } else if section.is_some_and(|parent| level <= parent) {
                        section = None;
                    }
                }
                if tag == TagEnd::Item
                    && parents.len() == 2
                    && let Some(ArgumentItem {
                        name: Some(name),
                        has_description: true,
                        ..
                    }) = item.take()
                {
                    documented.insert(name);
                }
                parents.pop();
            }
            Event::Text(text) => {
                if let Some((_, title)) = &mut heading {
                    title.push_str(&text);
                } else if let Some(item) = &mut item {
                    item.text(&text);
                }
            }
            Event::Code(code) => {
                if let Some((_, title)) = &mut heading {
                    title.push_str(&code);
                } else if let Some(item) = &mut item {
                    let starts_item = parents == [TagEnd::List(false), TagEnd::Item]
                        || parents == [TagEnd::List(false), TagEnd::Item, TagEnd::Paragraph];
                    item.code(code, starts_item);
                }
            }
            _ => {}
        }
    }
    documented
}
