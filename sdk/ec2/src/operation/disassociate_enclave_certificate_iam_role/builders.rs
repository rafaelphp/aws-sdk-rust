// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::disassociate_enclave_certificate_iam_role::_disassociate_enclave_certificate_iam_role_output::DisassociateEnclaveCertificateIamRoleOutputBuilder;

pub use crate::operation::disassociate_enclave_certificate_iam_role::_disassociate_enclave_certificate_iam_role_input::DisassociateEnclaveCertificateIamRoleInputBuilder;

impl DisassociateEnclaveCertificateIamRoleInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::disassociate_enclave_certificate_iam_role::DisassociateEnclaveCertificateIamRoleOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::disassociate_enclave_certificate_iam_role::DisassociateEnclaveCertificateIamRoleError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.disassociate_enclave_certificate_iam_role();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DisassociateEnclaveCertificateIamRole`.
///
/// <p>Disassociates an IAM role from an Certificate Manager (ACM) certificate. Disassociating an IAM role from an ACM certificate removes the Amazon S3 object that contains the certificate, certificate chain, and encrypted private key from the Amazon S3 bucket. It also revokes the IAM role's permission to use the KMS key used to encrypt the private key. This effectively revokes the role's permission to use the certificate.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DisassociateEnclaveCertificateIamRoleFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::disassociate_enclave_certificate_iam_role::builders::DisassociateEnclaveCertificateIamRoleInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl DisassociateEnclaveCertificateIamRoleFluentBuilder {
    /// Creates a new `DisassociateEnclaveCertificateIamRole`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DisassociateEnclaveCertificateIamRole as a reference.
    pub fn as_input(
        &self,
    ) -> &crate::operation::disassociate_enclave_certificate_iam_role::builders::DisassociateEnclaveCertificateIamRoleInputBuilder {
        &self.inner
    }
    /// Sends the request and returns the response.
    ///
    /// If an error occurs, an `SdkError` will be returned with additional details that
    /// can be matched against.
    ///
    /// By default, any retryable failures will be retried twice. Retry behavior
    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
    /// set when configuring the client.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::operation::disassociate_enclave_certificate_iam_role::DisassociateEnclaveCertificateIamRoleOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::disassociate_enclave_certificate_iam_role::DisassociateEnclaveCertificateIamRoleError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self.inner.build().map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        let runtime_plugins =
            crate::operation::disassociate_enclave_certificate_iam_role::DisassociateEnclaveCertificateIamRole::operation_runtime_plugins(
                self.handle.runtime_plugins.clone(),
                &self.handle.conf,
                self.config_override,
            );
        crate::operation::disassociate_enclave_certificate_iam_role::DisassociateEnclaveCertificateIamRole::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent.
    // TODO(enableNewSmithyRuntimeCleanup): Remove `async` and `Result` once we switch to orchestrator
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::orchestrator::CustomizableOperation<
            crate::operation::disassociate_enclave_certificate_iam_role::DisassociateEnclaveCertificateIamRoleOutput,
            crate::operation::disassociate_enclave_certificate_iam_role::DisassociateEnclaveCertificateIamRoleError,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::disassociate_enclave_certificate_iam_role::DisassociateEnclaveCertificateIamRoleError>,
    > {
        ::std::result::Result::Ok(crate::client::customize::orchestrator::CustomizableOperation {
            customizable_send: ::std::boxed::Box::new(move |config_override| {
                ::std::boxed::Box::pin(async { self.config_override(config_override).send().await })
            }),
            config_override: None,
            interceptors: vec![],
            runtime_plugins: vec![],
        })
    }
    pub(crate) fn config_override(mut self, config_override: impl Into<crate::config::Builder>) -> Self {
        self.set_config_override(Some(config_override.into()));
        self
    }

    pub(crate) fn set_config_override(&mut self, config_override: Option<crate::config::Builder>) -> &mut Self {
        self.config_override = config_override;
        self
    }
    /// <p>The ARN of the ACM certificate from which to disassociate the IAM role.</p>
    pub fn certificate_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.certificate_arn(input.into());
        self
    }
    /// <p>The ARN of the ACM certificate from which to disassociate the IAM role.</p>
    pub fn set_certificate_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_certificate_arn(input);
        self
    }
    /// <p>The ARN of the ACM certificate from which to disassociate the IAM role.</p>
    pub fn get_certificate_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_certificate_arn()
    }
    /// <p>The ARN of the IAM role to disassociate.</p>
    pub fn role_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.role_arn(input.into());
        self
    }
    /// <p>The ARN of the IAM role to disassociate.</p>
    pub fn set_role_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_role_arn(input);
        self
    }
    /// <p>The ARN of the IAM role to disassociate.</p>
    pub fn get_role_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_role_arn()
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(mut self, input: bool) -> Self {
        self.inner = self.inner.dry_run(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn set_dry_run(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_dry_run(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn get_dry_run(&self) -> &::std::option::Option<bool> {
        self.inner.get_dry_run()
    }
}
