// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_interconnect_loa::_describe_interconnect_loa_output::DescribeInterconnectLoaOutputBuilder;

pub use crate::operation::describe_interconnect_loa::_describe_interconnect_loa_input::DescribeInterconnectLoaInputBuilder;

impl DescribeInterconnectLoaInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::describe_interconnect_loa::DescribeInterconnectLoaOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::describe_interconnect_loa::DescribeInterconnectLoaError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.describe_interconnect_loa();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DescribeInterconnectLoa`.
///
/// <p>Deprecated. Use <code>DescribeLoa</code> instead.</p>
/// <p>Gets the LOA-CFA for the specified interconnect.</p>
/// <p>The Letter of Authorization - Connecting Facility Assignment (LOA-CFA) is a document that is used when establishing your cross connect to Amazon Web Services at the colocation facility. For more information, see <a href="https://docs.aws.amazon.com/directconnect/latest/UserGuide/Colocation.html">Requesting Cross Connects at Direct Connect Locations</a> in the <i>Direct Connect User Guide</i>.</p>
#[deprecated]
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DescribeInterconnectLoaFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::describe_interconnect_loa::builders::DescribeInterconnectLoaInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl DescribeInterconnectLoaFluentBuilder {
    /// Creates a new `DescribeInterconnectLoa`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DescribeInterconnectLoa as a reference.
    pub fn as_input(&self) -> &crate::operation::describe_interconnect_loa::builders::DescribeInterconnectLoaInputBuilder {
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
        crate::operation::describe_interconnect_loa::DescribeInterconnectLoaOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::describe_interconnect_loa::DescribeInterconnectLoaError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self.inner.build().map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::describe_interconnect_loa::DescribeInterconnectLoa::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::describe_interconnect_loa::DescribeInterconnectLoa::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent.
    // TODO(enableNewSmithyRuntimeCleanup): Remove `async` and `Result` once we switch to orchestrator
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::orchestrator::CustomizableOperation<
            crate::operation::describe_interconnect_loa::DescribeInterconnectLoaOutput,
            crate::operation::describe_interconnect_loa::DescribeInterconnectLoaError,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::describe_interconnect_loa::DescribeInterconnectLoaError>,
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
    /// <p>The ID of the interconnect.</p>
    pub fn interconnect_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.interconnect_id(input.into());
        self
    }
    /// <p>The ID of the interconnect.</p>
    pub fn set_interconnect_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_interconnect_id(input);
        self
    }
    /// <p>The ID of the interconnect.</p>
    pub fn get_interconnect_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_interconnect_id()
    }
    /// <p>The name of the service provider who establishes connectivity on your behalf. If you supply this parameter, the LOA-CFA lists the provider name alongside your company name as the requester of the cross connect.</p>
    pub fn provider_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.provider_name(input.into());
        self
    }
    /// <p>The name of the service provider who establishes connectivity on your behalf. If you supply this parameter, the LOA-CFA lists the provider name alongside your company name as the requester of the cross connect.</p>
    pub fn set_provider_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_provider_name(input);
        self
    }
    /// <p>The name of the service provider who establishes connectivity on your behalf. If you supply this parameter, the LOA-CFA lists the provider name alongside your company name as the requester of the cross connect.</p>
    pub fn get_provider_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_provider_name()
    }
    /// <p>The standard media type for the LOA-CFA document. The only supported value is application/pdf.</p>
    pub fn loa_content_type(mut self, input: crate::types::LoaContentType) -> Self {
        self.inner = self.inner.loa_content_type(input);
        self
    }
    /// <p>The standard media type for the LOA-CFA document. The only supported value is application/pdf.</p>
    pub fn set_loa_content_type(mut self, input: ::std::option::Option<crate::types::LoaContentType>) -> Self {
        self.inner = self.inner.set_loa_content_type(input);
        self
    }
    /// <p>The standard media type for the LOA-CFA document. The only supported value is application/pdf.</p>
    pub fn get_loa_content_type(&self) -> &::std::option::Option<crate::types::LoaContentType> {
        self.inner.get_loa_content_type()
    }
}
