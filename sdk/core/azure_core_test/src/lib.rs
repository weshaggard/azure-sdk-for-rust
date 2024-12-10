// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

#![doc = include_str!("../README.md")]

/// Live recording and playing back of client library tests.
pub mod recorded {
    pub use azure_core_test_macros::test;
}

pub use azure_core::test::TestMode;

/// Context information required by recorded client library tests.
///
/// This context is required for any recorded tests not attributed as `#[recorded::test(live)]`
/// to setup up the HTTP client to record or play back session records.
#[derive(Clone, Debug)]
pub struct TestContext {
    test_mode: TestMode,
    crate_dir: &'static str,
    test_name: &'static str,
}

impl TestContext {
    /// Not intended for use outside the `azure_core` crates.
    #[doc(hidden)]
    pub fn new(test_mode: TestMode, crate_dir: &'static str, test_name: &'static str) -> Self {
        Self {
            test_mode,
            crate_dir,
            test_name,
        }
    }

    /// Gets the current [`TestMode`].
    pub fn test_mode(&self) -> TestMode {
        self.test_mode
    }

    /// Gets the root directory of the crate under test.
    pub fn crate_dir(&self) -> &'static str {
        self.crate_dir
    }

    /// Gets the current test function name.
    pub fn test_name(&self) -> &'static str {
        self.test_name
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_content_new() {
        let ctx = TestContext::new(
            TestMode::default(),
            env!("CARGO_MANIFEST_DIR"),
            "test_content_new",
        );
        assert_eq!(ctx.test_mode(), TestMode::Playback);
        assert!(ctx
            .crate_dir()
            .replace("\\", "/")
            .ends_with("sdk/core/azure_core_test"));
        assert_eq!(ctx.test_name(), "test_content_new");
    }
}
