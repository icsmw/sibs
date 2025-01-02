pub fn get_fullname(path: &str) -> String {
    let parts = path.split("::").collect::<Vec<&str>>();
    let count = parts.len();
    parts
        .into_iter()
        .skip(count.saturating_sub(2))
        .collect::<Vec<&str>>()
        .join("::")
}

pub fn get_name(path: &str) -> String {
    path.split("::")
        .last()
        .expect("Module should have at least one member")
        .to_owned()
}

#[macro_export]
macro_rules! fn_name {
    () => {
        paste::item! {
            pub fn name() -> String {
                get_name(module_path!())
            }
        }
    };
}

#[macro_export]
macro_rules! import_embedded_fn {
    ($ref:expr, $module_ref:expr) => {
        paste::item! {
            $ref.add(
                $module_ref::fullname(),
                EmbeddedFnEntity {
                    uuid: Uuid::new_v4(),
                    fullname: $module_ref::fullname(),
                    name: $module_ref::name(),
                    args: $module_ref::args(),
                    result: $module_ref::returning(),
                    exec: $module_ref::executor,
                },
            )?;
        }
    };
}

#[macro_export]
macro_rules! declare_embedded_fn {
    ($args:expr, $returning:expr) => {
        paste::item! {
            pub fn name() -> String {
                get_name(module_path!())
            }
            pub fn fullname() -> String {
                get_fullname(module_path!())
            }
            pub fn args() -> Vec<Ty> {
                $args
            }
            pub fn returning() -> DeterminedTy {
                $returning
            }
        }
    };
}
