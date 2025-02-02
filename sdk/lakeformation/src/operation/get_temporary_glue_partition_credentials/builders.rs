// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_temporary_glue_partition_credentials::_get_temporary_glue_partition_credentials_output::GetTemporaryGluePartitionCredentialsOutputBuilder;

pub use crate::operation::get_temporary_glue_partition_credentials::_get_temporary_glue_partition_credentials_input::GetTemporaryGluePartitionCredentialsInputBuilder;

impl GetTemporaryGluePartitionCredentialsInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::get_temporary_glue_partition_credentials::GetTemporaryGluePartitionCredentialsOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::get_temporary_glue_partition_credentials::GetTemporaryGluePartitionCredentialsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.get_temporary_glue_partition_credentials();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `GetTemporaryGluePartitionCredentials`.
///
/// <p>This API is identical to <code>GetTemporaryTableCredentials</code> except that this is used when the target Data Catalog resource is of type Partition. Lake Formation restricts the permission of the vended credentials with the same scope down policy which restricts access to a single Amazon S3 prefix.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GetTemporaryGluePartitionCredentialsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_temporary_glue_partition_credentials::builders::GetTemporaryGluePartitionCredentialsInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl GetTemporaryGluePartitionCredentialsFluentBuilder {
    /// Creates a new `GetTemporaryGluePartitionCredentials`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the GetTemporaryGluePartitionCredentials as a reference.
    pub fn as_input(
        &self,
    ) -> &crate::operation::get_temporary_glue_partition_credentials::builders::GetTemporaryGluePartitionCredentialsInputBuilder {
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
        crate::operation::get_temporary_glue_partition_credentials::GetTemporaryGluePartitionCredentialsOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::get_temporary_glue_partition_credentials::GetTemporaryGluePartitionCredentialsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self.inner.build().map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        let runtime_plugins =
            crate::operation::get_temporary_glue_partition_credentials::GetTemporaryGluePartitionCredentials::operation_runtime_plugins(
                self.handle.runtime_plugins.clone(),
                &self.handle.conf,
                self.config_override,
            );
        crate::operation::get_temporary_glue_partition_credentials::GetTemporaryGluePartitionCredentials::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent.
    // TODO(enableNewSmithyRuntimeCleanup): Remove `async` and `Result` once we switch to orchestrator
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::orchestrator::CustomizableOperation<
            crate::operation::get_temporary_glue_partition_credentials::GetTemporaryGluePartitionCredentialsOutput,
            crate::operation::get_temporary_glue_partition_credentials::GetTemporaryGluePartitionCredentialsError,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::get_temporary_glue_partition_credentials::GetTemporaryGluePartitionCredentialsError>,
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
    /// <p>The ARN of the partitions' table.</p>
    pub fn table_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.table_arn(input.into());
        self
    }
    /// <p>The ARN of the partitions' table.</p>
    pub fn set_table_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_table_arn(input);
        self
    }
    /// <p>The ARN of the partitions' table.</p>
    pub fn get_table_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_table_arn()
    }
    /// <p>A list of partition values identifying a single partition.</p>
    pub fn partition(mut self, input: crate::types::PartitionValueList) -> Self {
        self.inner = self.inner.partition(input);
        self
    }
    /// <p>A list of partition values identifying a single partition.</p>
    pub fn set_partition(mut self, input: ::std::option::Option<crate::types::PartitionValueList>) -> Self {
        self.inner = self.inner.set_partition(input);
        self
    }
    /// <p>A list of partition values identifying a single partition.</p>
    pub fn get_partition(&self) -> &::std::option::Option<crate::types::PartitionValueList> {
        self.inner.get_partition()
    }
    /// Appends an item to `Permissions`.
    ///
    /// To override the contents of this collection use [`set_permissions`](Self::set_permissions).
    ///
    /// <p>Filters the request based on the user having been granted a list of specified permissions on the requested resource(s).</p>
    pub fn permissions(mut self, input: crate::types::Permission) -> Self {
        self.inner = self.inner.permissions(input);
        self
    }
    /// <p>Filters the request based on the user having been granted a list of specified permissions on the requested resource(s).</p>
    pub fn set_permissions(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Permission>>) -> Self {
        self.inner = self.inner.set_permissions(input);
        self
    }
    /// <p>Filters the request based on the user having been granted a list of specified permissions on the requested resource(s).</p>
    pub fn get_permissions(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Permission>> {
        self.inner.get_permissions()
    }
    /// <p>The time period, between 900 and 21,600 seconds, for the timeout of the temporary credentials.</p>
    pub fn duration_seconds(mut self, input: i32) -> Self {
        self.inner = self.inner.duration_seconds(input);
        self
    }
    /// <p>The time period, between 900 and 21,600 seconds, for the timeout of the temporary credentials.</p>
    pub fn set_duration_seconds(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_duration_seconds(input);
        self
    }
    /// <p>The time period, between 900 and 21,600 seconds, for the timeout of the temporary credentials.</p>
    pub fn get_duration_seconds(&self) -> &::std::option::Option<i32> {
        self.inner.get_duration_seconds()
    }
    /// <p>A structure representing context to access a resource (column names, query ID, etc).</p>
    pub fn audit_context(mut self, input: crate::types::AuditContext) -> Self {
        self.inner = self.inner.audit_context(input);
        self
    }
    /// <p>A structure representing context to access a resource (column names, query ID, etc).</p>
    pub fn set_audit_context(mut self, input: ::std::option::Option<crate::types::AuditContext>) -> Self {
        self.inner = self.inner.set_audit_context(input);
        self
    }
    /// <p>A structure representing context to access a resource (column names, query ID, etc).</p>
    pub fn get_audit_context(&self) -> &::std::option::Option<crate::types::AuditContext> {
        self.inner.get_audit_context()
    }
    /// Appends an item to `SupportedPermissionTypes`.
    ///
    /// To override the contents of this collection use [`set_supported_permission_types`](Self::set_supported_permission_types).
    ///
    /// <p>A list of supported permission types for the partition. Valid values are <code>COLUMN_PERMISSION</code> and <code>CELL_FILTER_PERMISSION</code>.</p>
    pub fn supported_permission_types(mut self, input: crate::types::PermissionType) -> Self {
        self.inner = self.inner.supported_permission_types(input);
        self
    }
    /// <p>A list of supported permission types for the partition. Valid values are <code>COLUMN_PERMISSION</code> and <code>CELL_FILTER_PERMISSION</code>.</p>
    pub fn set_supported_permission_types(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::PermissionType>>) -> Self {
        self.inner = self.inner.set_supported_permission_types(input);
        self
    }
    /// <p>A list of supported permission types for the partition. Valid values are <code>COLUMN_PERMISSION</code> and <code>CELL_FILTER_PERMISSION</code>.</p>
    pub fn get_supported_permission_types(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::PermissionType>> {
        self.inner.get_supported_permission_types()
    }
}
