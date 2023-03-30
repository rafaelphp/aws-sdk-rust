// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
use std::fmt::Write;

/// See [`BatchPutMetricsInput`](crate::input::BatchPutMetricsInput).
pub mod batch_put_metrics_input {
    
    /// A builder for [`BatchPutMetricsInput`](crate::input::BatchPutMetricsInput).
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) trial_component_name: std::option::Option<std::string::String>,
        pub(crate) metric_data: std::option::Option<std::vec::Vec<crate::model::RawMetricData>>,
    }
    impl Builder {
        /// <p>The name of the Trial Component to associate with the metrics.</p>
        pub fn trial_component_name(mut self, input: impl Into<std::string::String>) -> Self {
            self.trial_component_name = Some(input.into());
            self
        }
        /// <p>The name of the Trial Component to associate with the metrics.</p>
        pub fn set_trial_component_name(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.trial_component_name = input; self
        }
        /// Appends an item to `metric_data`.
        ///
        /// To override the contents of this collection use [`set_metric_data`](Self::set_metric_data).
        ///
        /// <p>A list of raw metric values to put.</p>
        pub fn metric_data(mut self, input: crate::model::RawMetricData) -> Self {
            let mut v = self.metric_data.unwrap_or_default();
                            v.push(input);
                            self.metric_data = Some(v);
                            self
        }
        /// <p>A list of raw metric values to put.</p>
        pub fn set_metric_data(mut self, input: std::option::Option<std::vec::Vec<crate::model::RawMetricData>>) -> Self {
            self.metric_data = input; self
        }
        /// Consumes the builder and constructs a [`BatchPutMetricsInput`](crate::input::BatchPutMetricsInput).
        pub fn build(self) -> Result<crate::input::BatchPutMetricsInput, aws_smithy_http::operation::error::BuildError> {
            Ok(
                crate::input::BatchPutMetricsInput {
                    trial_component_name: self.trial_component_name
                    ,
                    metric_data: self.metric_data
                    ,
                }
            )
        }
    }
    
    
}
impl BatchPutMetricsInput {
    /// Consumes the builder and constructs an Operation<[`BatchPutMetrics`](crate::operation::BatchPutMetrics)>
    #[allow(unused_mut)]
    #[allow(clippy::let_and_return)]
    #[allow(clippy::needless_borrow)]
    pub async fn make_operation(&self, _config: &crate::config::Config) -> std::result::Result<aws_smithy_http::operation::Operation<crate::operation::BatchPutMetrics, aws_http::retry::AwsResponseRetryClassifier>, aws_smithy_http::operation::error::BuildError> {
        let params_result = crate::endpoint::Params::builder().set_region(_config.region.as_ref().map(|r|r.as_ref().to_owned()))
        .set_use_dual_stack(_config.use_dual_stack)
        .set_use_fips(_config.use_fips)
        .set_endpoint(_config.endpoint_url
        .clone()).build()
                                    .map_err(|err|aws_smithy_http::endpoint::ResolveEndpointError::from_source("could not construct endpoint parameters", err));
                                let (endpoint_result, params) = match params_result {
                                    Ok(params) => (_config.endpoint_resolver.resolve_endpoint(&params), Some(params)),
                                    Err(e) => (Err(e), None)
                                };
        let mut request = {
            fn uri_base(_input: &crate::input::BatchPutMetricsInput, output: &mut String) -> Result<(), aws_smithy_http::operation::error::BuildError> {
                write!(output, "/BatchPutMetrics").expect("formatting should succeed");
                Ok(())
            }
            #[allow(clippy::unnecessary_wraps)]
            fn update_http_builder(
                            input: &crate::input::BatchPutMetricsInput,
                            builder: http::request::Builder
                        ) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::error::BuildError> {
                let mut uri = String::new();
                uri_base(input, &mut uri)?;
                Ok(builder.method("PUT").uri(uri))
            }
            let mut builder = update_http_builder(&self, http::request::Builder::new())?;
            builder = aws_smithy_http::header::set_request_header_if_absent(builder, http::header::CONTENT_TYPE, "application/json");
            builder
        };
        let mut properties = aws_smithy_http::property_bag::SharedPropertyBag::new();
        #[allow(clippy::useless_conversion)]
        let body = aws_smithy_http::body::SdkBody::from(
            crate::operation_ser::serialize_operation_crate_operation_batch_put_metrics(&self)?
        );
        if let Some(content_length) = body.content_length() {
                                request = aws_smithy_http::header::set_request_header_if_absent(request, http::header::CONTENT_LENGTH, content_length);
                            }
        let request = request.body(body).expect("should be valid request");
        let mut request = aws_smithy_http::operation::Request::from_parts(request, properties);
        request.properties_mut().insert(endpoint_result);
        if let Some(params) = params { request.properties_mut().insert(params); }
        request.properties_mut().insert(aws_smithy_http::http_versions::DEFAULT_HTTP_VERSION_LIST.clone());
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
                            request.properties_mut().insert(aws_types::SigningService::from_static(_config.signing_service()));
                            if let Some(region) = &_config.region {
                                request.properties_mut().insert(aws_types::region::SigningRegion::from(region.clone()));
                            }
        if let Some(region) = &_config.region {
                                request.properties_mut().insert(region.clone());
                            }
        aws_http::auth::set_credentials_cache(&mut request.properties_mut(), _config.credentials_cache.clone());
        let op = aws_smithy_http::operation::Operation::new(request, crate::operation::BatchPutMetrics::new())
                            .with_metadata(aws_smithy_http::operation::Metadata::new("BatchPutMetrics", "sagemakermetrics"));
        let op = op.with_retry_classifier(aws_http::retry::AwsResponseRetryClassifier::new());
        Ok(op)
    }
    /// Creates a new builder-style object to manufacture [`BatchPutMetricsInput`](crate::input::BatchPutMetricsInput).
    pub fn builder() -> crate::input::batch_put_metrics_input::Builder {
        crate::input::batch_put_metrics_input::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct BatchPutMetricsInput  {
    /// <p>The name of the Trial Component to associate with the metrics.</p>
    #[doc(hidden)]
    pub trial_component_name: std::option::Option<std::string::String>,
    /// <p>A list of raw metric values to put.</p>
    #[doc(hidden)]
    pub metric_data: std::option::Option<std::vec::Vec<crate::model::RawMetricData>>,
}
impl BatchPutMetricsInput {
    /// <p>The name of the Trial Component to associate with the metrics.</p>
    pub fn trial_component_name(&self) -> std::option::Option<& str> {
        self.trial_component_name.as_deref()
    }
    /// <p>A list of raw metric values to put.</p>
    pub fn metric_data(&self) -> std::option::Option<& [crate::model::RawMetricData]> {
        self.metric_data.as_deref()
    }
}

