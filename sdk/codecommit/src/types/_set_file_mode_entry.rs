// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Information about the file mode changes.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct SetFileModeEntry {
    /// <p>The full path to the file, including the name of the file.</p>
    pub file_path: ::std::option::Option<::std::string::String>,
    /// <p>The file mode for the file.</p>
    pub file_mode: ::std::option::Option<crate::types::FileModeTypeEnum>,
}
impl SetFileModeEntry {
    /// <p>The full path to the file, including the name of the file.</p>
    pub fn file_path(&self) -> ::std::option::Option<&str> {
        self.file_path.as_deref()
    }
    /// <p>The file mode for the file.</p>
    pub fn file_mode(&self) -> ::std::option::Option<&crate::types::FileModeTypeEnum> {
        self.file_mode.as_ref()
    }
}
impl SetFileModeEntry {
    /// Creates a new builder-style object to manufacture [`SetFileModeEntry`](crate::types::SetFileModeEntry).
    pub fn builder() -> crate::types::builders::SetFileModeEntryBuilder {
        crate::types::builders::SetFileModeEntryBuilder::default()
    }
}

/// A builder for [`SetFileModeEntry`](crate::types::SetFileModeEntry).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct SetFileModeEntryBuilder {
    pub(crate) file_path: ::std::option::Option<::std::string::String>,
    pub(crate) file_mode: ::std::option::Option<crate::types::FileModeTypeEnum>,
}
impl SetFileModeEntryBuilder {
    /// <p>The full path to the file, including the name of the file.</p>
    pub fn file_path(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.file_path = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The full path to the file, including the name of the file.</p>
    pub fn set_file_path(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.file_path = input;
        self
    }
    /// <p>The full path to the file, including the name of the file.</p>
    pub fn get_file_path(&self) -> &::std::option::Option<::std::string::String> {
        &self.file_path
    }
    /// <p>The file mode for the file.</p>
    pub fn file_mode(mut self, input: crate::types::FileModeTypeEnum) -> Self {
        self.file_mode = ::std::option::Option::Some(input);
        self
    }
    /// <p>The file mode for the file.</p>
    pub fn set_file_mode(mut self, input: ::std::option::Option<crate::types::FileModeTypeEnum>) -> Self {
        self.file_mode = input;
        self
    }
    /// <p>The file mode for the file.</p>
    pub fn get_file_mode(&self) -> &::std::option::Option<crate::types::FileModeTypeEnum> {
        &self.file_mode
    }
    /// Consumes the builder and constructs a [`SetFileModeEntry`](crate::types::SetFileModeEntry).
    pub fn build(self) -> crate::types::SetFileModeEntry {
        crate::types::SetFileModeEntry {
            file_path: self.file_path,
            file_mode: self.file_mode,
        }
    }
}
