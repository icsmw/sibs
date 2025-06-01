mod tests;

use crate::*;
use runtime::*;

#[macro_export]
macro_rules! completion_fn_return {
    ($test_name:ident, $content:literal, $expected_ty:expr, $expected_variables:expr, $pos:expr ) => {
        paste::item! {
            #[test]
            fn [< completion_fn_return_ $test_name >]() {
                use $crate::*;

                let mut driver = Driver::unbound($content, true);
                driver.read().unwrap_or_else(|err| panic!("{err}"));
                let mut completion = driver
                    .completion($pos, None)
                    .unwrap_or_else(|| panic!("Fail to get completion"));
                let suggestions = completion
                    .suggest()
                    .unwrap_or_else(|err| panic!("{err}"))
                    .unwrap_or_else(|| panic!("Fail to get suggestions"));
                assert!(!suggestions.is_empty());

                println!("Suggested: {:?}", suggestions);

                let suggested_variables: Vec<(&String, &Option<Ty>)> = suggestions
                    .iter()
                    .filter_map(|suggestion| match &suggestion.target {
                        CompletionMatch::Variable(name, var_ty) => Some((name, var_ty)),
                        CompletionMatch::Function(..) => None,
                    })
                    .collect();
                if suggested_variables.len() != $expected_variables.len() {
                    eprintln!("Expected variables: {:?}", $expected_variables);
                    eprintln!("Suggested variables: {:?}", suggested_variables);
                }
                assert_eq!(suggested_variables.len(), $expected_variables.len());
                for (name, ty) in suggested_variables {
                    let Some(ty) = ty else {
                        panic!("Variable {name} suggested, but type isn't determinated")
                    };
                    if !$expected_variables.contains(&name.as_str()) {
                        panic!("Variable {name} suggested, but isn't expected");
                    }
                    if ty != &$expected_ty {
                        panic!("Variable {name} suggested, but type is dismatch ({ty:?} != {:?})", $expected_ty);
                    }
                }

                assert!(!suggestions
                .iter()
                .any(|suggestion| match &suggestion.target {
                    CompletionMatch::Variable(..) => {
                        false
                    }
                    CompletionMatch::Function(name, _, return_ty) => return_ty
                        .as_ref()
                        .map(|arg_ty| if !arg_ty.compatible(&$expected_ty) {
                            eprintln!("Function {name} has unexpected type of the first argument: {return_ty:?} vs {:?}", $expected_ty);
                            true
                        } else {
                            false
                        })
                        .unwrap_or_default(),
                }));
            }
        }
    };
}

#[macro_export]
macro_rules! no_completion {
    ($test_name:ident, $content:literal, $pos:expr ) => {
        paste::item! {
            #[test]
            fn [< no_completion_ $test_name >]() {
                use $crate::*;

                let mut driver = Driver::unbound($content, true);
                driver.read().unwrap_or_else(|err| panic!("{err}"));
                let mut completion = driver
                    .completion($pos, None)
                    .unwrap_or_else(|| panic!("Fail to get completion"));
                let suggestions = completion
                    .suggest()
                    .unwrap_or_else(|err| panic!("{err}"));
                assert!(suggestions.is_none());
            }
        }
    };
}
