mod tests;

use crate::*;
use runtime::*;

#[macro_export]
macro_rules! positioning {
    ($test_name:ident, $content:literal, $pos:expr ) => {
        paste::item! {
            #[test]
            fn [< positioning_ $test_name >]() {
                use $crate::*;

                let mut driver = Driver::unbound($content, true);
                driver.read().unwrap_or_else(|err| panic!("{err}"));
                let mut positioning = driver
                    .positioning($pos, None)
                    .unwrap_or_else(|| panic!("Fail to get positioning"));



            }
        }
    };
}
