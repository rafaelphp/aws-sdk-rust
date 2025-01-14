// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::provision_permission_set::_provision_permission_set_output::ProvisionPermissionSetOutputBuilder;

pub use crate::operation::provision_permission_set::_provision_permission_set_input::ProvisionPermissionSetInputBuilder;

impl ProvisionPermissionSetInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::provision_permission_set::ProvisionPermissionSetOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::provision_permission_set::ProvisionPermissionSetError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.provision_permission_set();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ProvisionPermissionSet`.
///
/// <p>The process by which a specified permission set is provisioned to the specified target.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ProvisionPermissionSetFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::provision_permission_set::builders::ProvisionPermissionSetInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl ProvisionPermissionSetFluentBuilder {
    /// Creates a new `ProvisionPermissionSet`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ProvisionPermissionSet as a reference.
    pub fn as_input(&self) -> &crate::operation::provision_permission_set::builders::ProvisionPermissionSetInputBuilder {
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
        crate::operation::provision_permission_set::ProvisionPermissionSetOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::provision_permission_set::ProvisionPermissionSetError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self.inner.build().map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::provision_permission_set::ProvisionPermissionSet::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::provision_permission_set::ProvisionPermissionSet::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent.
    // TODO(enableNewSmithyRuntimeCleanup): Remove `async` and `Result` once we switch to orchestrator
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::orchestrator::CustomizableOperation<
            crate::operation::provision_permission_set::ProvisionPermissionSetOutput,
            crate::operation::provision_permission_set::ProvisionPermissionSetError,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::provision_permission_set::ProvisionPermissionSetError>,
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
    /// <p>The ARN of the IAM Identity Center instance under which the operation will be executed. For more information about ARNs, see <a href="/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs) and AWS Service Namespaces</a> in the <i>AWS General Reference</i>.</p>
    pub fn instance_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.instance_arn(input.into());
        self
    }
    /// <p>The ARN of the IAM Identity Center instance under which the operation will be executed. For more information about ARNs, see <a href="/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs) and AWS Service Namespaces</a> in the <i>AWS General Reference</i>.</p>
    pub fn set_instance_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_instance_arn(input);
        self
    }
    /// <p>The ARN of the IAM Identity Center instance under which the operation will be executed. For more information about ARNs, see <a href="/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs) and AWS Service Namespaces</a> in the <i>AWS General Reference</i>.</p>
    pub fn get_instance_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_instance_arn()
    }
    /// <p>The ARN of the permission set.</p>
    pub fn permission_set_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.permission_set_arn(input.into());
        self
    }
    /// <p>The ARN of the permission set.</p>
    pub fn set_permission_set_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_permission_set_arn(input);
        self
    }
    /// <p>The ARN of the permission set.</p>
    pub fn get_permission_set_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_permission_set_arn()
    }
    /// <p>TargetID is an AWS account identifier, typically a 10-12 digit string (For example, 123456789012).</p>
    pub fn target_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.target_id(input.into());
        self
    }
    /// <p>TargetID is an AWS account identifier, typically a 10-12 digit string (For example, 123456789012).</p>
    pub fn set_target_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_target_id(input);
        self
    }
    /// <p>TargetID is an AWS account identifier, typically a 10-12 digit string (For example, 123456789012).</p>
    pub fn get_target_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_target_id()
    }
    /// <p>The entity type for which the assignment will be created.</p>
    pub fn target_type(mut self, input: crate::types::ProvisionTargetType) -> Self {
        self.inner = self.inner.target_type(input);
        self
    }
    /// <p>The entity type for which the assignment will be created.</p>
    pub fn set_target_type(mut self, input: ::std::option::Option<crate::types::ProvisionTargetType>) -> Self {
        self.inner = self.inner.set_target_type(input);
        self
    }
    /// <p>The entity type for which the assignment will be created.</p>
    pub fn get_target_type(&self) -> &::std::option::Option<crate::types::ProvisionTargetType> {
        self.inner.get_target_type()
    }
}
