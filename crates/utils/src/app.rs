use semver::{BuildMetadata, Prerelease, Version};
use std::fmt;

/*
- TODO: Convert this into a read-only AppMetadata and an App Metadata builder, to avoid it being changed
*/

/// The read-only metadata associated with an instance of the app.
pub struct AppMetadata<'app> {
    /// Application name
    name: &'app str,
    /// Application author
    author: &'app str,
    /// Application description
    description: Option<&'app str>,
    /// Application version
    version: Version,
}

impl<'app> Default for AppMetadata<'app> {
    fn default() -> Self {
        Self {
            name: "Un-Named App",
            author: "Jane Doe",
            description: None,
            version: Version::new(0, 1, 0),
        }
    }
}

impl<'app> AppMetadata<'app> {
    /// Getter for the name of the app instance as a non-mutable string slice
    ///
    /// This has been implemented to enforce a read-only struct field, to avoid app users editing [AppMetadata].
    pub fn name(&self) -> &str {
        return &self.name;
    }

    /// Getter for the app's author as a non-mutable string slice
    ///
    /// This has been implemented to enforce a read-only struct field, to avoid app users editing [AppMetadata].
    pub fn author(&self) -> &str {
        return &self.author;
    }

    /// Getter for the description as a non-mutable string slice
    ///
    /// This has been implemented to enforce a read-only struct field, to avoid app users editing [AppMetadata].
    pub fn description(&self) -> &str {
        return &self.description.unwrap();
    }

    /// Getter for the version object as a non-mutable [Version] Object
    ///
    /// This has been implemented to enforce a read-only struct field, to avoid app users editing [AppMetadata].
    pub fn version(&self) -> &Version {
        return &self.version;
    }

    /// Creates a new app metadata instance from a name and author
    pub fn new(name: &'app str, author: &'app str) -> Self {
        Self {
            name: name.into(),
            author: author.into(),
            ..Self::default()
        }
    }

    // TODO: Move to builder
    pub fn set_description(mut self, description: &'app str) -> Self {
        self.description = Some(description.into());
        return self;
    }

    // TODO: Move to builder
    pub fn set_version(mut self, version: Version) -> Self {
        self.version = version;
        return self;
    }

    // TODO: Move to builder
    pub fn set_version_major(mut self, major: u64) -> Self {
        self.version.major = major;
        return self;
    }

    // TODO: Move to builder
    pub fn set_version_minor(mut self, minor: u64) -> Self {
        self.version.minor = minor;
        return self;
    }

    // TODO: Move to builder
    pub fn set_version_patch(mut self, patch: u64) -> Self {
        self.version.patch = patch;
        return self;
    }

    // TODO: Move to builder
    pub fn set_version_pre(mut self, pre: Prerelease) -> Self {
        self.version.pre = pre;
        return self;
    }

    // TODO: Move to builder
    pub fn set_version_build(mut self, build: BuildMetadata) -> Self {
        self.version.build = build;
        return self;
    }
}

impl<'app> fmt::Display for AppMetadata<'app> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(
            f,
            "App Metadata: {{\n\
            \x20    Name: {},\n\
            \x20    Author: {},\n\
            \x20    Version: {},\n\
            \x20    Description: \x22{}\x22\n\
            }}",
            self.name,
            self.author,
            self.version,
            self.description.unwrap(),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ra_ap_test_utils::assert_eq_text;

    #[test]
    fn format() {
        let app = AppMetadata {
            name: "Test App",
            author: "Jane Doe",
            description: Some("Lorem ipsum dolor sit amet, consectetur."),
            version: Version::new(0, 1, 0),
        };
        assert_eq_text!(
            &format!("{}", app),
            "App Metadata: {\n\
            \x20    Name: Test App,\n\
            \x20    Author: Jane Doe,\n\
            \x20    Version: 0.1.0,\n\
            \x20    Description: \x22Lorem ipsum dolor sit amet, consectetur.\x22\n\
            }"
        );
    }
}
