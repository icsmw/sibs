use crate::*;

pub fn register(efns: &mut EFns) -> Result<(), E> {
    #[import(strs)]
    /// Documentation placeholder
    fn repeat(target: String, count: usize) -> Result<String, E> {
        Ok(target.repeat(count))
    }
    #[import(strs)]
    /// Documentation placeholder
    fn to_ascii_lowercase(target: String) -> Result<String, E> {
        Ok(target.to_ascii_lowercase())
    }
    #[import(strs)]
    /// Documentation placeholder
    fn to_ascii_uppercase(target: String) -> Result<String, E> {
        Ok(target.to_ascii_uppercase())
    }
    #[import(strs)]
    /// Documentation placeholder
    fn to_lowercase(target: String) -> Result<String, E> {
        Ok(target.to_lowercase())
    }
    #[import(strs)]
    /// Documentation placeholder
    fn to_uppercase(target: String) -> Result<String, E> {
        Ok(target.to_uppercase())
    }
    #[import(strs)]
    /// Documentation placeholder
    fn replace(target: String, old: String, new: String) -> Result<String, E> {
        Ok(target.replace(old.as_str(), &new))
    }
    #[import(strs)]
    /// Documentation placeholder
    fn sub(target: String, from: usize, count: usize) -> Result<String, E> {
        let len = target.chars().count();
        if from >= len {
            return Ok(String::new());
        }
        let available_count = len - from;
        Ok(target
            .chars()
            .skip(from)
            .take(count.min(available_count))
            .collect())
    }
    #[import(strs)]
    /// Documentation placeholder
    fn split_off(mut target: String, at: usize) -> Result<String, E> {
        Ok(target.split_off(at))
    }
    #[import(strs)]
    /// Documentation placeholder
    fn trim(target: String) -> Result<String, E> {
        Ok(target.trim().to_string())
    }
    #[import(strs)]
    /// Documentation placeholder
    fn trim_end(target: String) -> Result<String, E> {
        Ok(target.trim_end().to_string())
    }
    #[import(strs)]
    /// Documentation placeholder
    fn trim_start(target: String) -> Result<String, E> {
        Ok(target.trim_start().to_string())
    }
    #[import(strs)]
    /// Documentation placeholder
    fn is_empty(target: String) -> Result<bool, E> {
        Ok(target.is_empty())
    }
    #[import(strs)]
    /// Documentation placeholder
    fn len(target: String) -> Result<usize, E> {
        Ok(target.len())
    }
    #[import(strs)]
    /// Documentation placeholder
    fn is_trimmed_empty(target: String) -> Result<bool, E> {
        Ok(target.trim().is_empty())
    }
    Ok(())
}

// #[cfg(test)]
// mod tests {
//     use crate::test_block;

//     test_block!(
//         repeat,
//         r#"
//             if "R".repeat(5) == "RRRRR" {
//                 true;
//             } else {
//                 false;
//             };
//         "#,
//         true
//     );

//     test_block!(
//         repeat_short,
//         r#"
//             "R".repeat(5) == "RRRRR";
//         "#,
//         true
//     );

//     test_block!(
//         to_ascii_lowercase,
//         r#"
//             if "R".to_ascii_lowercase() == "r" {
//                 true;
//             } else {
//                 false;
//             };
//         "#,
//         true
//     );

//     test_block!(
//         to_ascii_uppercase,
//         r#"
//             if "r".to_ascii_uppercase() == "R" {
//                 true;
//             } else {
//                 false;
//             };
//         "#,
//         true
//     );

//     test_block!(
//         to_lowercase,
//         r#"
//             if "R".to_lowercase() == "r" {
//                 true;
//             } else {
//                 false;
//             };
//         "#,
//         true
//     );

//     test_block!(
//         to_uppercase,
//         r#"
//             if "r".to_uppercase() == "R" {
//                 true;
//             } else {
//                 false;
//             };
//         "#,
//         true
//     );

//     test_block!(
//         sub,
//         r#"
//             $a = "Hello World!";
//             $b = $a.sub(0, 5);
//             $c = $a.str::sub(0, 5).str::sub(0, 2);
//             if $b == "Hello" && $c == "He" {
//                 true;
//             } else {
//                 false;
//             };
//         "#,
//         true
//     );

//     test_block!(
//         split_off,
//         r#"
//             if "Hello, World!".split_off(7) == "World!" {
//                 true;
//             } else {
//                 false;
//             };
//         "#,
//         true
//     );

//     test_block!(
//         trim,
//         r#"
//             if "   word   ".trim() == "word" {
//                 true;
//             } else {
//                 false;
//             };
//         "#,
//         true
//     );

//     test_block!(
//         trim_end,
//         r#"
//             if "   word   ".trim_end() == "   word" {
//                 true;
//             } else {
//                 false;
//             };
//         "#,
//         true
//     );

//     test_block!(
//         trim_start,
//         r#"
//             if "   word   ".trim_start() == "word   " {
//                 true;
//             } else {
//                 false;
//             };
//         "#,
//         true
//     );

//     test_block!(
//         len,
//         r#"
//             "12345".len() == 5;
//         "#,
//         true
//     );

//     test_block!(
//         is_empty,
//         r#"
//             "".is_empty();
//         "#,
//         true
//     );

//     test_block!(
//         is_trimmed_empty,
//         r#"
//             "   ".is_trimmed_empty();
//         "#,
//         true
//     );
// }
