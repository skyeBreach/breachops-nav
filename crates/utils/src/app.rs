use semver::Version;
use std::fmt;

/*
- TODO: Convert this into a read-only AppMetadata and an App Metadata builder, to avoid it being changed
- TODO: Have it generate from config file
*/

/// The read-only metadata associated with an instance of the app.
pub struct AppMetadata {
    /// Application name
    name: &'static str,
    /// Application author
    author: &'static str,
    /// Application description
    description: Option<&'static str>,
    /// Application version
    version: Version,
}

impl Default for AppMetadata {
    fn default() -> Self {
        Self::DEFAULT
    }
}

impl AppMetadata {
    /// Const default object, to allow apps to have access to a static or const object
    pub const DEFAULT: Self = Self {
        name: "Un-Named App",
        author: "Jane Doe",
        description: None,
        version: Version::new(0, 1, 0),
    };

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

    /// Constructs a new constant instance of AppMetadata
    pub const fn new(name: &'static str, author: &'static str, version: Version, description: &'static str) -> Self {
        Self {
            name: name,
            author: author,
            description: Some(description),
            version: version,
        }
    }
}

impl fmt::Display for AppMetadata {
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
        let app = AppMetadata::new(
            "Test App",
            "Jane Doe",
            Version::new(0, 1, 0),
            "Lorem ipsum dolor sit amet, consectetur.",
        );

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
