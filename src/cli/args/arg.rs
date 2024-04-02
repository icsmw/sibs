use uuid::Uuid;

use crate::cli::{args::Action, error::E};

pub struct Description {
    pub key: Vec<String>,
    pub desc: String,
    pub pairs: Vec<(String, String)>,
}

pub trait Argument {
    fn find_next_to(args: &mut Vec<String>, targets: &[&str]) -> Result<Option<String>, E> {
        if let Some(position) = args.iter().position(|arg| targets.contains(&arg.as_str())) {
            if position == args.len() - 1 {
                Err(E::InvalidRequestAfter(targets.join(", ")))?;
            }
            let _ = args.remove(position);
            Ok(Some(args.remove(position)))
        } else {
            Ok(None)
        }
    }
    fn find_prev_to_opt(
        args: &mut Vec<String>,
        targets: &[&str],
    ) -> Result<Option<Option<String>>, E> {
        if let Some(position) = args.iter().position(|arg| targets.contains(&arg.as_str())) {
            if position == 0 {
                Ok(Some(None))
            } else {
                let arg = args.remove(position - 1);
                args.remove(position - 1);
                Ok(Some(Some(arg)))
            }
        } else {
            Ok(None)
        }
    }
    fn has(args: &mut Vec<String>, targets: &[&str]) -> Result<bool, E> {
        if let Some(position) = args.iter().position(|arg| targets.contains(&arg.as_str())) {
            let _ = args.remove(position);
            Ok(true)
        } else {
            Ok(false)
        }
    }
    fn key() -> String;
    fn read(args: &mut Vec<String>) -> Result<Option<Box<dyn Action>>, E>
    where
        Self: Sized;
    fn desc() -> Description
    where
        Self: Sized;
}
