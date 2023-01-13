// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
use std::fmt::Write;

/// See [`GetLatestConfigurationInput`](crate::input::GetLatestConfigurationInput).
pub mod get_latest_configuration_input {

    /// A builder for [`GetLatestConfigurationInput`](crate::input::GetLatestConfigurationInput).
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) configuration_token: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>Token describing the current state of the configuration session. To obtain a token, first call the <code>StartConfigurationSession</code> API. Note that every call to <code>GetLatestConfiguration</code> will return a new <code>ConfigurationToken</code> (<code>NextPollConfigurationToken</code> in the response) and MUST be provided to subsequent <code>GetLatestConfiguration</code> API calls.</p>
        pub fn configuration_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.configuration_token = Some(input.into());
            self
        }
        /// <p>Token describing the current state of the configuration session. To obtain a token, first call the <code>StartConfigurationSession</code> API. Note that every call to <code>GetLatestConfiguration</code> will return a new <code>ConfigurationToken</code> (<code>NextPollConfigurationToken</code> in the response) and MUST be provided to subsequent <code>GetLatestConfiguration</code> API calls.</p>
        pub fn set_configuration_token(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.configuration_token = input;
            self
        }
        /// Consumes the builder and constructs a [`GetLatestConfigurationInput`](crate::input::GetLatestConfigurationInput).
        pub fn build(
            self,
        ) -> Result<
            crate::input::GetLatestConfigurationInput,
            aws_smithy_http::operation::error::BuildError,
        > {
            Ok(crate::input::GetLatestConfigurationInput {
                configuration_token: self.configuration_token,
            })
        }
    }
}
impl GetLatestConfigurationInput {
    /// Consumes the builder and constructs an Operation<[`GetLatestConfiguration`](crate::operation::GetLatestConfiguration)>
    #[allow(unused_mut)]
    #[allow(clippy::let_and_return)]
    #[allow(clippy::needless_borrow)]
    pub async fn make_operation(
        &self,
        _config: &crate::config::Config,
    ) -> std::result::Result<
        aws_smithy_http::operation::Operation<
            crate::operation::GetLatestConfiguration,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::operation::error::BuildError,
    > {
        let params_result = crate::endpoint::Params::builder()
            .set_region(_config.region.as_ref().map(|r| r.as_ref().to_owned()))
            .set_endpoint(_config.endpoint_url().map(|url| url.to_string()))
            .set_use_fips(_config.use_fips)
            .set_use_dual_stack(_config.use_dual_stack)
            .build()
            .map_err(|err| {
                aws_smithy_http::endpoint::ResolveEndpointError::from_source(
                    "could not construct endpoint parameters",
                    err,
                )
            });
        let (endpoint_result, params) = match params_result {
            Ok(params) => (
                _config.endpoint_resolver.resolve_endpoint(&params),
                Some(params),
            ),
            Err(e) => (Err(e), None),
        };
        let mut request = {
            fn uri_base(
                _input: &crate::input::GetLatestConfigurationInput,
                output: &mut String,
            ) -> Result<(), aws_smithy_http::operation::error::BuildError> {
                write!(output, "/configuration").expect("formatting should succeed");
                Ok(())
            }
            fn uri_query(
                _input: &crate::input::GetLatestConfigurationInput,
                mut output: &mut String,
            ) -> Result<(), aws_smithy_http::operation::error::BuildError> {
                let mut query = aws_smithy_http::query::Writer::new(&mut output);
                let inner_1 = &_input.configuration_token;
                let inner_1 = inner_1.as_ref().ok_or_else(|| {
                    aws_smithy_http::operation::error::BuildError::missing_field(
                        "configuration_token",
                        "cannot be empty or unset",
                    )
                })?;
                if inner_1.is_empty() {
                    return Err(
                        aws_smithy_http::operation::error::BuildError::missing_field(
                            "configuration_token",
                            "cannot be empty or unset",
                        ),
                    );
                }
                query.push_kv(
                    "configuration_token",
                    &aws_smithy_http::query::fmt_string(&inner_1),
                );
                Ok(())
            }
            #[allow(clippy::unnecessary_wraps)]
            fn update_http_builder(
                input: &crate::input::GetLatestConfigurationInput,
                builder: http::request::Builder,
            ) -> std::result::Result<
                http::request::Builder,
                aws_smithy_http::operation::error::BuildError,
            > {
                let mut uri = String::new();
                uri_base(input, &mut uri)?;
                uri_query(input, &mut uri)?;
                Ok(builder.method("GET").uri(uri))
            }
            let mut builder = update_http_builder(&self, http::request::Builder::new())?;
            builder
        };
        let mut properties = aws_smithy_http::property_bag::SharedPropertyBag::new();
        #[allow(clippy::useless_conversion)]
        let body = aws_smithy_http::body::SdkBody::from("");
        let request = request.body(body).expect("should be valid request");
        let mut request = aws_smithy_http::operation::Request::from_parts(request, properties);
        request.properties_mut().insert(endpoint_result);
        if let Some(params) = params {
            request.properties_mut().insert(params);
        }
        request
            .properties_mut()
            .insert(aws_smithy_http::http_versions::DEFAULT_HTTP_VERSION_LIST.clone());
        let mut user_agent = aws_http::user_agent::AwsUserAgent::new_from_environment(
            aws_types::os_shim_internal::Env::real(),
            crate::API_METADATA.clone(),
        );
        if let Some(app_name) = _config.app_name() {
            user_agent = user_agent.with_app_name(app_name.clone());
        }
        request.properties_mut().insert(user_agent);
        let mut signing_config = aws_sig_auth::signer::OperationSigningConfig::default_config();
        request.properties_mut().insert(signing_config);
        request
            .properties_mut()
            .insert(aws_types::SigningService::from_static(
                _config.signing_service(),
            ));
        if let Some(region) = &_config.region {
            request
                .properties_mut()
                .insert(aws_types::region::SigningRegion::from(region.clone()));
        }
        if let Some(region) = &_config.region {
            request.properties_mut().insert(region.clone());
        }
        aws_http::auth::set_provider(
            &mut request.properties_mut(),
            _config.credentials_provider.clone(),
        );
        let op = aws_smithy_http::operation::Operation::new(
            request,
            crate::operation::GetLatestConfiguration::new(),
        )
        .with_metadata(aws_smithy_http::operation::Metadata::new(
            "GetLatestConfiguration",
            "appconfigdata",
        ));
        let op = op.with_retry_classifier(aws_http::retry::AwsResponseRetryClassifier::new());
        Ok(op)
    }
    /// Creates a new builder-style object to manufacture [`GetLatestConfigurationInput`](crate::input::GetLatestConfigurationInput).
    pub fn builder() -> crate::input::get_latest_configuration_input::Builder {
        crate::input::get_latest_configuration_input::Builder::default()
    }
}

/// See [`StartConfigurationSessionInput`](crate::input::StartConfigurationSessionInput).
pub mod start_configuration_session_input {

    /// A builder for [`StartConfigurationSessionInput`](crate::input::StartConfigurationSessionInput).
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) application_identifier: std::option::Option<std::string::String>,
        pub(crate) environment_identifier: std::option::Option<std::string::String>,
        pub(crate) configuration_profile_identifier: std::option::Option<std::string::String>,
        pub(crate) required_minimum_poll_interval_in_seconds: std::option::Option<i32>,
    }
    impl Builder {
        /// <p>The application ID or the application name.</p>
        pub fn application_identifier(mut self, input: impl Into<std::string::String>) -> Self {
            self.application_identifier = Some(input.into());
            self
        }
        /// <p>The application ID or the application name.</p>
        pub fn set_application_identifier(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.application_identifier = input;
            self
        }
        /// <p>The environment ID or the environment name.</p>
        pub fn environment_identifier(mut self, input: impl Into<std::string::String>) -> Self {
            self.environment_identifier = Some(input.into());
            self
        }
        /// <p>The environment ID or the environment name.</p>
        pub fn set_environment_identifier(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.environment_identifier = input;
            self
        }
        /// <p>The configuration profile ID or the configuration profile name.</p>
        pub fn configuration_profile_identifier(
            mut self,
            input: impl Into<std::string::String>,
        ) -> Self {
            self.configuration_profile_identifier = Some(input.into());
            self
        }
        /// <p>The configuration profile ID or the configuration profile name.</p>
        pub fn set_configuration_profile_identifier(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.configuration_profile_identifier = input;
            self
        }
        /// <p>Sets a constraint on a session. If you specify a value of, for example, 60 seconds, then the client that established the session can't call <code>GetLatestConfiguration</code> more frequently then every 60 seconds.</p>
        pub fn required_minimum_poll_interval_in_seconds(mut self, input: i32) -> Self {
            self.required_minimum_poll_interval_in_seconds = Some(input);
            self
        }
        /// <p>Sets a constraint on a session. If you specify a value of, for example, 60 seconds, then the client that established the session can't call <code>GetLatestConfiguration</code> more frequently then every 60 seconds.</p>
        pub fn set_required_minimum_poll_interval_in_seconds(
            mut self,
            input: std::option::Option<i32>,
        ) -> Self {
            self.required_minimum_poll_interval_in_seconds = input;
            self
        }
        /// Consumes the builder and constructs a [`StartConfigurationSessionInput`](crate::input::StartConfigurationSessionInput).
        pub fn build(
            self,
        ) -> Result<
            crate::input::StartConfigurationSessionInput,
            aws_smithy_http::operation::error::BuildError,
        > {
            Ok(crate::input::StartConfigurationSessionInput {
                application_identifier: self.application_identifier,
                environment_identifier: self.environment_identifier,
                configuration_profile_identifier: self.configuration_profile_identifier,
                required_minimum_poll_interval_in_seconds: self
                    .required_minimum_poll_interval_in_seconds,
            })
        }
    }
}
impl StartConfigurationSessionInput {
    /// Consumes the builder and constructs an Operation<[`StartConfigurationSession`](crate::operation::StartConfigurationSession)>
    #[allow(unused_mut)]
    #[allow(clippy::let_and_return)]
    #[allow(clippy::needless_borrow)]
    pub async fn make_operation(
        &self,
        _config: &crate::config::Config,
    ) -> std::result::Result<
        aws_smithy_http::operation::Operation<
            crate::operation::StartConfigurationSession,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::operation::error::BuildError,
    > {
        let params_result = crate::endpoint::Params::builder()
            .set_region(_config.region.as_ref().map(|r| r.as_ref().to_owned()))
            .set_endpoint(_config.endpoint_url().map(|url| url.to_string()))
            .set_use_fips(_config.use_fips)
            .set_use_dual_stack(_config.use_dual_stack)
            .build()
            .map_err(|err| {
                aws_smithy_http::endpoint::ResolveEndpointError::from_source(
                    "could not construct endpoint parameters",
                    err,
                )
            });
        let (endpoint_result, params) = match params_result {
            Ok(params) => (
                _config.endpoint_resolver.resolve_endpoint(&params),
                Some(params),
            ),
            Err(e) => (Err(e), None),
        };
        let mut request = {
            fn uri_base(
                _input: &crate::input::StartConfigurationSessionInput,
                output: &mut String,
            ) -> Result<(), aws_smithy_http::operation::error::BuildError> {
                write!(output, "/configurationsessions").expect("formatting should succeed");
                Ok(())
            }
            #[allow(clippy::unnecessary_wraps)]
            fn update_http_builder(
                input: &crate::input::StartConfigurationSessionInput,
                builder: http::request::Builder,
            ) -> std::result::Result<
                http::request::Builder,
                aws_smithy_http::operation::error::BuildError,
            > {
                let mut uri = String::new();
                uri_base(input, &mut uri)?;
                Ok(builder.method("POST").uri(uri))
            }
            let mut builder = update_http_builder(&self, http::request::Builder::new())?;
            builder = aws_smithy_http::header::set_request_header_if_absent(
                builder,
                http::header::CONTENT_TYPE,
                "application/json",
            );
            builder
        };
        let mut properties = aws_smithy_http::property_bag::SharedPropertyBag::new();
        #[allow(clippy::useless_conversion)]
        let body = aws_smithy_http::body::SdkBody::from(
            crate::operation_ser::serialize_operation_crate_operation_start_configuration_session(
                &self,
            )?,
        );
        if let Some(content_length) = body.content_length() {
            request = aws_smithy_http::header::set_request_header_if_absent(
                request,
                http::header::CONTENT_LENGTH,
                content_length,
            );
        }
        let request = request.body(body).expect("should be valid request");
        let mut request = aws_smithy_http::operation::Request::from_parts(request, properties);
        request.properties_mut().insert(endpoint_result);
        if let Some(params) = params {
            request.properties_mut().insert(params);
        }
        request
            .properties_mut()
            .insert(aws_smithy_http::http_versions::DEFAULT_HTTP_VERSION_LIST.clone());
        let mut user_agent = aws_http::user_agent::AwsUserAgent::new_from_environment(
            aws_types::os_shim_internal::Env::real(),
            crate::API_METADATA.clone(),
        );
        if let Some(app_name) = _config.app_name() {
            user_agent = user_agent.with_app_name(app_name.clone());
        }
        request.properties_mut().insert(user_agent);
        let mut signing_config = aws_sig_auth::signer::OperationSigningConfig::default_config();
        request.properties_mut().insert(signing_config);
        request
            .properties_mut()
            .insert(aws_types::SigningService::from_static(
                _config.signing_service(),
            ));
        if let Some(region) = &_config.region {
            request
                .properties_mut()
                .insert(aws_types::region::SigningRegion::from(region.clone()));
        }
        if let Some(region) = &_config.region {
            request.properties_mut().insert(region.clone());
        }
        aws_http::auth::set_provider(
            &mut request.properties_mut(),
            _config.credentials_provider.clone(),
        );
        let op = aws_smithy_http::operation::Operation::new(
            request,
            crate::operation::StartConfigurationSession::new(),
        )
        .with_metadata(aws_smithy_http::operation::Metadata::new(
            "StartConfigurationSession",
            "appconfigdata",
        ));
        let op = op.with_retry_classifier(aws_http::retry::AwsResponseRetryClassifier::new());
        Ok(op)
    }
    /// Creates a new builder-style object to manufacture [`StartConfigurationSessionInput`](crate::input::StartConfigurationSessionInput).
    pub fn builder() -> crate::input::start_configuration_session_input::Builder {
        crate::input::start_configuration_session_input::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct StartConfigurationSessionInput {
    /// <p>The application ID or the application name.</p>
    #[doc(hidden)]
    pub application_identifier: std::option::Option<std::string::String>,
    /// <p>The environment ID or the environment name.</p>
    #[doc(hidden)]
    pub environment_identifier: std::option::Option<std::string::String>,
    /// <p>The configuration profile ID or the configuration profile name.</p>
    #[doc(hidden)]
    pub configuration_profile_identifier: std::option::Option<std::string::String>,
    /// <p>Sets a constraint on a session. If you specify a value of, for example, 60 seconds, then the client that established the session can't call <code>GetLatestConfiguration</code> more frequently then every 60 seconds.</p>
    #[doc(hidden)]
    pub required_minimum_poll_interval_in_seconds: std::option::Option<i32>,
}
impl StartConfigurationSessionInput {
    /// <p>The application ID or the application name.</p>
    pub fn application_identifier(&self) -> std::option::Option<&str> {
        self.application_identifier.as_deref()
    }
    /// <p>The environment ID or the environment name.</p>
    pub fn environment_identifier(&self) -> std::option::Option<&str> {
        self.environment_identifier.as_deref()
    }
    /// <p>The configuration profile ID or the configuration profile name.</p>
    pub fn configuration_profile_identifier(&self) -> std::option::Option<&str> {
        self.configuration_profile_identifier.as_deref()
    }
    /// <p>Sets a constraint on a session. If you specify a value of, for example, 60 seconds, then the client that established the session can't call <code>GetLatestConfiguration</code> more frequently then every 60 seconds.</p>
    pub fn required_minimum_poll_interval_in_seconds(&self) -> std::option::Option<i32> {
        self.required_minimum_poll_interval_in_seconds
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct GetLatestConfigurationInput {
    /// <p>Token describing the current state of the configuration session. To obtain a token, first call the <code>StartConfigurationSession</code> API. Note that every call to <code>GetLatestConfiguration</code> will return a new <code>ConfigurationToken</code> (<code>NextPollConfigurationToken</code> in the response) and MUST be provided to subsequent <code>GetLatestConfiguration</code> API calls.</p>
    #[doc(hidden)]
    pub configuration_token: std::option::Option<std::string::String>,
}
impl GetLatestConfigurationInput {
    /// <p>Token describing the current state of the configuration session. To obtain a token, first call the <code>StartConfigurationSession</code> API. Note that every call to <code>GetLatestConfiguration</code> will return a new <code>ConfigurationToken</code> (<code>NextPollConfigurationToken</code> in the response) and MUST be provided to subsequent <code>GetLatestConfiguration</code> API calls.</p>
    pub fn configuration_token(&self) -> std::option::Option<&str> {
        self.configuration_token.as_deref()
    }
}
