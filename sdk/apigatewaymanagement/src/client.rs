// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[derive(Debug)]
            pub(crate) struct Handle {
                pub(crate) client: aws_smithy_client::Client<aws_smithy_client::erase::DynConnector, aws_smithy_client::erase::DynMiddleware<aws_smithy_client::erase::DynConnector>>,
                pub(crate) conf: crate::Config,
            }

            /// Client for AmazonApiGatewayManagementApi
                    ///
                    /// Client for invoking operations on AmazonApiGatewayManagementApi. Each operation on AmazonApiGatewayManagementApi is a method on this
                    /// this struct. `.send()` MUST be invoked on the generated operations to dispatch the request to the service.
///
                        /// # Examples
                        /// **Constructing a client and invoking an operation**
                        /// ```rust,no_run
                        /// # async fn docs() {
                        ///     // create a shared configuration. This can be used & shared between multiple service clients.
                        ///     let shared_config = aws_config::load_from_env().await;
                        ///     let client = aws_sdk_apigatewaymanagement::Client::new(&shared_config);
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
                        /// let config = aws_sdk_apigatewaymanagement::config::Builder::from(&shared_config)
                        ///   .retry_config(RetryConfig::disabled())
                        ///   .build();
                        /// let client = aws_sdk_apigatewaymanagement::Client::from_conf(config);
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
    /// Constructs a fluent builder for the [`DeleteConnection`](crate::client::fluent_builders::DeleteConnection) operation.
                        ///
                        /// - The fluent builder is configurable:
                        ///   - [`connection_id(impl Into<String>)`](crate::client::fluent_builders::DeleteConnection::connection_id) / [`set_connection_id(Option<String>)`](crate::client::fluent_builders::DeleteConnection::set_connection_id): (undocumented)
                        /// - On success, responds with [`DeleteConnectionOutput`](crate::output::DeleteConnectionOutput)
                        
                        /// - On failure, responds with [`SdkError<DeleteConnectionError>`](crate::error::DeleteConnectionError)
    pub fn delete_connection(&self) -> fluent_builders::DeleteConnection {
                            fluent_builders::DeleteConnection::new(self.handle.clone())
                        }
    /// Constructs a fluent builder for the [`GetConnection`](crate::client::fluent_builders::GetConnection) operation.
                        ///
                        /// - The fluent builder is configurable:
                        ///   - [`connection_id(impl Into<String>)`](crate::client::fluent_builders::GetConnection::connection_id) / [`set_connection_id(Option<String>)`](crate::client::fluent_builders::GetConnection::set_connection_id): (undocumented)
                        /// - On success, responds with [`GetConnectionOutput`](crate::output::GetConnectionOutput) with field(s):
                        ///   - [`connected_at(Option<DateTime>)`](crate::output::GetConnectionOutput::connected_at): <p>The time in ISO 8601 format for when the connection was established.</p>
    ///   - [`identity(Option<Identity>)`](crate::output::GetConnectionOutput::identity): (undocumented)
    ///   - [`last_active_at(Option<DateTime>)`](crate::output::GetConnectionOutput::last_active_at): <p>The time in ISO 8601 format for when the connection was last active.</p>
                        /// - On failure, responds with [`SdkError<GetConnectionError>`](crate::error::GetConnectionError)
    pub fn get_connection(&self) -> fluent_builders::GetConnection {
                            fluent_builders::GetConnection::new(self.handle.clone())
                        }
    /// Constructs a fluent builder for the [`PostToConnection`](crate::client::fluent_builders::PostToConnection) operation.
                        ///
                        /// - The fluent builder is configurable:
                        ///   - [`data(Blob)`](crate::client::fluent_builders::PostToConnection::data) / [`set_data(Option<Blob>)`](crate::client::fluent_builders::PostToConnection::set_data): <p>The data to be sent to the client specified by its connection id.</p>
    ///   - [`connection_id(impl Into<String>)`](crate::client::fluent_builders::PostToConnection::connection_id) / [`set_connection_id(Option<String>)`](crate::client::fluent_builders::PostToConnection::set_connection_id): <p>The identifier of the connection that a specific client is using.</p>
                        /// - On success, responds with [`PostToConnectionOutput`](crate::output::PostToConnectionOutput)
                        
                        /// - On failure, responds with [`SdkError<PostToConnectionError>`](crate::error::PostToConnectionError)
    pub fn post_to_connection(&self) -> fluent_builders::PostToConnection {
                            fluent_builders::PostToConnection::new(self.handle.clone())
                        }
}
pub mod fluent_builders {
    
    //! Utilities to ergonomically construct a request to the service.
    //! 
    //! Fluent builders are created through the [`Client`](crate::client::Client) by calling
    //! one if its operation methods. After parameters are set using the builder methods,
    //! the `send` method can be called to initiate the request.
    /// Fluent builder constructing a request to `DeleteConnection`.
                        ///
    /// <p>Delete the connection with the provided id.</p>
    #[derive(std::clone::Clone, std::fmt::Debug)]
    pub struct DeleteConnection {
                            handle: std::sync::Arc<super::Handle>,
                            inner: crate::input::delete_connection_input::Builder
                        }
    impl DeleteConnection  {
        /// Creates a new `DeleteConnection`.
                                pub(crate) fn new(handle: std::sync::Arc<super::Handle>) -> Self {
                                    Self { handle, inner: Default::default() }
                                }
        
                                /// Consume this builder, creating a customizable operation that can be modified before being
                                /// sent. The operation's inner [http::Request] can be modified as well.
                                pub async fn customize(self) -> std::result::Result<
                                    crate::operation::customize::CustomizableOperation<crate::operation::DeleteConnection, aws_http::retry::AwsResponseRetryClassifier,>,
                                    aws_smithy_http::result::SdkError<crate::error::DeleteConnectionError>
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
                                pub async fn send(self) -> std::result::Result<crate::output::DeleteConnectionOutput, aws_smithy_http::result::SdkError<crate::error::DeleteConnectionError>>
                                 {
                                    let op = self.inner.build().map_err(aws_smithy_http::result::SdkError::construction_failure)?
                                        .make_operation(&self.handle.conf)
                                        .await
                                        .map_err(aws_smithy_http::result::SdkError::construction_failure)?;
                                    self.handle.client.call(op).await
                                }
        #[allow(missing_docs)] // documentation missing in model
        pub fn connection_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.connection_id(input.into());
            self
        }
        #[allow(missing_docs)] // documentation missing in model
        pub fn set_connection_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_connection_id(input);
            self
        }
    }
    /// Fluent builder constructing a request to `GetConnection`.
                        ///
    /// <p>Get information about the connection with the provided id.</p>
    #[derive(std::clone::Clone, std::fmt::Debug)]
    pub struct GetConnection {
                            handle: std::sync::Arc<super::Handle>,
                            inner: crate::input::get_connection_input::Builder
                        }
    impl GetConnection  {
        /// Creates a new `GetConnection`.
                                pub(crate) fn new(handle: std::sync::Arc<super::Handle>) -> Self {
                                    Self { handle, inner: Default::default() }
                                }
        
                                /// Consume this builder, creating a customizable operation that can be modified before being
                                /// sent. The operation's inner [http::Request] can be modified as well.
                                pub async fn customize(self) -> std::result::Result<
                                    crate::operation::customize::CustomizableOperation<crate::operation::GetConnection, aws_http::retry::AwsResponseRetryClassifier,>,
                                    aws_smithy_http::result::SdkError<crate::error::GetConnectionError>
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
                                pub async fn send(self) -> std::result::Result<crate::output::GetConnectionOutput, aws_smithy_http::result::SdkError<crate::error::GetConnectionError>>
                                 {
                                    let op = self.inner.build().map_err(aws_smithy_http::result::SdkError::construction_failure)?
                                        .make_operation(&self.handle.conf)
                                        .await
                                        .map_err(aws_smithy_http::result::SdkError::construction_failure)?;
                                    self.handle.client.call(op).await
                                }
        #[allow(missing_docs)] // documentation missing in model
        pub fn connection_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.connection_id(input.into());
            self
        }
        #[allow(missing_docs)] // documentation missing in model
        pub fn set_connection_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_connection_id(input);
            self
        }
    }
    /// Fluent builder constructing a request to `PostToConnection`.
                        ///
    /// <p>Sends the provided data to the specified connection.</p>
    #[derive(std::clone::Clone, std::fmt::Debug)]
    pub struct PostToConnection {
                            handle: std::sync::Arc<super::Handle>,
                            inner: crate::input::post_to_connection_input::Builder
                        }
    impl PostToConnection  {
        /// Creates a new `PostToConnection`.
                                pub(crate) fn new(handle: std::sync::Arc<super::Handle>) -> Self {
                                    Self { handle, inner: Default::default() }
                                }
        
                                /// Consume this builder, creating a customizable operation that can be modified before being
                                /// sent. The operation's inner [http::Request] can be modified as well.
                                pub async fn customize(self) -> std::result::Result<
                                    crate::operation::customize::CustomizableOperation<crate::operation::PostToConnection, aws_http::retry::AwsResponseRetryClassifier,>,
                                    aws_smithy_http::result::SdkError<crate::error::PostToConnectionError>
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
                                pub async fn send(self) -> std::result::Result<crate::output::PostToConnectionOutput, aws_smithy_http::result::SdkError<crate::error::PostToConnectionError>>
                                 {
                                    let op = self.inner.build().map_err(aws_smithy_http::result::SdkError::construction_failure)?
                                        .make_operation(&self.handle.conf)
                                        .await
                                        .map_err(aws_smithy_http::result::SdkError::construction_failure)?;
                                    self.handle.client.call(op).await
                                }
        /// <p>The data to be sent to the client specified by its connection id.</p>
        pub fn data(mut self, input: aws_smithy_types::Blob) -> Self {
            self.inner = self.inner.data(input);
            self
        }
        /// <p>The data to be sent to the client specified by its connection id.</p>
        pub fn set_data(mut self, input: std::option::Option<aws_smithy_types::Blob>) -> Self {
            self.inner = self.inner.set_data(input);
            self
        }
        /// <p>The identifier of the connection that a specific client is using.</p>
        pub fn connection_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.connection_id(input.into());
            self
        }
        /// <p>The identifier of the connection that a specific client is using.</p>
        pub fn set_connection_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_connection_id(input);
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

