#[cfg(test)]
mod walker {
    use crate::reader::Reader;

    #[test]
    fn until_char() {
        let words = ["one", "two", r"\$%^\_0", r"a\@b"];
        let targets = ['@', '$', '_'];
        let mut count = 0;
        targets.iter().for_each(|target| {
            let mut bound = Reader::unbound(words.join(&target.to_string()));
            let mut cursor: usize = 0;
            words.iter().for_each(|word| {
                let (read, char) = if let Some((read, char)) = bound.until().char(&[target]) {
                    assert!(bound.move_to().next());
                    (read, char)
                } else {
                    (bound.move_to().end(), *target)
                };
                let token = bound.token().unwrap();
                assert_eq!(read, *word);
                assert_eq!(char, *target);
                assert_eq!(token.content, *word);
                assert_eq!(token.from, cursor);
                assert_eq!(token.len, word.len());
                cursor += word.len() + 1;
                count += 1;
            });
        });
        assert_eq!(count, words.len() * targets.len());
    }
    #[test]
    fn move_to_none_numeric() {
        let words = ["one", "two", "-one", "\\-two"];
        let targets = [
            ("111", "111", 111),
            ("-222", "-222", -222),
            ("1", "1", 1),
            ("-1", "-1", -1),
            (" - 111", "-111", -111),
            ("-  222", "-222", -222),
            (" - 1", "-1", -1),
            ("       -   1", "-1", -1),
        ];
        targets.iter().for_each(|target| {
            words.iter().for_each(|word| {
                let mut reader = Reader::unbound(format!("{}{word}", target.0));
                if let Some(value) = reader.move_to().none_numeric() {
                    let token = reader.token().unwrap();
                    assert_eq!(target.0, token.content);
                    assert_eq!(target.1, value);
                    assert_eq!(target.2, value.trim().parse::<isize>().unwrap());
                    assert_eq!(
                        reader.next().char().unwrap().to_string(),
                        word.get(0..1).unwrap()
                    );
                } else {
                    panic!("Fail to read numeric value");
                }
                let mut reader = Reader::unbound(target.0.to_string());
                if let Some(value) = reader.move_to().none_numeric() {
                    let token = reader.token().unwrap();
                    assert_eq!(target.0, token.content);
                    assert_eq!(target.1, value);
                    assert_eq!(target.2, value.trim().parse::<isize>().unwrap());
                } else {
                    panic!("Fail to read numeric value");
                }
            });
        });
    }
    #[test]
    fn until_word() {
        let words = ["one", "two", r"\$\>\!%^\=_0", r"a\>b"];
        let targets = [">", "==", "!=", "=>"];
        let mut count = 0;
        targets.iter().for_each(|target| {
            let mut bound = Reader::unbound(words.join(target.as_ref()));
            let mut cursor: usize = 0;
            words.iter().for_each(|word| {
                let (read, stopped) = if let Some((read, stopped)) = bound.until().word(&[*target])
                {
                    assert!(bound.move_to().if_next(&stopped));
                    (read, stopped)
                } else {
                    (bound.move_to().end(), target.to_string())
                };
                let token = bound.token().unwrap();
                assert_eq!(read, *word);
                assert_eq!(stopped, *target);
                assert_eq!(token.content, *word);
                assert_eq!(token.from, cursor);
                assert_eq!(token.len, word.len());
                cursor += word.len() + target.len();
                count += 1;
            });
        });
        assert_eq!(count, words.len() * targets.len());
    }
    #[test]
    fn move_to_char() {
        let words = ["    ", "\t\t\t\n\n\n", "\t \n \t \n"];
        let targets = ['@', '$', '_'];
        let mut count = 0;
        let times = 4;
        words.iter().for_each(|word| {
            targets.iter().for_each(|target| {
                let mut content = String::new();
                for _ in 0..times {
                    content = format!("{content}{word}{target}");
                }
                let mut bound = Reader::unbound(content);
                for n in 0..times {
                    let stopped = bound.move_to().char(&[target]).unwrap();
                    let token = bound.token().unwrap();
                    assert_eq!(stopped, *target);
                    assert_eq!(token.content, *word);
                    let from = n * (word.len() + 1);
                    assert_eq!(token.from, from);
                    assert_eq!(token.len, word.len());
                    count += 1;
                }
            });
        });
        assert_eq!(count, words.len() * targets.len() * times);
    }
    #[test]
    fn move_to_expression() {
        let words = ["    ", "\t\t\t\n\n\n", "\t \n \t \n"];
        let targets = [">", "==", "!=", "=>"];
        let mut count = 0;
        let times = 4;
        words.iter().for_each(|word| {
            targets.iter().for_each(|target| {
                let mut content = String::new();
                for _ in 0..times {
                    content = format!("{content}{word}{target}");
                }
                let mut bound = Reader::unbound(content);
                for n in 0..times {
                    let stopped = bound.move_to().expression(&[target]).unwrap();
                    let token = bound.token().unwrap();
                    assert_eq!(stopped, *target);
                    assert_eq!(token.content.trim(), *target);
                    let from = n * (word.len() + target.len());
                    assert_eq!(token.from, from);
                    assert_eq!(token.len, word.len() + target.len());
                    count += 1;
                }
            });
        });
        assert_eq!(count, words.len() * targets.len() * times);
    }
    #[test]
    fn move_to_word() {
        let words = ["    ", "\t\t\t\n\n\n", "\t \n \t \n"];
        let targets = ["true", "false", "with space"];
        let mut count = 0;
        let times = 4;
        words.iter().for_each(|word| {
            targets.iter().for_each(|target| {
                let mut content = String::new();
                for _ in 0..times {
                    content = format!("{content}{word}{target}");
                }
                let mut bound = Reader::unbound(content);
                for n in 0..times {
                    let stopped = bound.move_to().word(&[target]).unwrap();
                    let token = bound.token().unwrap();
                    assert_eq!(stopped, *target);
                    assert_eq!(token.content.trim(), *target);
                    let from = n * (word.len() + target.len());
                    assert_eq!(token.from, from);
                    assert_eq!(token.len, word.len() + target.len());
                    count += 1;
                }
            });
        });
        assert_eq!(count, words.len() * targets.len() * times);
        count = 0;
        targets.iter().for_each(|target| {
            let content = format!("{target}{target}");
            let mut bound = Reader::unbound(content);
            assert!(bound.move_to().word(&[target]).is_none());
            count += 1;
        });
        assert_eq!(count, targets.len());
    }

    #[test]
    fn group_between() {
        let noise = "abcdefg123456";
        let borders = [('{', '}'), ('<', '>'), ('[', ']'), ('>', '<')];
        let mut count = 0;
        borders.iter().for_each(|(left, right)| {
            {
                // Nested groups
                let content = format!("{left}{noise}{right}{noise}\\{left}{noise}\\{right}{noise}");
                let mut bound = Reader::unbound(format!(" \t\n {left}{content}{right}{noise}"));
                let between = bound.group().between(left, right).unwrap();
                assert_eq!(between, content);
                let token = bound.token().unwrap();
                assert_eq!(token.content, between);
                let mut bound = Reader::unbound(between);
                let between = bound.group().between(left, right).unwrap();
                let token = bound.token().unwrap();
                assert_eq!(token.content, between);
                assert_eq!(between, noise);
            }
            {
                // Nested shifted groups
                let content = format!("{noise}\\{left}{left}{noise}{right}\\{right}{noise}");
                let mut bound = Reader::unbound(format!("{left}{content}{right}{noise}"));
                let between = bound.group().between(left, right).unwrap();
                assert_eq!(between, content);
                let token = bound.token().unwrap();
                assert_eq!(token.content, between);
                let mut bound = Reader::unbound(between);
                bound.until().char(&[left]);
                let between = bound.group().between(left, right).unwrap();
                let token = bound.token().unwrap();
                assert_eq!(token.content, between);
                assert_eq!(between, noise);
            }
            {
                // Following groups with spaces between
                let content = format!("{noise}\\{left}{noise}\\{right}{noise}");
                let mut bound = Reader::unbound(format!(
                    "{left}{content}{right} \t \n{left}{content}{right}"
                ));
                let between = bound.group().between(left, right).unwrap();
                assert_eq!(between, content);
                let token = bound.token().unwrap();
                assert_eq!(token.content, between);
                let between = bound.group().between(left, right).unwrap();
                let token = bound.token().unwrap();
                assert_eq!(token.content, between);
                assert_eq!(between, content);
            }
            {
                // Following groups without spaces
                let content = format!("{noise}\\{left}{noise}\\{right}{noise}");
                let mut bound =
                    Reader::unbound(format!("{left}{content}{right}{left}{content}{right}"));
                let between = bound.group().between(left, right).unwrap();
                assert_eq!(between, content);
                let token = bound.token().unwrap();
                assert_eq!(token.content, between);
                let between = bound.group().between(left, right).unwrap();
                let token = bound.token().unwrap();
                assert_eq!(token.content, between);
                assert_eq!(between, content);
            }
            {
                // Empty groups
                let mut bound = Reader::unbound(format!("{left}{right}"));
                let between: String = bound.group().between(left, right).unwrap();
                assert_eq!(between, "");
                let token = bound.token().unwrap();
                assert_eq!(token.content, between);
            }
            {
                // Group without ending
                let mut bound = Reader::unbound(format!("{left}----------"));
                assert!(bound.group().between(left, right).is_none());
                let mut bound = Reader::unbound(format!("{left}----------\\{left}"));
                assert!(bound.group().between(left, right).is_none());
            }
            count += 1;
        });
        assert_eq!(count, borders.len());
    }
    #[test]
    fn mapping() {
        let noise = "=================";
        let inner = format!("<{noise}>{noise}");
        let mut bound = Reader::unbound(format!("[{inner}]"));
        let between = bound.group().between(&'[', &']').unwrap();
        assert_eq!(between, inner);
        let mut token = bound.token().unwrap();
        assert_eq!(token.content, inner);
        assert_eq!(token.from, 1);
        assert_eq!(token.len, inner.len());
        let between = token.bound.group().between(&'<', &'>').unwrap();
        assert_eq!(between, noise);
        let nested_token = token.bound.token().unwrap();
        assert_eq!(nested_token.content, noise);
        assert_eq!(nested_token.from, 2);
        assert_eq!(nested_token.len, noise.len());
    }
    #[test]
    fn to_end() {
        let noise = "=================";
        let mut bound = Reader::unbound(noise.to_string());
        let full = bound.move_to().end();
        assert_eq!(full, noise);
        let token = bound.token().unwrap();
        assert_eq!(token.content, noise);
        assert_eq!(token.from, 0);
        assert_eq!(token.len, noise.len());
        let mut bound = Reader::unbound(format!("{noise}@{noise}"));
        bound.until().char(&[&'@']).unwrap();
        bound.move_to().next();
        let rest = bound.move_to().end();
        assert_eq!(rest, noise);
        let token = bound.token().unwrap();
        assert_eq!(token.content, noise);
        assert_eq!(token.from, noise.len() + 1);
        assert_eq!(token.len, noise.len());
    }
}

#[cfg(test)]
mod reading {
    use crate::{
        error::LinkedErr,
        inf::context::Context,
        reader::{error::E, read_file},
    };

    #[tokio::test]
    async fn reading() -> Result<(), LinkedErr<E>> {
        let target = std::env::current_dir()
            .unwrap()
            .join("./src/tests/reading/full/build.sibs");
        let mut cx = Context::from_filename(&target)?;
        match read_file(&mut cx).await {
            Ok(components) => {
                assert_eq!(components.len(), 11);
            }
            Err(err) => {
                cx.gen_report_from_err(&err)?;
                cx.post_reports();
                let _ = cx.tracker.shutdown().await;
                return Err(err);
            }
        }
        assert_eq!(read_file(&mut cx).await?.len(), 11);
        Ok(())
    }
}
