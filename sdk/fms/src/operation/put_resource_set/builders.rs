// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::put_resource_set::_put_resource_set_output::PutResourceSetOutputBuilder;

pub use crate::operation::put_resource_set::_put_resource_set_input::PutResourceSetInputBuilder;

impl PutResourceSetInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::put_resource_set::PutResourceSetOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::put_resource_set::PutResourceSetError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.put_resource_set();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `PutResourceSet`.
///
/// <p>Creates the resource set.</p>
/// <p>An Firewall Manager resource set defines the resources to import into an Firewall Manager policy from another Amazon Web Services service.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct PutResourceSetFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::put_resource_set::builders::PutResourceSetInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl PutResourceSetFluentBuilder {
    /// Creates a new `PutResourceSet`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the PutResourceSet as a reference.
    pub fn as_input(&self) -> &crate::operation::put_resource_set::builders::PutResourceSetInputBuilder {
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
        crate::operation::put_resource_set::PutResourceSetOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::put_resource_set::PutResourceSetError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self.inner.build().map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::put_resource_set::PutResourceSet::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::put_resource_set::PutResourceSet::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent.
    // TODO(enableNewSmithyRuntimeCleanup): Remove `async` and `Result` once we switch to orchestrator
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::orchestrator::CustomizableOperation<
            crate::operation::put_resource_set::PutResourceSetOutput,
            crate::operation::put_resource_set::PutResourceSetError,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::put_resource_set::PutResourceSetError>,
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
    /// <p>Details about the resource set to be created or updated.&gt;</p>
    pub fn resource_set(mut self, input: crate::types::ResourceSet) -> Self {
        self.inner = self.inner.resource_set(input);
        self
    }
    /// <p>Details about the resource set to be created or updated.&gt;</p>
    pub fn set_resource_set(mut self, input: ::std::option::Option<crate::types::ResourceSet>) -> Self {
        self.inner = self.inner.set_resource_set(input);
        self
    }
    /// <p>Details about the resource set to be created or updated.&gt;</p>
    pub fn get_resource_set(&self) -> &::std::option::Option<crate::types::ResourceSet> {
        self.inner.get_resource_set()
    }
    /// Appends an item to `TagList`.
    ///
    /// To override the contents of this collection use [`set_tag_list`](Self::set_tag_list).
    ///
    /// <p>Retrieves the tags associated with the specified resource set. Tags are key:value pairs that you can use to categorize and manage your resources, for purposes like billing. For example, you might set the tag key to "customer" and the value to the customer name or ID. You can specify one or more tags to add to each Amazon Web Services resource, up to 50 tags for a resource.</p>
    pub fn tag_list(mut self, input: crate::types::Tag) -> Self {
        self.inner = self.inner.tag_list(input);
        self
    }
    /// <p>Retrieves the tags associated with the specified resource set. Tags are key:value pairs that you can use to categorize and manage your resources, for purposes like billing. For example, you might set the tag key to "customer" and the value to the customer name or ID. You can specify one or more tags to add to each Amazon Web Services resource, up to 50 tags for a resource.</p>
    pub fn set_tag_list(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>) -> Self {
        self.inner = self.inner.set_tag_list(input);
        self
    }
    /// <p>Retrieves the tags associated with the specified resource set. Tags are key:value pairs that you can use to categorize and manage your resources, for purposes like billing. For example, you might set the tag key to "customer" and the value to the customer name or ID. You can specify one or more tags to add to each Amazon Web Services resource, up to 50 tags for a resource.</p>
    pub fn get_tag_list(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Tag>> {
        self.inner.get_tag_list()
    }
}
