// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p> </p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DataSourceS3Configuration {
    /// <p>The bucket name of the customer S3 bucket.</p>
    pub bucket_name: ::std::option::Option<::std::string::String>,
    /// <p> </p>
    pub object_key_prefix: ::std::option::Option<::std::string::String>,
}
impl DataSourceS3Configuration {
    /// <p>The bucket name of the customer S3 bucket.</p>
    pub fn bucket_name(&self) -> ::std::option::Option<&str> {
        self.bucket_name.as_deref()
    }
    /// <p> </p>
    pub fn object_key_prefix(&self) -> ::std::option::Option<&str> {
        self.object_key_prefix.as_deref()
    }
}
impl DataSourceS3Configuration {
    /// Creates a new builder-style object to manufacture [`DataSourceS3Configuration`](crate::types::DataSourceS3Configuration).
    pub fn builder() -> crate::types::builders::DataSourceS3ConfigurationBuilder {
        crate::types::builders::DataSourceS3ConfigurationBuilder::default()
    }
}

/// A builder for [`DataSourceS3Configuration`](crate::types::DataSourceS3Configuration).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct DataSourceS3ConfigurationBuilder {
    pub(crate) bucket_name: ::std::option::Option<::std::string::String>,
    pub(crate) object_key_prefix: ::std::option::Option<::std::string::String>,
}
impl DataSourceS3ConfigurationBuilder {
    /// <p>The bucket name of the customer S3 bucket.</p>
    pub fn bucket_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.bucket_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The bucket name of the customer S3 bucket.</p>
    pub fn set_bucket_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.bucket_name = input;
        self
    }
    /// <p>The bucket name of the customer S3 bucket.</p>
    pub fn get_bucket_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.bucket_name
    }
    /// <p> </p>
    pub fn object_key_prefix(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.object_key_prefix = ::std::option::Option::Some(input.into());
        self
    }
    /// <p> </p>
    pub fn set_object_key_prefix(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.object_key_prefix = input;
        self
    }
    /// <p> </p>
    pub fn get_object_key_prefix(&self) -> &::std::option::Option<::std::string::String> {
        &self.object_key_prefix
    }
    /// Consumes the builder and constructs a [`DataSourceS3Configuration`](crate::types::DataSourceS3Configuration).
    pub fn build(self) -> crate::types::DataSourceS3Configuration {
        crate::types::DataSourceS3Configuration {
            bucket_name: self.bucket_name,
            object_key_prefix: self.object_key_prefix,
        }
    }
}
