use breachops_nav_utils::{AppMetadata, Version};

/// Static const object that defines the base metadata for this application
#[allow(dead_code)]
pub static APP_METADATA: AppMetadata = AppMetadata::new(
    "skyeBreach",
    "skyeBreach",
    Version::new(0, 1, 0),
    "Lorem ipsum dolor sit amet, consectetur.",
);
