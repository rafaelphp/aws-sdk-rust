// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct UpdateEnvironmentOutput  {
    /// <p>The Amazon Resource Name (ARN) of the Amazon MWAA environment. For example, <code>arn:aws:airflow:us-east-1:123456789012:environment/MyMWAAEnvironment</code>.</p>
    #[doc(hidden)]
    pub arn: std::option::Option<std::string::String>,
}
impl UpdateEnvironmentOutput {
    /// <p>The Amazon Resource Name (ARN) of the Amazon MWAA environment. For example, <code>arn:aws:airflow:us-east-1:123456789012:environment/MyMWAAEnvironment</code>.</p>
    pub fn arn(&self) -> std::option::Option<& str> {
        self.arn.as_deref()
    }
}
/// See [`UpdateEnvironmentOutput`](crate::output::UpdateEnvironmentOutput).
pub mod update_environment_output {
    
    /// A builder for [`UpdateEnvironmentOutput`](crate::output::UpdateEnvironmentOutput).
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) arn: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>The Amazon Resource Name (ARN) of the Amazon MWAA environment. For example, <code>arn:aws:airflow:us-east-1:123456789012:environment/MyMWAAEnvironment</code>.</p>
        pub fn arn(mut self, input: impl Into<std::string::String>) -> Self {
            self.arn = Some(input.into());
            self
        }
        /// <p>The Amazon Resource Name (ARN) of the Amazon MWAA environment. For example, <code>arn:aws:airflow:us-east-1:123456789012:environment/MyMWAAEnvironment</code>.</p>
        pub fn set_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.arn = input; self
        }
        /// Consumes the builder and constructs a [`UpdateEnvironmentOutput`](crate::output::UpdateEnvironmentOutput).
        pub fn build(self) -> crate::output::UpdateEnvironmentOutput {
            crate::output::UpdateEnvironmentOutput {
                arn: self.arn
                ,
            }
        }
    }
    
    
}
impl UpdateEnvironmentOutput {
    /// Creates a new builder-style object to manufacture [`UpdateEnvironmentOutput`](crate::output::UpdateEnvironmentOutput).
    pub fn builder() -> crate::output::update_environment_output::Builder {
        crate::output::update_environment_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct UntagResourceOutput  {
}
/// See [`UntagResourceOutput`](crate::output::UntagResourceOutput).
pub mod untag_resource_output {
    
    /// A builder for [`UntagResourceOutput`](crate::output::UntagResourceOutput).
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
    }
    impl Builder {
        /// Consumes the builder and constructs a [`UntagResourceOutput`](crate::output::UntagResourceOutput).
        pub fn build(self) -> crate::output::UntagResourceOutput {
            crate::output::UntagResourceOutput {
            }
        }
    }
    
    
}
impl UntagResourceOutput {
    /// Creates a new builder-style object to manufacture [`UntagResourceOutput`](crate::output::UntagResourceOutput).
    pub fn builder() -> crate::output::untag_resource_output::Builder {
        crate::output::untag_resource_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct TagResourceOutput  {
}
/// See [`TagResourceOutput`](crate::output::TagResourceOutput).
pub mod tag_resource_output {
    
    /// A builder for [`TagResourceOutput`](crate::output::TagResourceOutput).
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
    }
    impl Builder {
        /// Consumes the builder and constructs a [`TagResourceOutput`](crate::output::TagResourceOutput).
        pub fn build(self) -> crate::output::TagResourceOutput {
            crate::output::TagResourceOutput {
            }
        }
    }
    
    
}
impl TagResourceOutput {
    /// Creates a new builder-style object to manufacture [`TagResourceOutput`](crate::output::TagResourceOutput).
    pub fn builder() -> crate::output::tag_resource_output::Builder {
        crate::output::tag_resource_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct PublishMetricsOutput  {
}
/// See [`PublishMetricsOutput`](crate::output::PublishMetricsOutput).
pub mod publish_metrics_output {
    
    /// A builder for [`PublishMetricsOutput`](crate::output::PublishMetricsOutput).
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
    }
    impl Builder {
        /// Consumes the builder and constructs a [`PublishMetricsOutput`](crate::output::PublishMetricsOutput).
        pub fn build(self) -> crate::output::PublishMetricsOutput {
            crate::output::PublishMetricsOutput {
            }
        }
    }
    
    
}
impl PublishMetricsOutput {
    /// Creates a new builder-style object to manufacture [`PublishMetricsOutput`](crate::output::PublishMetricsOutput).
    pub fn builder() -> crate::output::publish_metrics_output::Builder {
        crate::output::publish_metrics_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct ListTagsForResourceOutput  {
    /// <p>The key-value tag pairs associated to your environment. To learn more, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws_tagging.html">Tagging Amazon Web Services resources</a>.</p>
    #[doc(hidden)]
    pub tags: std::option::Option<std::collections::HashMap<std::string::String, std::string::String>>,
}
impl ListTagsForResourceOutput {
    /// <p>The key-value tag pairs associated to your environment. To learn more, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws_tagging.html">Tagging Amazon Web Services resources</a>.</p>
    pub fn tags(&self) -> std::option::Option<& std::collections::HashMap<std::string::String, std::string::String>> {
        self.tags.as_ref()
    }
}
/// See [`ListTagsForResourceOutput`](crate::output::ListTagsForResourceOutput).
pub mod list_tags_for_resource_output {
    
    /// A builder for [`ListTagsForResourceOutput`](crate::output::ListTagsForResourceOutput).
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) tags: std::option::Option<std::collections::HashMap<std::string::String, std::string::String>>,
    }
    impl Builder {
        /// Adds a key-value pair to `tags`.
        ///
        /// To override the contents of this collection use [`set_tags`](Self::set_tags).
        ///
        /// <p>The key-value tag pairs associated to your environment. To learn more, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws_tagging.html">Tagging Amazon Web Services resources</a>.</p>
        pub fn tags(mut self, k: impl Into<std::string::String>, v: impl Into<std::string::String>) -> Self {
            let mut hash_map = self.tags.unwrap_or_default();
                            hash_map.insert(k.into(), v.into());
                            self.tags = Some(hash_map);
                            self
        }
        /// <p>The key-value tag pairs associated to your environment. To learn more, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws_tagging.html">Tagging Amazon Web Services resources</a>.</p>
        pub fn set_tags(mut self, input: std::option::Option<std::collections::HashMap<std::string::String, std::string::String>>) -> Self {
            self.tags = input; self
        }
        /// Consumes the builder and constructs a [`ListTagsForResourceOutput`](crate::output::ListTagsForResourceOutput).
        pub fn build(self) -> crate::output::ListTagsForResourceOutput {
            crate::output::ListTagsForResourceOutput {
                tags: self.tags
                ,
            }
        }
    }
    
    
}
impl ListTagsForResourceOutput {
    /// Creates a new builder-style object to manufacture [`ListTagsForResourceOutput`](crate::output::ListTagsForResourceOutput).
    pub fn builder() -> crate::output::list_tags_for_resource_output::Builder {
        crate::output::list_tags_for_resource_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct ListEnvironmentsOutput  {
    /// <p>Returns a list of Amazon MWAA environments.</p>
    #[doc(hidden)]
    pub environments: std::option::Option<std::vec::Vec<std::string::String>>,
    /// <p>Retrieves the next page of the results.</p>
    #[doc(hidden)]
    pub next_token: std::option::Option<std::string::String>,
}
impl ListEnvironmentsOutput {
    /// <p>Returns a list of Amazon MWAA environments.</p>
    pub fn environments(&self) -> std::option::Option<& [std::string::String]> {
        self.environments.as_deref()
    }
    /// <p>Retrieves the next page of the results.</p>
    pub fn next_token(&self) -> std::option::Option<& str> {
        self.next_token.as_deref()
    }
}
/// See [`ListEnvironmentsOutput`](crate::output::ListEnvironmentsOutput).
pub mod list_environments_output {
    
    /// A builder for [`ListEnvironmentsOutput`](crate::output::ListEnvironmentsOutput).
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) environments: std::option::Option<std::vec::Vec<std::string::String>>,
        pub(crate) next_token: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// Appends an item to `environments`.
        ///
        /// To override the contents of this collection use [`set_environments`](Self::set_environments).
        ///
        /// <p>Returns a list of Amazon MWAA environments.</p>
        pub fn environments(mut self, input: impl Into<std::string::String>) -> Self {
            let mut v = self.environments.unwrap_or_default();
                            v.push(input.into());
                            self.environments = Some(v);
                            self
        }
        /// <p>Returns a list of Amazon MWAA environments.</p>
        pub fn set_environments(mut self, input: std::option::Option<std::vec::Vec<std::string::String>>) -> Self {
            self.environments = input; self
        }
        /// <p>Retrieves the next page of the results.</p>
        pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.next_token = Some(input.into());
            self
        }
        /// <p>Retrieves the next page of the results.</p>
        pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.next_token = input; self
        }
        /// Consumes the builder and constructs a [`ListEnvironmentsOutput`](crate::output::ListEnvironmentsOutput).
        pub fn build(self) -> crate::output::ListEnvironmentsOutput {
            crate::output::ListEnvironmentsOutput {
                environments: self.environments
                ,
                next_token: self.next_token
                ,
            }
        }
    }
    
    
}
impl ListEnvironmentsOutput {
    /// Creates a new builder-style object to manufacture [`ListEnvironmentsOutput`](crate::output::ListEnvironmentsOutput).
    pub fn builder() -> crate::output::list_environments_output::Builder {
        crate::output::list_environments_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct GetEnvironmentOutput  {
    /// <p>An object containing all available details about the environment.</p>
    #[doc(hidden)]
    pub environment: std::option::Option<crate::model::Environment>,
}
impl GetEnvironmentOutput {
    /// <p>An object containing all available details about the environment.</p>
    pub fn environment(&self) -> std::option::Option<& crate::model::Environment> {
        self.environment.as_ref()
    }
}
/// See [`GetEnvironmentOutput`](crate::output::GetEnvironmentOutput).
pub mod get_environment_output {
    
    /// A builder for [`GetEnvironmentOutput`](crate::output::GetEnvironmentOutput).
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) environment: std::option::Option<crate::model::Environment>,
    }
    impl Builder {
        /// <p>An object containing all available details about the environment.</p>
        pub fn environment(mut self, input: crate::model::Environment) -> Self {
            self.environment = Some(input);
            self
        }
        /// <p>An object containing all available details about the environment.</p>
        pub fn set_environment(mut self, input: std::option::Option<crate::model::Environment>) -> Self {
            self.environment = input; self
        }
        /// Consumes the builder and constructs a [`GetEnvironmentOutput`](crate::output::GetEnvironmentOutput).
        pub fn build(self) -> crate::output::GetEnvironmentOutput {
            crate::output::GetEnvironmentOutput {
                environment: self.environment
                ,
            }
        }
    }
    
    
}
impl GetEnvironmentOutput {
    /// Creates a new builder-style object to manufacture [`GetEnvironmentOutput`](crate::output::GetEnvironmentOutput).
    pub fn builder() -> crate::output::get_environment_output::Builder {
        crate::output::get_environment_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct DeleteEnvironmentOutput  {
}
/// See [`DeleteEnvironmentOutput`](crate::output::DeleteEnvironmentOutput).
pub mod delete_environment_output {
    
    /// A builder for [`DeleteEnvironmentOutput`](crate::output::DeleteEnvironmentOutput).
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
    }
    impl Builder {
        /// Consumes the builder and constructs a [`DeleteEnvironmentOutput`](crate::output::DeleteEnvironmentOutput).
        pub fn build(self) -> crate::output::DeleteEnvironmentOutput {
            crate::output::DeleteEnvironmentOutput {
            }
        }
    }
    
    
}
impl DeleteEnvironmentOutput {
    /// Creates a new builder-style object to manufacture [`DeleteEnvironmentOutput`](crate::output::DeleteEnvironmentOutput).
    pub fn builder() -> crate::output::delete_environment_output::Builder {
        crate::output::delete_environment_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct CreateWebLoginTokenOutput  {
    /// <p>An Airflow web server login token.</p>
    #[doc(hidden)]
    pub web_token: std::option::Option<std::string::String>,
    /// <p>The Airflow web server hostname for the environment.</p>
    #[doc(hidden)]
    pub web_server_hostname: std::option::Option<std::string::String>,
}
impl CreateWebLoginTokenOutput {
    /// <p>An Airflow web server login token.</p>
    pub fn web_token(&self) -> std::option::Option<& str> {
        self.web_token.as_deref()
    }
    /// <p>The Airflow web server hostname for the environment.</p>
    pub fn web_server_hostname(&self) -> std::option::Option<& str> {
        self.web_server_hostname.as_deref()
    }
}
impl  std::fmt::Debug for CreateWebLoginTokenOutput  {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("CreateWebLoginTokenOutput");
        formatter.field("web_token", &"*** Sensitive Data Redacted ***");
        formatter.field("web_server_hostname", &self.web_server_hostname);
        formatter.finish()
    }
}
/// See [`CreateWebLoginTokenOutput`](crate::output::CreateWebLoginTokenOutput).
pub mod create_web_login_token_output {
    
    /// A builder for [`CreateWebLoginTokenOutput`](crate::output::CreateWebLoginTokenOutput).
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default)]
    pub struct Builder {
        pub(crate) web_token: std::option::Option<std::string::String>,
        pub(crate) web_server_hostname: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>An Airflow web server login token.</p>
        pub fn web_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.web_token = Some(input.into());
            self
        }
        /// <p>An Airflow web server login token.</p>
        pub fn set_web_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.web_token = input; self
        }
        /// <p>The Airflow web server hostname for the environment.</p>
        pub fn web_server_hostname(mut self, input: impl Into<std::string::String>) -> Self {
            self.web_server_hostname = Some(input.into());
            self
        }
        /// <p>The Airflow web server hostname for the environment.</p>
        pub fn set_web_server_hostname(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.web_server_hostname = input; self
        }
        /// Consumes the builder and constructs a [`CreateWebLoginTokenOutput`](crate::output::CreateWebLoginTokenOutput).
        pub fn build(self) -> crate::output::CreateWebLoginTokenOutput {
            crate::output::CreateWebLoginTokenOutput {
                web_token: self.web_token
                ,
                web_server_hostname: self.web_server_hostname
                ,
            }
        }
    }
    impl std::fmt::Debug for Builder {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let mut formatter = f.debug_struct("Builder");
            formatter.field("web_token", &"*** Sensitive Data Redacted ***");
            formatter.field("web_server_hostname", &self.web_server_hostname);
            formatter.finish()
        }
    }
    
    
}
impl CreateWebLoginTokenOutput {
    /// Creates a new builder-style object to manufacture [`CreateWebLoginTokenOutput`](crate::output::CreateWebLoginTokenOutput).
    pub fn builder() -> crate::output::create_web_login_token_output::Builder {
        crate::output::create_web_login_token_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct CreateEnvironmentOutput  {
    /// <p>The Amazon Resource Name (ARN) returned in the response for the environment.</p>
    #[doc(hidden)]
    pub arn: std::option::Option<std::string::String>,
}
impl CreateEnvironmentOutput {
    /// <p>The Amazon Resource Name (ARN) returned in the response for the environment.</p>
    pub fn arn(&self) -> std::option::Option<& str> {
        self.arn.as_deref()
    }
}
/// See [`CreateEnvironmentOutput`](crate::output::CreateEnvironmentOutput).
pub mod create_environment_output {
    
    /// A builder for [`CreateEnvironmentOutput`](crate::output::CreateEnvironmentOutput).
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) arn: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>The Amazon Resource Name (ARN) returned in the response for the environment.</p>
        pub fn arn(mut self, input: impl Into<std::string::String>) -> Self {
            self.arn = Some(input.into());
            self
        }
        /// <p>The Amazon Resource Name (ARN) returned in the response for the environment.</p>
        pub fn set_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.arn = input; self
        }
        /// Consumes the builder and constructs a [`CreateEnvironmentOutput`](crate::output::CreateEnvironmentOutput).
        pub fn build(self) -> crate::output::CreateEnvironmentOutput {
            crate::output::CreateEnvironmentOutput {
                arn: self.arn
                ,
            }
        }
    }
    
    
}
impl CreateEnvironmentOutput {
    /// Creates a new builder-style object to manufacture [`CreateEnvironmentOutput`](crate::output::CreateEnvironmentOutput).
    pub fn builder() -> crate::output::create_environment_output::Builder {
        crate::output::create_environment_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct CreateCliTokenOutput  {
    /// <p>An Airflow CLI login token.</p>
    #[doc(hidden)]
    pub cli_token: std::option::Option<std::string::String>,
    /// <p>The Airflow web server hostname for the environment.</p>
    #[doc(hidden)]
    pub web_server_hostname: std::option::Option<std::string::String>,
}
impl CreateCliTokenOutput {
    /// <p>An Airflow CLI login token.</p>
    pub fn cli_token(&self) -> std::option::Option<& str> {
        self.cli_token.as_deref()
    }
    /// <p>The Airflow web server hostname for the environment.</p>
    pub fn web_server_hostname(&self) -> std::option::Option<& str> {
        self.web_server_hostname.as_deref()
    }
}
impl  std::fmt::Debug for CreateCliTokenOutput  {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("CreateCliTokenOutput");
        formatter.field("cli_token", &"*** Sensitive Data Redacted ***");
        formatter.field("web_server_hostname", &self.web_server_hostname);
        formatter.finish()
    }
}
/// See [`CreateCliTokenOutput`](crate::output::CreateCliTokenOutput).
pub mod create_cli_token_output {
    
    /// A builder for [`CreateCliTokenOutput`](crate::output::CreateCliTokenOutput).
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default)]
    pub struct Builder {
        pub(crate) cli_token: std::option::Option<std::string::String>,
        pub(crate) web_server_hostname: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>An Airflow CLI login token.</p>
        pub fn cli_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.cli_token = Some(input.into());
            self
        }
        /// <p>An Airflow CLI login token.</p>
        pub fn set_cli_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.cli_token = input; self
        }
        /// <p>The Airflow web server hostname for the environment.</p>
        pub fn web_server_hostname(mut self, input: impl Into<std::string::String>) -> Self {
            self.web_server_hostname = Some(input.into());
            self
        }
        /// <p>The Airflow web server hostname for the environment.</p>
        pub fn set_web_server_hostname(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.web_server_hostname = input; self
        }
        /// Consumes the builder and constructs a [`CreateCliTokenOutput`](crate::output::CreateCliTokenOutput).
        pub fn build(self) -> crate::output::CreateCliTokenOutput {
            crate::output::CreateCliTokenOutput {
                cli_token: self.cli_token
                ,
                web_server_hostname: self.web_server_hostname
                ,
            }
        }
    }
    impl std::fmt::Debug for Builder {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let mut formatter = f.debug_struct("Builder");
            formatter.field("cli_token", &"*** Sensitive Data Redacted ***");
            formatter.field("web_server_hostname", &self.web_server_hostname);
            formatter.finish()
        }
    }
    
    
}
impl CreateCliTokenOutput {
    /// Creates a new builder-style object to manufacture [`CreateCliTokenOutput`](crate::output::CreateCliTokenOutput).
    pub fn builder() -> crate::output::create_cli_token_output::Builder {
        crate::output::create_cli_token_output::Builder::default()
    }
}

