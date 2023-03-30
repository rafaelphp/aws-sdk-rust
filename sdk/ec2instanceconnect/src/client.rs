// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[derive(Debug)]
            pub(crate) struct Handle {
                pub(crate) client: aws_smithy_client::Client<aws_smithy_client::erase::DynConnector, aws_smithy_client::erase::DynMiddleware<aws_smithy_client::erase::DynConnector>>,
                pub(crate) conf: crate::Config,
            }

            /// Client for AWS EC2 Instance Connect
                    ///
                    /// Client for invoking operations on AWS EC2 Instance Connect. Each operation on AWS EC2 Instance Connect is a method on this
                    /// this struct. `.send()` MUST be invoked on the generated operations to dispatch the request to the service.
///
                        /// # Examples
                        /// **Constructing a client and invoking an operation**
                        /// ```rust,no_run
                        /// # async fn docs() {
                        ///     // create a shared configuration. This can be used & shared between multiple service clients.
                        ///     let shared_config = aws_config::load_from_env().await;
                        ///     let client = aws_sdk_ec2instanceconnect::Client::new(&shared_config);
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
                        /// let config = aws_sdk_ec2instanceconnect::config::Builder::from(&shared_config)
                        ///   .retry_config(RetryConfig::disabled())
                        ///   .build();
                        /// let client = aws_sdk_ec2instanceconnect::Client::from_conf(config);
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
    /// Constructs a fluent builder for the [`SendSerialConsoleSSHPublicKey`](crate::client::fluent_builders::SendSerialConsoleSSHPublicKey) operation.
                        ///
                        /// - The fluent builder is configurable:
                        ///   - [`instance_id(impl Into<String>)`](crate::client::fluent_builders::SendSerialConsoleSSHPublicKey::instance_id) / [`set_instance_id(Option<String>)`](crate::client::fluent_builders::SendSerialConsoleSSHPublicKey::set_instance_id): <p>The ID of the EC2 instance.</p>
    ///   - [`serial_port(i32)`](crate::client::fluent_builders::SendSerialConsoleSSHPublicKey::serial_port) / [`set_serial_port(i32)`](crate::client::fluent_builders::SendSerialConsoleSSHPublicKey::set_serial_port): <p>The serial port of the EC2 instance. Currently only port 0 is supported.</p>  <p>Default: 0</p>
    ///   - [`ssh_public_key(impl Into<String>)`](crate::client::fluent_builders::SendSerialConsoleSSHPublicKey::ssh_public_key) / [`set_ssh_public_key(Option<String>)`](crate::client::fluent_builders::SendSerialConsoleSSHPublicKey::set_ssh_public_key): <p>The public key material. To use the public key, you must have the matching private key. For information about the supported key formats and lengths, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ec2-key-pairs.html#how-to-generate-your-own-key-and-import-it-to-aws">Requirements for key pairs</a> in the <i>Amazon EC2 User Guide</i>.</p>
                        /// - On success, responds with [`SendSerialConsoleSshPublicKeyOutput`](crate::output::SendSerialConsoleSshPublicKeyOutput) with field(s):
                        ///   - [`request_id(Option<String>)`](crate::output::SendSerialConsoleSshPublicKeyOutput::request_id): <p>The ID of the request. Please provide this ID when contacting AWS Support for assistance.</p>
    ///   - [`success(bool)`](crate::output::SendSerialConsoleSshPublicKeyOutput::success): <p>Is true if the request succeeds and an error otherwise.</p>
                        /// - On failure, responds with [`SdkError<SendSerialConsoleSSHPublicKeyError>`](crate::error::SendSerialConsoleSSHPublicKeyError)
    pub fn send_serial_console_ssh_public_key(&self) -> fluent_builders::SendSerialConsoleSSHPublicKey {
                            fluent_builders::SendSerialConsoleSSHPublicKey::new(self.handle.clone())
                        }
    /// Constructs a fluent builder for the [`SendSSHPublicKey`](crate::client::fluent_builders::SendSSHPublicKey) operation.
                        ///
                        /// - The fluent builder is configurable:
                        ///   - [`instance_id(impl Into<String>)`](crate::client::fluent_builders::SendSSHPublicKey::instance_id) / [`set_instance_id(Option<String>)`](crate::client::fluent_builders::SendSSHPublicKey::set_instance_id): <p>The ID of the EC2 instance.</p>
    ///   - [`instance_os_user(impl Into<String>)`](crate::client::fluent_builders::SendSSHPublicKey::instance_os_user) / [`set_instance_os_user(Option<String>)`](crate::client::fluent_builders::SendSSHPublicKey::set_instance_os_user): <p>The OS user on the EC2 instance for whom the key can be used to authenticate.</p>
    ///   - [`ssh_public_key(impl Into<String>)`](crate::client::fluent_builders::SendSSHPublicKey::ssh_public_key) / [`set_ssh_public_key(Option<String>)`](crate::client::fluent_builders::SendSSHPublicKey::set_ssh_public_key): <p>The public key material. To use the public key, you must have the matching private key.</p>
    ///   - [`availability_zone(impl Into<String>)`](crate::client::fluent_builders::SendSSHPublicKey::availability_zone) / [`set_availability_zone(Option<String>)`](crate::client::fluent_builders::SendSSHPublicKey::set_availability_zone): <p>The Availability Zone in which the EC2 instance was launched.</p>
                        /// - On success, responds with [`SendSshPublicKeyOutput`](crate::output::SendSshPublicKeyOutput) with field(s):
                        ///   - [`request_id(Option<String>)`](crate::output::SendSshPublicKeyOutput::request_id): <p>The ID of the request. Please provide this ID when contacting AWS Support for assistance.</p>
    ///   - [`success(bool)`](crate::output::SendSshPublicKeyOutput::success): <p>Is true if the request succeeds and an error otherwise.</p>
                        /// - On failure, responds with [`SdkError<SendSSHPublicKeyError>`](crate::error::SendSSHPublicKeyError)
    pub fn send_ssh_public_key(&self) -> fluent_builders::SendSSHPublicKey {
                            fluent_builders::SendSSHPublicKey::new(self.handle.clone())
                        }
}
pub mod fluent_builders {
    
    //! Utilities to ergonomically construct a request to the service.
    //! 
    //! Fluent builders are created through the [`Client`](crate::client::Client) by calling
    //! one if its operation methods. After parameters are set using the builder methods,
    //! the `send` method can be called to initiate the request.
    /// Fluent builder constructing a request to `SendSerialConsoleSSHPublicKey`.
                        ///
    /// <p>Pushes an SSH public key to the specified EC2 instance. The key remains for 60 seconds, which gives you 60 seconds to establish a serial console connection to the instance using SSH. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ec2-serial-console.html">EC2 Serial Console</a> in the <i>Amazon EC2 User Guide</i>.</p>
    #[derive(std::clone::Clone, std::fmt::Debug)]
    pub struct SendSerialConsoleSSHPublicKey {
                            handle: std::sync::Arc<super::Handle>,
                            inner: crate::input::send_serial_console_ssh_public_key_input::Builder
                        }
    impl SendSerialConsoleSSHPublicKey  {
        /// Creates a new `SendSerialConsoleSSHPublicKey`.
                                pub(crate) fn new(handle: std::sync::Arc<super::Handle>) -> Self {
                                    Self { handle, inner: Default::default() }
                                }
        
                                /// Consume this builder, creating a customizable operation that can be modified before being
                                /// sent. The operation's inner [http::Request] can be modified as well.
                                pub async fn customize(self) -> std::result::Result<
                                    crate::operation::customize::CustomizableOperation<crate::operation::SendSerialConsoleSSHPublicKey, aws_http::retry::AwsResponseRetryClassifier,>,
                                    aws_smithy_http::result::SdkError<crate::error::SendSerialConsoleSSHPublicKeyError>
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
                                pub async fn send(self) -> std::result::Result<crate::output::SendSerialConsoleSshPublicKeyOutput, aws_smithy_http::result::SdkError<crate::error::SendSerialConsoleSSHPublicKeyError>>
                                 {
                                    let op = self.inner.build().map_err(aws_smithy_http::result::SdkError::construction_failure)?
                                        .make_operation(&self.handle.conf)
                                        .await
                                        .map_err(aws_smithy_http::result::SdkError::construction_failure)?;
                                    self.handle.client.call(op).await
                                }
        /// <p>The ID of the EC2 instance.</p>
        pub fn instance_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.instance_id(input.into());
            self
        }
        /// <p>The ID of the EC2 instance.</p>
        pub fn set_instance_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_instance_id(input);
            self
        }
        /// <p>The serial port of the EC2 instance. Currently only port 0 is supported.</p> 
        /// <p>Default: 0</p>
        pub fn serial_port(mut self, input: i32) -> Self {
            self.inner = self.inner.serial_port(input);
            self
        }
        /// <p>The serial port of the EC2 instance. Currently only port 0 is supported.</p> 
        /// <p>Default: 0</p>
        pub fn set_serial_port(mut self, input: std::option::Option<i32>) -> Self {
            self.inner = self.inner.set_serial_port(input);
            self
        }
        /// <p>The public key material. To use the public key, you must have the matching private key. For information about the supported key formats and lengths, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ec2-key-pairs.html#how-to-generate-your-own-key-and-import-it-to-aws">Requirements for key pairs</a> in the <i>Amazon EC2 User Guide</i>.</p>
        pub fn ssh_public_key(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.ssh_public_key(input.into());
            self
        }
        /// <p>The public key material. To use the public key, you must have the matching private key. For information about the supported key formats and lengths, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ec2-key-pairs.html#how-to-generate-your-own-key-and-import-it-to-aws">Requirements for key pairs</a> in the <i>Amazon EC2 User Guide</i>.</p>
        pub fn set_ssh_public_key(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_ssh_public_key(input);
            self
        }
    }
    /// Fluent builder constructing a request to `SendSSHPublicKey`.
                        ///
    /// <p>Pushes an SSH public key to the specified EC2 instance for use by the specified user. The key remains for 60 seconds. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/Connect-using-EC2-Instance-Connect.html">Connect to your Linux instance using EC2 Instance Connect</a> in the <i>Amazon EC2 User Guide</i>.</p>
    #[derive(std::clone::Clone, std::fmt::Debug)]
    pub struct SendSSHPublicKey {
                            handle: std::sync::Arc<super::Handle>,
                            inner: crate::input::send_ssh_public_key_input::Builder
                        }
    impl SendSSHPublicKey  {
        /// Creates a new `SendSSHPublicKey`.
                                pub(crate) fn new(handle: std::sync::Arc<super::Handle>) -> Self {
                                    Self { handle, inner: Default::default() }
                                }
        
                                /// Consume this builder, creating a customizable operation that can be modified before being
                                /// sent. The operation's inner [http::Request] can be modified as well.
                                pub async fn customize(self) -> std::result::Result<
                                    crate::operation::customize::CustomizableOperation<crate::operation::SendSSHPublicKey, aws_http::retry::AwsResponseRetryClassifier,>,
                                    aws_smithy_http::result::SdkError<crate::error::SendSSHPublicKeyError>
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
                                pub async fn send(self) -> std::result::Result<crate::output::SendSshPublicKeyOutput, aws_smithy_http::result::SdkError<crate::error::SendSSHPublicKeyError>>
                                 {
                                    let op = self.inner.build().map_err(aws_smithy_http::result::SdkError::construction_failure)?
                                        .make_operation(&self.handle.conf)
                                        .await
                                        .map_err(aws_smithy_http::result::SdkError::construction_failure)?;
                                    self.handle.client.call(op).await
                                }
        /// <p>The ID of the EC2 instance.</p>
        pub fn instance_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.instance_id(input.into());
            self
        }
        /// <p>The ID of the EC2 instance.</p>
        pub fn set_instance_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_instance_id(input);
            self
        }
        /// <p>The OS user on the EC2 instance for whom the key can be used to authenticate.</p>
        pub fn instance_os_user(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.instance_os_user(input.into());
            self
        }
        /// <p>The OS user on the EC2 instance for whom the key can be used to authenticate.</p>
        pub fn set_instance_os_user(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_instance_os_user(input);
            self
        }
        /// <p>The public key material. To use the public key, you must have the matching private key.</p>
        pub fn ssh_public_key(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.ssh_public_key(input.into());
            self
        }
        /// <p>The public key material. To use the public key, you must have the matching private key.</p>
        pub fn set_ssh_public_key(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_ssh_public_key(input);
            self
        }
        /// <p>The Availability Zone in which the EC2 instance was launched.</p>
        pub fn availability_zone(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.availability_zone(input.into());
            self
        }
        /// <p>The Availability Zone in which the EC2 instance was launched.</p>
        pub fn set_availability_zone(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_availability_zone(input);
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

