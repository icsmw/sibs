use uuid::Uuid;

use crate::cli::{args::Action, error::E};

pub struct Description {
    pub key: Vec<String>,
    pub desc: String,
}

pub trait Argument {
    fn multiple_defs(args: &[String], targets: &[&str]) -> Result<(), E> {
        if args
            .iter()
            .filter(|arg| targets.contains(&arg.as_str()))
            .count()
            > 1
        {
            Err(E::DuplicateOfKey(targets.join(", ").to_owned()))
        } else {
            Ok(())
        }
    }
    fn find(args: &mut Vec<String>, targets: &[&str]) -> Result<bool, E> {
        Self::multiple_defs(args, targets)?;
        let prev = args.len();
        args.retain(|arg| !targets.contains(&arg.as_str()));
        Ok(prev != args.len())
    }
    fn with_next(args: &mut Vec<String>, targets: &[&str]) -> Result<(bool, Option<String>), E> {
        Self::multiple_defs(args, targets)?;
        if let Some(position) = args.iter().position(|arg| targets.contains(&arg.as_str())) {
            Ok(if position == args.len() - 1 {
                (true, None)
            } else {
                let _ = args.remove(position);
                let r = args.remove(position);
                (true, Some(r))
            })
        } else {
            Ok((false, None))
        }
    }
    fn with_prev(args: &mut Vec<String>, targets: &[&str]) -> Result<(bool, Option<String>), E> {
        Self::multiple_defs(args, targets)?;
        if let Some(position) = args.iter().position(|arg| targets.contains(&arg.as_str())) {
            Ok(if position == 0 {
                (true, None)
            } else {
                let _ = args.remove(position);
                (true, Some(args.remove(position - 1)))
            })
        } else {
            Ok((false, None))
        }
    }
    fn key() -> String;
    fn read(args: &mut Vec<String>) -> Result<Option<Box<dyn Action>>, E>;
    fn desc() -> Description;
}
