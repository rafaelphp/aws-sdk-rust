// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The configuration details of an Amazon S3 input or output bucket.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct S3DataConfig {
    /// <p>The file path of the Amazon S3 bucket.</p>
    pub path: ::std::option::Option<::std::string::String>,
    /// <p>The Amazon Resource Name (ARN) of the Key Management Service (KMS) key that Amazon Personalize uses to encrypt or decrypt the input and output files.</p>
    pub kms_key_arn: ::std::option::Option<::std::string::String>,
}
impl S3DataConfig {
    /// <p>The file path of the Amazon S3 bucket.</p>
    pub fn path(&self) -> ::std::option::Option<&str> {
        self.path.as_deref()
    }
    /// <p>The Amazon Resource Name (ARN) of the Key Management Service (KMS) key that Amazon Personalize uses to encrypt or decrypt the input and output files.</p>
    pub fn kms_key_arn(&self) -> ::std::option::Option<&str> {
        self.kms_key_arn.as_deref()
    }
}
impl S3DataConfig {
    /// Creates a new builder-style object to manufacture [`S3DataConfig`](crate::types::S3DataConfig).
    pub fn builder() -> crate::types::builders::S3DataConfigBuilder {
        crate::types::builders::S3DataConfigBuilder::default()
    }
}

/// A builder for [`S3DataConfig`](crate::types::S3DataConfig).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct S3DataConfigBuilder {
    pub(crate) path: ::std::option::Option<::std::string::String>,
    pub(crate) kms_key_arn: ::std::option::Option<::std::string::String>,
}
impl S3DataConfigBuilder {
    /// <p>The file path of the Amazon S3 bucket.</p>
    pub fn path(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.path = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The file path of the Amazon S3 bucket.</p>
    pub fn set_path(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.path = input;
        self
    }
    /// <p>The file path of the Amazon S3 bucket.</p>
    pub fn get_path(&self) -> &::std::option::Option<::std::string::String> {
        &self.path
    }
    /// <p>The Amazon Resource Name (ARN) of the Key Management Service (KMS) key that Amazon Personalize uses to encrypt or decrypt the input and output files.</p>
    pub fn kms_key_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.kms_key_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the Key Management Service (KMS) key that Amazon Personalize uses to encrypt or decrypt the input and output files.</p>
    pub fn set_kms_key_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.kms_key_arn = input;
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the Key Management Service (KMS) key that Amazon Personalize uses to encrypt or decrypt the input and output files.</p>
    pub fn get_kms_key_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.kms_key_arn
    }
    /// Consumes the builder and constructs a [`S3DataConfig`](crate::types::S3DataConfig).
    pub fn build(self) -> crate::types::S3DataConfig {
        crate::types::S3DataConfig {
            path: self.path,
            kms_key_arn: self.kms_key_arn,
        }
    }
}
