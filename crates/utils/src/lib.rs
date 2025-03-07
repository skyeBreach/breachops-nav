pub mod app;

pub use crate::app::AppMetadata;

#[allow(dead_code, unused_variables, unused_imports)]
#[cfg(test)]
mod tests {
    use super::*;
    use ra_ap_test_utils::assert_eq_text;
}
