// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[derive(Debug)]
            pub(crate) struct Handle {
                pub(crate) client: aws_smithy_client::Client<aws_smithy_client::erase::DynConnector, aws_smithy_client::erase::DynMiddleware<aws_smithy_client::erase::DynConnector>>,
                pub(crate) conf: crate::Config,
            }

            /// Client for Amazon Connect Contact Lens
                    ///
                    /// Client for invoking operations on Amazon Connect Contact Lens. Each operation on Amazon Connect Contact Lens is a method on this
                    /// this struct. `.send()` MUST be invoked on the generated operations to dispatch the request to the service.
///
                        /// # Examples
                        /// **Constructing a client and invoking an operation**
                        /// ```rust,no_run
                        /// # async fn docs() {
                        ///     // create a shared configuration. This can be used & shared between multiple service clients.
                        ///     let shared_config = aws_config::load_from_env().await;
                        ///     let client = aws_sdk_connectcontactlens::Client::new(&shared_config);
                        ///     // invoke an operation
                        ///     /* let rsp = client
                        ///         .<operation_name>().
                        ///         .<param>("some value")
                        ///         .send().await; */
                        /// # }
                        /// ```
                        /// **Constructing a client with custom configuration**
                        /// ```rust,no_run
                        /// use aws_config::retry::RetryConfig;
                        /// # async fn docs() {
                        /// let shared_config = aws_config::load_from_env().await;
                        /// let config = aws_sdk_connectcontactlens::config::Builder::from(&shared_config)
                        ///   .retry_config(RetryConfig::disabled())
                        ///   .build();
                        /// let client = aws_sdk_connectcontactlens::Client::from_conf(config);
                        /// # }
            #[derive(std::fmt::Debug)]
            pub struct Client {
                handle: std::sync::Arc<Handle>
            }

            impl std::clone::Clone for Client {
                fn clone(&self) -> Self {
                    Self { handle: self.handle.clone() }
                }
            }

            #[doc(inline)]
            pub use aws_smithy_client::Builder;

            impl From<aws_smithy_client::Client<aws_smithy_client::erase::DynConnector, aws_smithy_client::erase::DynMiddleware<aws_smithy_client::erase::DynConnector>>> for Client {
                fn from(client: aws_smithy_client::Client<aws_smithy_client::erase::DynConnector, aws_smithy_client::erase::DynMiddleware<aws_smithy_client::erase::DynConnector>>) -> Self {
                    Self::with_config(client, crate::Config::builder().build())
                }
            }

            impl Client {
                /// Creates a client with the given service configuration.
                pub fn with_config(client: aws_smithy_client::Client<aws_smithy_client::erase::DynConnector, aws_smithy_client::erase::DynMiddleware<aws_smithy_client::erase::DynConnector>>, conf: crate::Config) -> Self {
                    Self {
                        handle: std::sync::Arc::new(Handle {
                            client,
                            conf,
                        })
                    }
                }

                /// Returns the client's configuration.
                pub fn conf(&self) -> &crate::Config {
                    &self.handle.conf
                }
            }
impl Client  {
    /// Constructs a fluent builder for the [`ListRealtimeContactAnalysisSegments`](crate::client::fluent_builders::ListRealtimeContactAnalysisSegments) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::ListRealtimeContactAnalysisSegments::into_paginator).
                        ///
                        /// - The fluent builder is configurable:
                        ///   - [`instance_id(impl Into<String>)`](crate::client::fluent_builders::ListRealtimeContactAnalysisSegments::instance_id) / [`set_instance_id(Option<String>)`](crate::client::fluent_builders::ListRealtimeContactAnalysisSegments::set_instance_id): <p>The identifier of the Amazon Connect instance.</p>
    ///   - [`contact_id(impl Into<String>)`](crate::client::fluent_builders::ListRealtimeContactAnalysisSegments::contact_id) / [`set_contact_id(Option<String>)`](crate::client::fluent_builders::ListRealtimeContactAnalysisSegments::set_contact_id): <p>The identifier of the contact.</p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::ListRealtimeContactAnalysisSegments::max_results) / [`set_max_results(i32)`](crate::client::fluent_builders::ListRealtimeContactAnalysisSegments::set_max_results): <p>The maximimum number of results to return per page.</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListRealtimeContactAnalysisSegments::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListRealtimeContactAnalysisSegments::set_next_token): <p>The token for the next set of results. Use the value returned in the previous response in the next request to retrieve the next set of results.</p>
                        /// - On success, responds with [`ListRealtimeContactAnalysisSegmentsOutput`](crate::output::ListRealtimeContactAnalysisSegmentsOutput) with field(s):
                        ///   - [`segments(Option<Vec<RealtimeContactAnalysisSegment>>)`](crate::output::ListRealtimeContactAnalysisSegmentsOutput::segments): <p>An analyzed transcript or category.</p>
    ///   - [`next_token(Option<String>)`](crate::output::ListRealtimeContactAnalysisSegmentsOutput::next_token): <p>If there are additional results, this is the token for the next set of results. If response includes <code>nextToken</code> there are two possible scenarios:</p>  <ul>   <li> <p>There are more segments so another call is required to get them.</p> </li>   <li> <p>There are no more segments at this time, but more may be available later (real-time analysis is in progress) so the client should call the operation again to get new segments.</p> </li>  </ul>  <p>If response does not include <code>nextToken</code>, the analysis is completed (successfully or failed) and there are no more segments to retrieve.</p>
                        /// - On failure, responds with [`SdkError<ListRealtimeContactAnalysisSegmentsError>`](crate::error::ListRealtimeContactAnalysisSegmentsError)
    pub fn list_realtime_contact_analysis_segments(&self) -> fluent_builders::ListRealtimeContactAnalysisSegments {
                            fluent_builders::ListRealtimeContactAnalysisSegments::new(self.handle.clone())
                        }
}
pub mod fluent_builders {
    
    //! Utilities to ergonomically construct a request to the service.
    //! 
    //! Fluent builders are created through the [`Client`](crate::client::Client) by calling
    //! one if its operation methods. After parameters are set using the builder methods,
    //! the `send` method can be called to initiate the request.
    /// Fluent builder constructing a request to `ListRealtimeContactAnalysisSegments`.
                        ///
    /// <p>Provides a list of analysis segments for a real-time analysis session.</p>
    #[derive(std::clone::Clone, std::fmt::Debug)]
    pub struct ListRealtimeContactAnalysisSegments {
                            handle: std::sync::Arc<super::Handle>,
                            inner: crate::input::list_realtime_contact_analysis_segments_input::Builder
                        }
    impl ListRealtimeContactAnalysisSegments  {
        /// Creates a new `ListRealtimeContactAnalysisSegments`.
                                pub(crate) fn new(handle: std::sync::Arc<super::Handle>) -> Self {
                                    Self { handle, inner: Default::default() }
                                }
        
                                /// Consume this builder, creating a customizable operation that can be modified before being
                                /// sent. The operation's inner [http::Request] can be modified as well.
                                pub async fn customize(self) -> std::result::Result<
                                    crate::operation::customize::CustomizableOperation<crate::operation::ListRealtimeContactAnalysisSegments, aws_http::retry::AwsResponseRetryClassifier,>,
                                    aws_smithy_http::result::SdkError<crate::error::ListRealtimeContactAnalysisSegmentsError>
                                >  {
                                    let handle = self.handle.clone();
                                    let operation = self.inner.build().map_err(aws_smithy_http::result::SdkError::construction_failure)?
                                        .make_operation(&handle.conf)
                                        .await
                                        .map_err(aws_smithy_http::result::SdkError::construction_failure)?;
                                    Ok(crate::operation::customize::CustomizableOperation { handle, operation })
                                }
        
                                /// Sends the request and returns the response.
                                ///
                                /// If an error occurs, an `SdkError` will be returned with additional details that
                                /// can be matched against.
                                ///
                                /// By default, any retryable failures will be retried twice. Retry behavior
                                /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
                                /// set when configuring the client.
                                pub async fn send(self) -> std::result::Result<crate::output::ListRealtimeContactAnalysisSegmentsOutput, aws_smithy_http::result::SdkError<crate::error::ListRealtimeContactAnalysisSegmentsError>>
                                 {
                                    let op = self.inner.build().map_err(aws_smithy_http::result::SdkError::construction_failure)?
                                        .make_operation(&self.handle.conf)
                                        .await
                                        .map_err(aws_smithy_http::result::SdkError::construction_failure)?;
                                    self.handle.client.call(op).await
                                }
        /// Create a paginator for this request
                                    ///
                                    /// Paginators are used by calling [`send().await`](crate::paginator::ListRealtimeContactAnalysisSegmentsPaginator::send) which returns a [`Stream`](tokio_stream::Stream).
                                    pub fn into_paginator(self) -> crate::paginator::ListRealtimeContactAnalysisSegmentsPaginator {
                                        crate::paginator::ListRealtimeContactAnalysisSegmentsPaginator::new(self.handle, self.inner)
                                    }
        /// <p>The identifier of the Amazon Connect instance.</p>
        pub fn instance_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.instance_id(input.into());
            self
        }
        /// <p>The identifier of the Amazon Connect instance.</p>
        pub fn set_instance_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_instance_id(input);
            self
        }
        /// <p>The identifier of the contact.</p>
        pub fn contact_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.contact_id(input.into());
            self
        }
        /// <p>The identifier of the contact.</p>
        pub fn set_contact_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_contact_id(input);
            self
        }
        /// <p>The maximimum number of results to return per page.</p>
        pub fn max_results(mut self, input: i32) -> Self {
            self.inner = self.inner.max_results(input);
            self
        }
        /// <p>The maximimum number of results to return per page.</p>
        pub fn set_max_results(mut self, input: std::option::Option<i32>) -> Self {
            self.inner = self.inner.set_max_results(input);
            self
        }
        /// <p>The token for the next set of results. Use the value returned in the previous response in the next request to retrieve the next set of results.</p>
        pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.next_token(input.into());
            self
        }
        /// <p>The token for the next set of results. Use the value returned in the previous response in the next request to retrieve the next set of results.</p>
        pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_next_token(input);
            self
        }
    }
    
    
}

impl Client {
    /// Creates a new client from an [SDK Config](aws_types::sdk_config::SdkConfig).
                    ///
                    /// # Panics
                    ///
                    /// - This method will panic if the `sdk_config` is missing an async sleep implementation. If you experience this panic, set
                    ///     the `sleep_impl` on the Config passed into this function to fix it.
                    /// - This method will panic if the `sdk_config` is missing an HTTP connector. If you experience this panic, set the
                    ///     `http_connector` on the Config passed into this function to fix it.
                    pub fn new(sdk_config: &aws_types::sdk_config::SdkConfig) -> Self {
                        Self::from_conf(sdk_config.into())
                    }
    
                    /// Creates a new client from the service [`Config`](crate::Config).
                    ///
                    /// # Panics
                    ///
                    /// - This method will panic if the `conf` is missing an async sleep implementation. If you experience this panic, set
                    ///     the `sleep_impl` on the Config passed into this function to fix it.
                    /// - This method will panic if the `conf` is missing an HTTP connector. If you experience this panic, set the
                    ///     `http_connector` on the Config passed into this function to fix it.
                    pub fn from_conf(conf: crate::Config) -> Self {
                        let retry_config = conf.retry_config().cloned().unwrap_or_else(aws_smithy_types::retry::RetryConfig::disabled);
                        let timeout_config = conf.timeout_config().cloned().unwrap_or_else(aws_smithy_types::timeout::TimeoutConfig::disabled);
                        let sleep_impl = conf.sleep_impl();
                        if (retry_config.has_retry() || timeout_config.has_timeouts()) && sleep_impl.is_none() {
                            panic!("An async sleep implementation is required for retries or timeouts to work. \
                                    Set the `sleep_impl` on the Config passed into this function to fix this panic.");
                        }
    
                        let connector = conf.http_connector().and_then(|c| {
                            let timeout_config = conf
                                .timeout_config()
                                .cloned()
                                .unwrap_or_else(aws_smithy_types::timeout::TimeoutConfig::disabled);
                            let connector_settings = aws_smithy_client::http_connector::ConnectorSettings::from_timeout_config(
                                &timeout_config,
                            );
                            c.connector(&connector_settings, conf.sleep_impl())
                        });
    
                        let builder = aws_smithy_client::Builder::new();
    
                        let builder = match connector {
                            // Use provided connector
                            Some(c) => builder.connector(c),
                            None =>{
                                #[cfg(any(feature = "rustls", feature = "native-tls"))]
                                {
                                    // Use default connector based on enabled features
                                    builder.dyn_https_connector(aws_smithy_client::http_connector::ConnectorSettings::from_timeout_config(&timeout_config))
                                }
                                #[cfg(not(any(feature = "rustls", feature = "native-tls")))]
                                {
                                    panic!("No HTTP connector was available. Enable the `rustls` or `native-tls` crate feature or set a connector to fix this.");
                                }
                            }
                        };
                        let mut builder = builder
                            .middleware(aws_smithy_client::erase::DynMiddleware::new(crate::middleware::DefaultMiddleware::new()))
                            .retry_config(retry_config.into())
                            .operation_timeout_config(timeout_config.into());
                        builder.set_sleep_impl(sleep_impl);
                        let client = builder.build();
    
                        Self { handle: std::sync::Arc::new(Handle { client, conf }) }
                    }
}

