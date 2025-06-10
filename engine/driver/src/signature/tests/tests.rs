use super::*;

#[test]
fn test() {
    let mut driver = Driver::unbound(
        r#" 
component component_a() {
    task task_a() {
        let strvariable = "hey";
        strs::repeat("fd",12)
    }
};
"#,
        true,
    );
    driver.read().unwrap_or_else(|err| panic!("{err}"));
    let mut signature = driver
        .signature(107, None)
        .unwrap_or_else(|| panic!("Fail to get signature"));
    println!("{signature:?}");
}
