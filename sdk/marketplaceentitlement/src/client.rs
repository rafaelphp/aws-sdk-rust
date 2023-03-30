// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[derive(Debug)]
            pub(crate) struct Handle {
                pub(crate) client: aws_smithy_client::Client<aws_smithy_client::erase::DynConnector, aws_smithy_client::erase::DynMiddleware<aws_smithy_client::erase::DynConnector>>,
                pub(crate) conf: crate::Config,
            }

            /// Client for AWS Marketplace Entitlement Service
                    ///
                    /// Client for invoking operations on AWS Marketplace Entitlement Service. Each operation on AWS Marketplace Entitlement Service is a method on this
                    /// this struct. `.send()` MUST be invoked on the generated operations to dispatch the request to the service.
///
                        /// # Examples
                        /// **Constructing a client and invoking an operation**
                        /// ```rust,no_run
                        /// # async fn docs() {
                        ///     // create a shared configuration. This can be used & shared between multiple service clients.
                        ///     let shared_config = aws_config::load_from_env().await;
                        ///     let client = aws_sdk_marketplaceentitlement::Client::new(&shared_config);
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
                        /// let config = aws_sdk_marketplaceentitlement::config::Builder::from(&shared_config)
                        ///   .retry_config(RetryConfig::disabled())
                        ///   .build();
                        /// let client = aws_sdk_marketplaceentitlement::Client::from_conf(config);
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
    /// Constructs a fluent builder for the [`GetEntitlements`](crate::client::fluent_builders::GetEntitlements) operation.
                        ///
                        /// - The fluent builder is configurable:
                        ///   - [`product_code(impl Into<String>)`](crate::client::fluent_builders::GetEntitlements::product_code) / [`set_product_code(Option<String>)`](crate::client::fluent_builders::GetEntitlements::set_product_code): <p>Product code is used to uniquely identify a product in AWS Marketplace. The product code will be provided by AWS Marketplace when the product listing is created.</p>
    ///   - [`filter(HashMap<GetEntitlementFilterName, Vec<String>>)`](crate::client::fluent_builders::GetEntitlements::filter) / [`set_filter(Option<HashMap<GetEntitlementFilterName, Vec<String>>>)`](crate::client::fluent_builders::GetEntitlements::set_filter): <p>Filter is used to return entitlements for a specific customer or for a specific dimension. Filters are described as keys mapped to a lists of values. Filtered requests are <i>unioned</i> for each value in the value list, and then <i>intersected</i> for each filter key.</p>
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::GetEntitlements::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::GetEntitlements::set_next_token): <p>For paginated calls to GetEntitlements, pass the NextToken from the previous GetEntitlementsResult.</p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::GetEntitlements::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::GetEntitlements::set_max_results): <p>The maximum number of items to retrieve from the GetEntitlements operation. For pagination, use the NextToken field in subsequent calls to GetEntitlements.</p>
                        /// - On success, responds with [`GetEntitlementsOutput`](crate::output::GetEntitlementsOutput) with field(s):
                        ///   - [`entitlements(Option<Vec<Entitlement>>)`](crate::output::GetEntitlementsOutput::entitlements): <p>The set of entitlements found through the GetEntitlements operation. If the result contains an empty set of entitlements, NextToken might still be present and should be used.</p>
    ///   - [`next_token(Option<String>)`](crate::output::GetEntitlementsOutput::next_token): <p>For paginated results, use NextToken in subsequent calls to GetEntitlements. If the result contains an empty set of entitlements, NextToken might still be present and should be used.</p>
                        /// - On failure, responds with [`SdkError<GetEntitlementsError>`](crate::error::GetEntitlementsError)
    pub fn get_entitlements(&self) -> fluent_builders::GetEntitlements {
                            fluent_builders::GetEntitlements::new(self.handle.clone())
                        }
}
pub mod fluent_builders {
    
    //! Utilities to ergonomically construct a request to the service.
    //! 
    //! Fluent builders are created through the [`Client`](crate::client::Client) by calling
    //! one if its operation methods. After parameters are set using the builder methods,
    //! the `send` method can be called to initiate the request.
    /// Fluent builder constructing a request to `GetEntitlements`.
                        ///
    /// <p>GetEntitlements retrieves entitlement values for a given product. The results can be filtered based on customer identifier or product dimensions.</p>
    #[derive(std::clone::Clone, std::fmt::Debug)]
    pub struct GetEntitlements {
                            handle: std::sync::Arc<super::Handle>,
                            inner: crate::input::get_entitlements_input::Builder
                        }
    impl GetEntitlements  {
        /// Creates a new `GetEntitlements`.
                                pub(crate) fn new(handle: std::sync::Arc<super::Handle>) -> Self {
                                    Self { handle, inner: Default::default() }
                                }
        
                                /// Consume this builder, creating a customizable operation that can be modified before being
                                /// sent. The operation's inner [http::Request] can be modified as well.
                                pub async fn customize(self) -> std::result::Result<
                                    crate::operation::customize::CustomizableOperation<crate::operation::GetEntitlements, aws_http::retry::AwsResponseRetryClassifier,>,
                                    aws_smithy_http::result::SdkError<crate::error::GetEntitlementsError>
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
                                pub async fn send(self) -> std::result::Result<crate::output::GetEntitlementsOutput, aws_smithy_http::result::SdkError<crate::error::GetEntitlementsError>>
                                 {
                                    let op = self.inner.build().map_err(aws_smithy_http::result::SdkError::construction_failure)?
                                        .make_operation(&self.handle.conf)
                                        .await
                                        .map_err(aws_smithy_http::result::SdkError::construction_failure)?;
                                    self.handle.client.call(op).await
                                }
        /// <p>Product code is used to uniquely identify a product in AWS Marketplace. The product code will be provided by AWS Marketplace when the product listing is created.</p>
        pub fn product_code(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.product_code(input.into());
            self
        }
        /// <p>Product code is used to uniquely identify a product in AWS Marketplace. The product code will be provided by AWS Marketplace when the product listing is created.</p>
        pub fn set_product_code(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_product_code(input);
            self
        }
        /// Adds a key-value pair to `Filter`.
        ///
        /// To override the contents of this collection use [`set_filter`](Self::set_filter).
        ///
        /// <p>Filter is used to return entitlements for a specific customer or for a specific dimension. Filters are described as keys mapped to a lists of values. Filtered requests are <i>unioned</i> for each value in the value list, and then <i>intersected</i> for each filter key.</p>
        pub fn filter(mut self, k: crate::model::GetEntitlementFilterName, v: std::vec::Vec<std::string::String>) -> Self {
            self.inner = self.inner.filter(k, v);
            self
        }
        /// <p>Filter is used to return entitlements for a specific customer or for a specific dimension. Filters are described as keys mapped to a lists of values. Filtered requests are <i>unioned</i> for each value in the value list, and then <i>intersected</i> for each filter key.</p>
        pub fn set_filter(mut self, input: std::option::Option<std::collections::HashMap<crate::model::GetEntitlementFilterName, std::vec::Vec<std::string::String>>>) -> Self {
            self.inner = self.inner.set_filter(input);
            self
        }
        /// <p>For paginated calls to GetEntitlements, pass the NextToken from the previous GetEntitlementsResult.</p>
        pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.next_token(input.into());
            self
        }
        /// <p>For paginated calls to GetEntitlements, pass the NextToken from the previous GetEntitlementsResult.</p>
        pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_next_token(input);
            self
        }
        /// <p>The maximum number of items to retrieve from the GetEntitlements operation. For pagination, use the NextToken field in subsequent calls to GetEntitlements.</p>
        pub fn max_results(mut self, input: i32) -> Self {
            self.inner = self.inner.max_results(input);
            self
        }
        /// <p>The maximum number of items to retrieve from the GetEntitlements operation. For pagination, use the NextToken field in subsequent calls to GetEntitlements.</p>
        pub fn set_max_results(mut self, input: std::option::Option<i32>) -> Self {
            self.inner = self.inner.set_max_results(input);
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

