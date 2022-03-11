// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Service config.
///
///
/// Service configuration allows for customization of endpoints, region, credentials providers,
/// and retry configuration. Generally, it is constructed automatically for you from a shared
/// configuration loaded by the `aws-config` crate. For example:
///
/// ```ignore
/// // Load a shared config from the environment
/// let shared_config = aws_config::from_env().load().await;
/// // The client constructor automatically converts the shared config into the service config
/// let client = Client::new(&shared_config);
/// ```
///
/// The service config can also be constructed manually using its builder.
///
pub struct Config {
    pub(crate) make_token: crate::idempotency_token::IdempotencyTokenProvider,
    app_name: Option<aws_types::app_name::AppName>,
    pub(crate) timeout_config: Option<aws_smithy_types::timeout::Config>,
    pub(crate) sleep_impl: Option<std::sync::Arc<dyn aws_smithy_async::rt::sleep::AsyncSleep>>,
    pub(crate) retry_config: Option<aws_smithy_types::retry::RetryConfig>,
    pub(crate) endpoint_resolver: ::std::sync::Arc<dyn aws_endpoint::ResolveAwsEndpoint>,
    pub(crate) region: Option<aws_types::region::Region>,
    pub(crate) credentials_provider: aws_types::credentials::SharedCredentialsProvider,
}
impl std::fmt::Debug for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut config = f.debug_struct("Config");
        config.finish()
    }
}
impl Config {
    /// Constructs a config builder.
    pub fn builder() -> Builder {
        Builder::default()
    }
    /// Returns the name of the app that is using the client, if it was provided.
    ///
    /// This _optional_ name is used to identify the application in the user agent that
    /// gets sent along with requests.
    pub fn app_name(&self) -> Option<&aws_types::app_name::AppName> {
        self.app_name.as_ref()
    }
    /// Creates a new [service config](crate::Config) from a [shared `config`](aws_types::sdk_config::SdkConfig).
    pub fn new(config: &aws_types::sdk_config::SdkConfig) -> Self {
        Builder::from(config).build()
    }
    /// The signature version 4 service signing name to use in the credential scope when signing requests.
    ///
    /// The signing service may be overridden by the `Endpoint`, or by specifying a custom
    /// [`SigningService`](aws_types::SigningService) during operation construction
    pub fn signing_service(&self) -> &'static str {
        "comprehendmedical"
    }
}
/// Builder for creating a `Config`.
#[derive(Default)]
pub struct Builder {
    make_token: Option<crate::idempotency_token::IdempotencyTokenProvider>,
    app_name: Option<aws_types::app_name::AppName>,
    timeout_config: Option<aws_smithy_types::timeout::Config>,
    sleep_impl: Option<std::sync::Arc<dyn aws_smithy_async::rt::sleep::AsyncSleep>>,
    retry_config: Option<aws_smithy_types::retry::RetryConfig>,
    endpoint_resolver: Option<::std::sync::Arc<dyn aws_endpoint::ResolveAwsEndpoint>>,
    region: Option<aws_types::region::Region>,
    credentials_provider: Option<aws_types::credentials::SharedCredentialsProvider>,
}
impl Builder {
    /// Constructs a config builder.
    pub fn new() -> Self {
        Self::default()
    }
    /// Sets the idempotency token provider to use for service calls that require tokens.
    pub fn make_token(
        mut self,
        make_token: impl Into<crate::idempotency_token::IdempotencyTokenProvider>,
    ) -> Self {
        self.make_token = Some(make_token.into());
        self
    }
    /// Sets the name of the app that is using the client.
    ///
    /// This _optional_ name is used to identify the application in the user agent that
    /// gets sent along with requests.
    pub fn app_name(mut self, app_name: aws_types::app_name::AppName) -> Self {
        self.set_app_name(Some(app_name));
        self
    }

    /// Sets the name of the app that is using the client.
    ///
    /// This _optional_ name is used to identify the application in the user agent that
    /// gets sent along with requests.
    pub fn set_app_name(&mut self, app_name: Option<aws_types::app_name::AppName>) -> &mut Self {
        self.app_name = app_name;
        self
    }
    /// Set the timeout_config for the builder
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use std::time::Duration;
    /// use aws_sdk_comprehendmedical::config::Config;
    /// use aws_smithy_types::{timeout, tristate::TriState};
    ///
    /// let api_timeouts = timeout::Api::new()
    ///     .with_call_attempt_timeout(TriState::Set(Duration::from_secs(1)));
    /// let timeout_config = timeout::Config::new()
    ///     .with_api_timeouts(api_timeouts);
    /// let config = Config::builder().timeout_config(timeout_config).build();
    /// ```
    pub fn timeout_config(mut self, timeout_config: aws_smithy_types::timeout::Config) -> Self {
        self.set_timeout_config(Some(timeout_config));
        self
    }

    /// Set the timeout_config for the builder
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use std::time::Duration;
    /// use aws_sdk_comprehendmedical::config::{Builder, Config};
    /// use aws_smithy_types::{timeout, tristate::TriState};
    ///
    /// fn set_request_timeout(builder: &mut Builder) {
    ///     let api_timeouts = timeout::Api::new()
    ///         .with_call_attempt_timeout(TriState::Set(Duration::from_secs(1)));
    ///     let timeout_config = timeout::Config::new()
    ///         .with_api_timeouts(api_timeouts);
    ///     builder.set_timeout_config(Some(timeout_config));
    /// }
    ///
    /// let mut builder = Config::builder();
    /// set_request_timeout(&mut builder);
    /// let config = builder.build();
    /// ```
    pub fn set_timeout_config(
        &mut self,
        timeout_config: Option<aws_smithy_types::timeout::Config>,
    ) -> &mut Self {
        self.timeout_config = timeout_config;
        self
    }
    /// Set the sleep_impl for the builder
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use aws_sdk_comprehendmedical::config::Config;
    /// use aws_smithy_async::rt::sleep::AsyncSleep;
    /// use aws_smithy_async::rt::sleep::Sleep;
    ///
    /// #[derive(Debug)]
    /// pub struct ForeverSleep;
    ///
    /// impl AsyncSleep for ForeverSleep {
    ///     fn sleep(&self, duration: std::time::Duration) -> Sleep {
    ///         Sleep::new(std::future::pending())
    ///     }
    /// }
    ///
    /// let sleep_impl = std::sync::Arc::new(ForeverSleep);
    /// let config = Config::builder().sleep_impl(sleep_impl).build();
    /// ```
    pub fn sleep_impl(
        mut self,
        sleep_impl: std::sync::Arc<dyn aws_smithy_async::rt::sleep::AsyncSleep>,
    ) -> Self {
        self.set_sleep_impl(Some(sleep_impl));
        self
    }

    /// Set the sleep_impl for the builder
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use aws_sdk_comprehendmedical::config::{Builder, Config};
    /// use aws_smithy_async::rt::sleep::AsyncSleep;
    /// use aws_smithy_async::rt::sleep::Sleep;
    ///
    /// #[derive(Debug)]
    /// pub struct ForeverSleep;
    ///
    /// impl AsyncSleep for ForeverSleep {
    ///     fn sleep(&self, duration: std::time::Duration) -> Sleep {
    ///         Sleep::new(std::future::pending())
    ///     }
    /// }
    ///
    /// fn set_never_ending_sleep_impl(builder: &mut Builder) {
    ///     let sleep_impl = std::sync::Arc::new(ForeverSleep);
    ///     builder.set_sleep_impl(Some(sleep_impl));
    /// }
    ///
    /// let mut builder = Config::builder();
    /// set_never_ending_sleep_impl(&mut builder);
    /// let config = builder.build();
    /// ```
    pub fn set_sleep_impl(
        &mut self,
        sleep_impl: Option<std::sync::Arc<dyn aws_smithy_async::rt::sleep::AsyncSleep>>,
    ) -> &mut Self {
        self.sleep_impl = sleep_impl;
        self
    }
    /// Set the retry_config for the builder
    ///
    /// # Examples
    /// ```no_run
    /// use aws_sdk_comprehendmedical::config::Config;
    /// use aws_smithy_types::retry::RetryConfig;
    ///
    /// let retry_config = RetryConfig::new().with_max_attempts(5);
    /// let config = Config::builder().retry_config(retry_config).build();
    /// ```
    pub fn retry_config(mut self, retry_config: aws_smithy_types::retry::RetryConfig) -> Self {
        self.set_retry_config(Some(retry_config));
        self
    }

    /// Set the retry_config for the builder
    ///
    /// # Examples
    /// ```no_run
    /// use aws_sdk_comprehendmedical::config::{Builder, Config};
    /// use aws_smithy_types::retry::RetryConfig;
    ///
    /// fn disable_retries(builder: &mut Builder) {
    ///     let retry_config = RetryConfig::new().with_max_attempts(1);
    ///     builder.set_retry_config(Some(retry_config));
    /// }
    ///
    /// let mut builder = Config::builder();
    /// disable_retries(&mut builder);
    /// let config = builder.build();
    /// ```
    pub fn set_retry_config(
        &mut self,
        retry_config: Option<aws_smithy_types::retry::RetryConfig>,
    ) -> &mut Self {
        self.retry_config = retry_config;
        self
    }
    // TODO(docs): include an example of using a static endpoint
    /// Sets the endpoint resolver to use when making requests.
    pub fn endpoint_resolver(
        mut self,
        endpoint_resolver: impl aws_endpoint::ResolveAwsEndpoint + 'static,
    ) -> Self {
        self.endpoint_resolver = Some(::std::sync::Arc::new(endpoint_resolver));
        self
    }
    /// Sets the AWS region to use when making requests.
    ///
    /// # Examples
    /// ```no_run
    /// use aws_types::region::Region;
    /// use aws_sdk_comprehendmedical::config::{Builder, Config};
    ///
    /// let config = aws_sdk_comprehendmedical::Config::builder()
    ///     .region(Region::new("us-east-1"))
    ///     .build();
    /// ```
    pub fn region(mut self, region: impl Into<Option<aws_types::region::Region>>) -> Self {
        self.region = region.into();
        self
    }
    /// Sets the credentials provider for this service
    pub fn credentials_provider(
        mut self,
        credentials_provider: impl aws_types::credentials::ProvideCredentials + 'static,
    ) -> Self {
        self.credentials_provider = Some(aws_types::credentials::SharedCredentialsProvider::new(
            credentials_provider,
        ));
        self
    }

    /// Sets the credentials provider for this service
    pub fn set_credentials_provider(
        &mut self,
        credentials_provider: Option<aws_types::credentials::SharedCredentialsProvider>,
    ) -> &mut Self {
        self.credentials_provider = credentials_provider;
        self
    }
    /// Builds a [`Config`].
    pub fn build(self) -> Config {
        Config {
            make_token: self
                .make_token
                .unwrap_or_else(crate::idempotency_token::default_provider),
            app_name: self.app_name,
            timeout_config: self.timeout_config,
            sleep_impl: self.sleep_impl,
            retry_config: self.retry_config,
            endpoint_resolver: self
                .endpoint_resolver
                .unwrap_or_else(|| ::std::sync::Arc::new(crate::aws_endpoint::endpoint_resolver())),
            region: self.region,
            credentials_provider: self.credentials_provider.unwrap_or_else(|| {
                aws_types::credentials::SharedCredentialsProvider::new(
                    crate::no_credentials::NoCredentials,
                )
            }),
        }
    }
}

impl From<&aws_types::sdk_config::SdkConfig> for Builder {
    fn from(input: &aws_types::sdk_config::SdkConfig) -> Self {
        let mut builder = Builder::default();
        builder = builder.region(input.region().cloned());
        builder.set_retry_config(input.retry_config().cloned());
        builder.set_timeout_config(input.timeout_config().cloned());
        builder.set_sleep_impl(input.sleep_impl().clone());
        builder.set_credentials_provider(input.credentials_provider().cloned());
        builder.set_app_name(input.app_name().cloned());
        builder
    }
}

impl From<&aws_types::sdk_config::SdkConfig> for Config {
    fn from(sdk_config: &aws_types::sdk_config::SdkConfig) -> Self {
        Builder::from(sdk_config).build()
    }
}
