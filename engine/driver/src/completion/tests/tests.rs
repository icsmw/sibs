use super::*;

completion_fn_return!(
    test_1,
    r#" 
component component_a() {
    task task_a() {
        let strvariable = "hey";
        let newstring: str = strvariable;
    }
};
"#,
    Ty::Determined(DeterminedTy::Str),
    ["strvariable"],
    112
);

completion_fn_return!(
    test_2,
    r#" 
component component_a() {
    task task_a() {
        let strvariable_a = "hey";
        let strvariable_b = "hey";
        let newstring: str = strvariable_b;
    }
};
"#,
    Ty::Determined(DeterminedTy::Str),
    ["strvariable_a", "strvariable_b"],
    150
);

completion_fn_return!(
    test_3,
    r#"
mod aaa {
    fn sum(a: num, b: num) {
        a + b;
    };
};      
component component_a() {
    task task_a() {
        let sumvariable: num;
        sumvariable.sum;
        let strvariable = "hey";
        strvariable.sub;
        let newstring: str = strvariable;
        let variable_a = 1;
        let variable_b = 1;
        let variable_c = variable_a + variable_b;
        let varibale_d = if eeevariaeee > 1 {
            let sub_var = env;
            variable_a;
        } else {
            variable_b;
        }
        variable.fns::sum(a);
    }
};
"#,
    Ty::Determined(DeterminedTy::Str),
    ["strvariable", "newstring"],
    261
);

// #[test]
// fn test2() {
//     let pos = 261;
//     let expected_variables = ["strvariable", "newstring"];
//     let expected_ty = Ty::Determined(DeterminedTy::Str);
//     let mut driver = Driver::unbound(
//         r#"
// mod aaa {
//     fn sum(a: num, b: num) {
//         a + b;
//     };
// };
// component component_a() {
//     task task_a() {
//         let sumvariable: num;
//         sumvariable.sum;
//         let strvariable = "hey";
//         strvariable.sub;
//         let newstring: str = strvariable;
//         let variable_a = 1;
//         let variable_b = 1;
//         let variable_c = variable_a + variable_b;
//         let varibale_d = if eeevariaeee > 1 {
//             let sub_var = env;
//             variable_a;
//         } else {
//             variable_b;
//         }
//         variable.fns::sum(a);
//     }
// };
// "#,
//         true,
//     );
//     driver.read().unwrap_or_else(|err| panic!("{err}"));
//     let mut completion = driver
//         .completion(pos, None)
//         .unwrap_or_else(|| panic!("Fail to get completion"));
//     let suggestions = completion
//         .suggest()
//         .unwrap_or_else(|err| panic!("{err}"))
//         .unwrap_or_else(|| panic!("Fail to get suggestions"));
//     assert!(!suggestions.is_empty());
//     let suggested_variables: Vec<(&String, &Option<Ty>)> = suggestions
//         .iter()
//         .filter_map(|suggestion| match &suggestion.target {
//             CompletionMatch::Variable(name, var_ty) => Some((name, var_ty)),
//             CompletionMatch::Function(..) => None,
//         })
//         .collect();
//     assert_eq!(suggested_variables.len(), expected_variables.len());
//     for (name, ty) in suggested_variables {
//         let Some(ty) = ty else {
//             panic!("Variable {name} suggested, but type isn't determinated")
//         };
//         if !expected_variables.contains(&name.as_str()) {
//             panic!("Variable {name} suggested, but isn't expected");
//         }
//         if ty != &expected_ty {
//             panic!("Variable {name} suggested, but type is dismatch ({ty:?} != {expected_ty:?})");
//         }
//     }

//     assert!(!suggestions
//     .iter()
//     .any(|suggestion| match &suggestion.target {
//         CompletionMatch::Variable(..) => {
//             false
//         }
//         CompletionMatch::Function(name, _, return_ty) => return_ty
//             .as_ref()
//             .map(|arg_ty| if !arg_ty.compatible(&expected_ty) {
//                 eprintln!("Function {name} has unexpected type of the first argument: {return_ty:?} vs {expected_ty:?}");
//                 true
//             } else {
//                 false
//             })
//             .unwrap_or_default(),
//     }));
// }

// #[test]
// fn test() {
//     let mut driver = Driver::unbound(
//         r#"
// mod aaa {
//     fn sum(a: num, b: num) {
//         a + b;
//     };
// };
// component component_a() {
//     task task_a() {
//         let sumvariable: num;
//         sumvariable.sum;
//         let strvariable = "hey";
//         strvariable.sub;
//         let newstring: str = strvariable;
//         let variable_a = 1;
//         let variable_b = 1;
//         let variable_c = variable_a + variable_b;
//         let varibale_d = if eeevariaeee > 1 {
//             let sub_var = env;
//             variable_a;
//         } else {
//             variable_b;
//         }
//         variable.fns::sum(a);
//     }
// };
// "#,
//         true,
//     );
//     driver.read().unwrap();
//     driver.print_errs().unwrap();
//     let mut completion = driver.completion(368, None).unwrap();
//     println!("Suggestions: {:?}", completion.suggest());
// }
