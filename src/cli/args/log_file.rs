use crate::{
    cli::{
        args::{Argument, Description},
        error::E,
    },
    elements::Component,
    inf::context::Context,
};

const ARGS: [&str; 2] = ["--logs", "-l"];

#[derive(Debug, Clone)]
pub struct LogFile {
    pub file: String,
}

impl Argument<LogFile> for LogFile {
    fn read(args: &mut Vec<String>) -> Result<Option<LogFile>, E> {
        Self::find_next_to(args, &ARGS).map(|file| file.map(|file| Self { file }))
    }
    fn desc() -> Description {
        Description {
            key: ARGS.iter().map(|s| s.to_string()).collect::<Vec<String>>(),
            desc: String::from("saves logs into given file"),
            pairs: vec![],
        }
    }
    async fn action(&mut self, _components: &[Component], _context: &mut Context) -> Result<(), E> {
        Ok(())
    }
}
