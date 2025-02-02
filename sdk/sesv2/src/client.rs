// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[derive(Debug)]
pub(crate) struct Handle {
    pub(crate) conf: crate::Config,
    pub(crate) runtime_plugins: ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugins,
}

/// Client for Amazon Simple Email Service
///
/// Client for invoking operations on Amazon Simple Email Service. Each operation on Amazon Simple Email Service is a method on this
/// this struct. `.send()` MUST be invoked on the generated operations to dispatch the request to the service.
/// ## Constructing a `Client`
///
/// A [`Config`] is required to construct a client. For most use cases, the [`aws-config`]
/// crate should be used to automatically resolve this config using
/// [`aws_config::load_from_env()`], since this will resolve an [`SdkConfig`] which can be shared
/// across multiple different AWS SDK clients. This config resolution process can be customized
/// by calling [`aws_config::from_env()`] instead, which returns a [`ConfigLoader`] that uses
/// the [builder pattern] to customize the default config.
///
/// In the simplest case, creating a client looks as follows:
/// ```rust,no_run
/// # async fn wrapper() {
/// let config = aws_config::load_from_env().await;
/// let client = aws_sdk_sesv2::Client::new(&config);
/// # }
/// ```
///
/// Occasionally, SDKs may have additional service-specific that can be set on the [`Config`] that
/// is absent from [`SdkConfig`], or slightly different settings for a specific client may be desired.
/// The [`Config`] struct implements `From<&SdkConfig>`, so setting these specific settings can be
/// done as follows:
///
/// ```rust,no_run
/// # async fn wrapper() {
/// let sdk_config = ::aws_config::load_from_env().await;
/// let config = aws_sdk_sesv2::config::Builder::from(&sdk_config)
/// # /*
///     .some_service_specific_setting("value")
/// # */
///     .build();
/// # }
/// ```
///
/// See the [`aws-config` docs] and [`Config`] for more information on customizing configuration.
///
/// _Note:_ Client construction is expensive due to connection thread pool initialization, and should
/// be done once at application start-up.
///
/// [`Config`]: crate::Config
/// [`ConfigLoader`]: https://docs.rs/aws-config/*/aws_config/struct.ConfigLoader.html
/// [`SdkConfig`]: https://docs.rs/aws-config/*/aws_config/struct.SdkConfig.html
/// [`aws-config` docs]: https://docs.rs/aws-config/*
/// [`aws-config`]: https://crates.io/crates/aws-config
/// [`aws_config::from_env()`]: https://docs.rs/aws-config/*/aws_config/fn.from_env.html
/// [`aws_config::load_from_env()`]: https://docs.rs/aws-config/*/aws_config/fn.load_from_env.html
/// [builder pattern]: https://rust-lang.github.io/api-guidelines/type-safety.html#builders-enable-construction-of-complex-values-c-builder
/// # Using the `Client`
///
/// A client has a function for every operation that can be performed by the service.
/// For example, the [`CreateConfigurationSet`](crate::operation::create_configuration_set) operation has
/// a [`Client::create_configuration_set`], function which returns a builder for that operation.
/// The fluent builder ultimately has a `send()` function that returns an async future that
/// returns a result, as illustrated below:
///
/// ```rust,ignore
/// let result = client.create_configuration_set()
///     .configuration_set_name("example")
///     .send()
///     .await;
/// ```
///
/// The underlying HTTP requests that get made by this can be modified with the `customize_operation`
/// function on the fluent builder. See the [`customize`](crate::client::customize) module for more
/// information.
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct Client {
    handle: ::std::sync::Arc<Handle>,
}

impl Client {
    /// Creates a new client from the service [`Config`](crate::Config).
    ///
    /// # Panics
    ///
    /// This method will panic if the `conf` has retry or timeouts enabled without a `sleep_impl`.
    /// If you experience this panic, it can be fixed by setting the `sleep_impl`, or by disabling
    /// retries and timeouts.
    pub fn from_conf(conf: crate::Config) -> Self {
        let retry_config = conf
            .retry_config()
            .cloned()
            .unwrap_or_else(::aws_smithy_types::retry::RetryConfig::disabled);
        let timeout_config = conf
            .timeout_config()
            .cloned()
            .unwrap_or_else(::aws_smithy_types::timeout::TimeoutConfig::disabled);
        let sleep_impl = conf.sleep_impl();
        if (retry_config.has_retry() || timeout_config.has_timeouts()) && sleep_impl.is_none() {
            panic!(
                "An async sleep implementation is required for retries or timeouts to work. \
                                        Set the `sleep_impl` on the Config passed into this function to fix this panic."
            );
        }

        Self {
            handle: ::std::sync::Arc::new(Handle {
                conf: conf.clone(),
                runtime_plugins: crate::config::base_client_runtime_plugins(conf),
            }),
        }
    }

    /// Returns the client's configuration.
    pub fn config(&self) -> &crate::Config {
        &self.handle.conf
    }

    #[doc(hidden)]
    // TODO(enableNewSmithyRuntimeCleanup): Delete this function when cleaning up middleware
    // This is currently kept around so the tests still compile in both modes
    /// Creates a client with the given service configuration.
    pub fn with_config<C, M, R>(_client: ::aws_smithy_client::Client<C, M, R>, conf: crate::Config) -> Self {
        Self::from_conf(conf)
    }

    #[doc(hidden)]
    // TODO(enableNewSmithyRuntimeCleanup): Delete this function when cleaning up middleware
    // This is currently kept around so the tests still compile in both modes
    /// Returns the client's configuration.
    pub fn conf(&self) -> &crate::Config {
        &self.handle.conf
    }
}

impl Client {
    /// Creates a new client from an [SDK Config](::aws_types::sdk_config::SdkConfig).
    ///
    /// # Panics
    ///
    /// - This method will panic if the `sdk_config` is missing an async sleep implementation. If you experience this panic, set
    ///     the `sleep_impl` on the Config passed into this function to fix it.
    /// - This method will panic if the `sdk_config` is missing an HTTP connector. If you experience this panic, set the
    ///     `http_connector` on the Config passed into this function to fix it.
    pub fn new(sdk_config: &::aws_types::sdk_config::SdkConfig) -> Self {
        Self::from_conf(sdk_config.into())
    }
}

mod batch_get_metric_data;

mod create_configuration_set;

mod create_configuration_set_event_destination;

mod create_contact;

mod create_contact_list;

mod create_custom_verification_email_template;

mod create_dedicated_ip_pool;

mod create_deliverability_test_report;

mod create_email_identity;

mod create_email_identity_policy;

mod create_email_template;

mod create_import_job;

/// Operation customization and supporting types.
///
/// The underlying HTTP requests made during an operation can be customized
/// by calling the `customize()` method on the builder returned from a client
/// operation call. For example, this can be used to add an additional HTTP header:
///
/// ```ignore
/// # async fn wrapper() -> ::std::result::Result<(), aws_sdk_sesv2::Error> {
/// # let client: aws_sdk_sesv2::Client = unimplemented!();
/// use ::http::header::{HeaderName, HeaderValue};
///
/// let result = client.batch_get_metric_data()
///     .customize()
///     .await?
///     .mutate_request(|req| {
///         // Add `x-example-header` with value
///         req.headers_mut()
///             .insert(
///                 HeaderName::from_static("x-example-header"),
///                 HeaderValue::from_static("1"),
///             );
///     })
///     .send()
///     .await;
/// # }
/// ```
pub mod customize;

mod delete_configuration_set;

mod delete_configuration_set_event_destination;

mod delete_contact;

mod delete_contact_list;

mod delete_custom_verification_email_template;

mod delete_dedicated_ip_pool;

mod delete_email_identity;

mod delete_email_identity_policy;

mod delete_email_template;

mod delete_suppressed_destination;

mod get_account;

mod get_blacklist_reports;

mod get_configuration_set;

mod get_configuration_set_event_destinations;

mod get_contact;

mod get_contact_list;

mod get_custom_verification_email_template;

mod get_dedicated_ip;

mod get_dedicated_ip_pool;

mod get_dedicated_ips;

mod get_deliverability_dashboard_options;

mod get_deliverability_test_report;

mod get_domain_deliverability_campaign;

mod get_domain_statistics_report;

mod get_email_identity;

mod get_email_identity_policies;

mod get_email_template;

mod get_import_job;

mod get_suppressed_destination;

mod list_configuration_sets;

mod list_contact_lists;

mod list_contacts;

mod list_custom_verification_email_templates;

mod list_dedicated_ip_pools;

mod list_deliverability_test_reports;

mod list_domain_deliverability_campaigns;

mod list_email_identities;

mod list_email_templates;

mod list_import_jobs;

mod list_recommendations;

mod list_suppressed_destinations;

mod list_tags_for_resource;

mod put_account_dedicated_ip_warmup_attributes;

mod put_account_details;

mod put_account_sending_attributes;

mod put_account_suppression_attributes;

mod put_account_vdm_attributes;

mod put_configuration_set_delivery_options;

mod put_configuration_set_reputation_options;

mod put_configuration_set_sending_options;

mod put_configuration_set_suppression_options;

mod put_configuration_set_tracking_options;

mod put_configuration_set_vdm_options;

mod put_dedicated_ip_in_pool;

mod put_dedicated_ip_pool_scaling_attributes;

mod put_dedicated_ip_warmup_attributes;

mod put_deliverability_dashboard_option;

mod put_email_identity_configuration_set_attributes;

mod put_email_identity_dkim_attributes;

mod put_email_identity_dkim_signing_attributes;

mod put_email_identity_feedback_attributes;

mod put_email_identity_mail_from_attributes;

mod put_suppressed_destination;

mod send_bulk_email;

mod send_custom_verification_email;

mod send_email;

mod tag_resource;

mod test_render_email_template;

mod untag_resource;

mod update_configuration_set_event_destination;

mod update_contact;

mod update_contact_list;

mod update_custom_verification_email_template;

mod update_email_identity_policy;

mod update_email_template;
