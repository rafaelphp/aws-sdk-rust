// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::disconnect_from_service::_disconnect_from_service_output::DisconnectFromServiceOutputBuilder;

pub use crate::operation::disconnect_from_service::_disconnect_from_service_input::DisconnectFromServiceInputBuilder;

impl DisconnectFromServiceInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::disconnect_from_service::DisconnectFromServiceOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::disconnect_from_service::DisconnectFromServiceError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.disconnect_from_service();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DisconnectFromService`.
///
/// <p>Disconnects specific Source Servers from Application Migration Service. Data replication is stopped immediately. All AWS resources created by Application Migration Service for enabling the replication of these source servers will be terminated / deleted within 90 minutes. Launched Test or Cutover instances will NOT be terminated. If the agent on the source server has not been prevented from communicating with the Application Migration Service service, then it will receive a command to uninstall itself (within approximately 10 minutes). The following properties of the SourceServer will be changed immediately: dataReplicationInfo.dataReplicationState will be set to DISCONNECTED; The totalStorageBytes property for each of dataReplicationInfo.replicatedDisks will be set to zero; dataReplicationInfo.lagDuration and dataReplicationInfo.lagDuration will be nullified.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DisconnectFromServiceFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::disconnect_from_service::builders::DisconnectFromServiceInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl DisconnectFromServiceFluentBuilder {
    /// Creates a new `DisconnectFromService`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DisconnectFromService as a reference.
    pub fn as_input(&self) -> &crate::operation::disconnect_from_service::builders::DisconnectFromServiceInputBuilder {
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
        crate::operation::disconnect_from_service::DisconnectFromServiceOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::disconnect_from_service::DisconnectFromServiceError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self.inner.build().map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::disconnect_from_service::DisconnectFromService::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::disconnect_from_service::DisconnectFromService::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent.
    // TODO(enableNewSmithyRuntimeCleanup): Remove `async` and `Result` once we switch to orchestrator
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::orchestrator::CustomizableOperation<
            crate::operation::disconnect_from_service::DisconnectFromServiceOutput,
            crate::operation::disconnect_from_service::DisconnectFromServiceError,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::disconnect_from_service::DisconnectFromServiceError>,
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
    /// <p>Request to disconnect Source Server from service by Server ID.</p>
    pub fn source_server_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.source_server_id(input.into());
        self
    }
    /// <p>Request to disconnect Source Server from service by Server ID.</p>
    pub fn set_source_server_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_source_server_id(input);
        self
    }
    /// <p>Request to disconnect Source Server from service by Server ID.</p>
    pub fn get_source_server_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_source_server_id()
    }
    /// <p>Request to disconnect Source Server from service by Account ID.</p>
    pub fn account_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.account_id(input.into());
        self
    }
    /// <p>Request to disconnect Source Server from service by Account ID.</p>
    pub fn set_account_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_account_id(input);
        self
    }
    /// <p>Request to disconnect Source Server from service by Account ID.</p>
    pub fn get_account_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_account_id()
    }
}
