// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_custom_db_engine_version::_create_custom_db_engine_version_output::CreateCustomDbEngineVersionOutputBuilder;

pub use crate::operation::create_custom_db_engine_version::_create_custom_db_engine_version_input::CreateCustomDbEngineVersionInputBuilder;

impl CreateCustomDbEngineVersionInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::create_custom_db_engine_version::CreateCustomDbEngineVersionOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_custom_db_engine_version::CreateCustomDBEngineVersionError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.create_custom_db_engine_version();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CreateCustomDBEngineVersion`.
///
/// <p>Creates a custom DB engine version (CEV).</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateCustomDBEngineVersionFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_custom_db_engine_version::builders::CreateCustomDbEngineVersionInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl CreateCustomDBEngineVersionFluentBuilder {
    /// Creates a new `CreateCustomDBEngineVersion`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the CreateCustomDBEngineVersion as a reference.
    pub fn as_input(&self) -> &crate::operation::create_custom_db_engine_version::builders::CreateCustomDbEngineVersionInputBuilder {
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
        crate::operation::create_custom_db_engine_version::CreateCustomDbEngineVersionOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_custom_db_engine_version::CreateCustomDBEngineVersionError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self.inner.build().map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::create_custom_db_engine_version::CreateCustomDBEngineVersion::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::create_custom_db_engine_version::CreateCustomDBEngineVersion::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent.
    // TODO(enableNewSmithyRuntimeCleanup): Remove `async` and `Result` once we switch to orchestrator
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::orchestrator::CustomizableOperation<
            crate::operation::create_custom_db_engine_version::CreateCustomDbEngineVersionOutput,
            crate::operation::create_custom_db_engine_version::CreateCustomDBEngineVersionError,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::create_custom_db_engine_version::CreateCustomDBEngineVersionError>,
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
    /// <p>The database engine to use for your custom engine version (CEV). The only supported value is <code>custom-oracle-ee</code>.</p>
    pub fn engine(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.engine(input.into());
        self
    }
    /// <p>The database engine to use for your custom engine version (CEV). The only supported value is <code>custom-oracle-ee</code>.</p>
    pub fn set_engine(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_engine(input);
        self
    }
    /// <p>The database engine to use for your custom engine version (CEV). The only supported value is <code>custom-oracle-ee</code>.</p>
    pub fn get_engine(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_engine()
    }
    /// <p>The name of your CEV. The name format is 19.<i>customized_string</i>. For example, a valid CEV name is <code>19.my_cev1</code>. This setting is required for RDS Custom for Oracle, but optional for Amazon RDS. The combination of <code>Engine</code> and <code>EngineVersion</code> is unique per customer per Region.</p>
    pub fn engine_version(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.engine_version(input.into());
        self
    }
    /// <p>The name of your CEV. The name format is 19.<i>customized_string</i>. For example, a valid CEV name is <code>19.my_cev1</code>. This setting is required for RDS Custom for Oracle, but optional for Amazon RDS. The combination of <code>Engine</code> and <code>EngineVersion</code> is unique per customer per Region.</p>
    pub fn set_engine_version(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_engine_version(input);
        self
    }
    /// <p>The name of your CEV. The name format is 19.<i>customized_string</i>. For example, a valid CEV name is <code>19.my_cev1</code>. This setting is required for RDS Custom for Oracle, but optional for Amazon RDS. The combination of <code>Engine</code> and <code>EngineVersion</code> is unique per customer per Region.</p>
    pub fn get_engine_version(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_engine_version()
    }
    /// <p>The name of an Amazon S3 bucket that contains database installation files for your CEV. For example, a valid bucket name is <code>my-custom-installation-files</code>.</p>
    pub fn database_installation_files_s3_bucket_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.database_installation_files_s3_bucket_name(input.into());
        self
    }
    /// <p>The name of an Amazon S3 bucket that contains database installation files for your CEV. For example, a valid bucket name is <code>my-custom-installation-files</code>.</p>
    pub fn set_database_installation_files_s3_bucket_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_database_installation_files_s3_bucket_name(input);
        self
    }
    /// <p>The name of an Amazon S3 bucket that contains database installation files for your CEV. For example, a valid bucket name is <code>my-custom-installation-files</code>.</p>
    pub fn get_database_installation_files_s3_bucket_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_database_installation_files_s3_bucket_name()
    }
    /// <p>The Amazon S3 directory that contains the database installation files for your CEV. For example, a valid bucket name is <code>123456789012/cev1</code>. If this setting isn't specified, no prefix is assumed.</p>
    pub fn database_installation_files_s3_prefix(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.database_installation_files_s3_prefix(input.into());
        self
    }
    /// <p>The Amazon S3 directory that contains the database installation files for your CEV. For example, a valid bucket name is <code>123456789012/cev1</code>. If this setting isn't specified, no prefix is assumed.</p>
    pub fn set_database_installation_files_s3_prefix(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_database_installation_files_s3_prefix(input);
        self
    }
    /// <p>The Amazon S3 directory that contains the database installation files for your CEV. For example, a valid bucket name is <code>123456789012/cev1</code>. If this setting isn't specified, no prefix is assumed.</p>
    pub fn get_database_installation_files_s3_prefix(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_database_installation_files_s3_prefix()
    }
    /// <p>The ID of the Amazon Machine Image (AMI). For RDS Custom for SQL Server, an AMI ID is required to create a CEV. For RDS Custom for Oracle, the default is the most recent AMI available, but you can specify an AMI ID that was used in a different Oracle CEV. Find the AMIs used by your CEVs by calling the <a href="https://docs.aws.amazon.com/AmazonRDS/latest/APIReference/API_DescribeDBEngineVersions.html">DescribeDBEngineVersions</a> operation.</p>
    pub fn image_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.image_id(input.into());
        self
    }
    /// <p>The ID of the Amazon Machine Image (AMI). For RDS Custom for SQL Server, an AMI ID is required to create a CEV. For RDS Custom for Oracle, the default is the most recent AMI available, but you can specify an AMI ID that was used in a different Oracle CEV. Find the AMIs used by your CEVs by calling the <a href="https://docs.aws.amazon.com/AmazonRDS/latest/APIReference/API_DescribeDBEngineVersions.html">DescribeDBEngineVersions</a> operation.</p>
    pub fn set_image_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_image_id(input);
        self
    }
    /// <p>The ID of the Amazon Machine Image (AMI). For RDS Custom for SQL Server, an AMI ID is required to create a CEV. For RDS Custom for Oracle, the default is the most recent AMI available, but you can specify an AMI ID that was used in a different Oracle CEV. Find the AMIs used by your CEVs by calling the <a href="https://docs.aws.amazon.com/AmazonRDS/latest/APIReference/API_DescribeDBEngineVersions.html">DescribeDBEngineVersions</a> operation.</p>
    pub fn get_image_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_image_id()
    }
    /// <p>The Amazon Web Services KMS key identifier for an encrypted CEV. A symmetric encryption KMS key is required for RDS Custom, but optional for Amazon RDS.</p>
    /// <p>If you have an existing symmetric encryption KMS key in your account, you can use it with RDS Custom. No further action is necessary. If you don't already have a symmetric encryption KMS key in your account, follow the instructions in <a href="https://docs.aws.amazon.com/kms/latest/developerguide/create-keys.html#create-symmetric-cmk"> Creating a symmetric encryption KMS key</a> in the <i>Amazon Web Services Key Management Service Developer Guide</i>.</p>
    /// <p>You can choose the same symmetric encryption key when you create a CEV and a DB instance, or choose different keys.</p>
    pub fn kms_key_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.kms_key_id(input.into());
        self
    }
    /// <p>The Amazon Web Services KMS key identifier for an encrypted CEV. A symmetric encryption KMS key is required for RDS Custom, but optional for Amazon RDS.</p>
    /// <p>If you have an existing symmetric encryption KMS key in your account, you can use it with RDS Custom. No further action is necessary. If you don't already have a symmetric encryption KMS key in your account, follow the instructions in <a href="https://docs.aws.amazon.com/kms/latest/developerguide/create-keys.html#create-symmetric-cmk"> Creating a symmetric encryption KMS key</a> in the <i>Amazon Web Services Key Management Service Developer Guide</i>.</p>
    /// <p>You can choose the same symmetric encryption key when you create a CEV and a DB instance, or choose different keys.</p>
    pub fn set_kms_key_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_kms_key_id(input);
        self
    }
    /// <p>The Amazon Web Services KMS key identifier for an encrypted CEV. A symmetric encryption KMS key is required for RDS Custom, but optional for Amazon RDS.</p>
    /// <p>If you have an existing symmetric encryption KMS key in your account, you can use it with RDS Custom. No further action is necessary. If you don't already have a symmetric encryption KMS key in your account, follow the instructions in <a href="https://docs.aws.amazon.com/kms/latest/developerguide/create-keys.html#create-symmetric-cmk"> Creating a symmetric encryption KMS key</a> in the <i>Amazon Web Services Key Management Service Developer Guide</i>.</p>
    /// <p>You can choose the same symmetric encryption key when you create a CEV and a DB instance, or choose different keys.</p>
    pub fn get_kms_key_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_kms_key_id()
    }
    /// <p>An optional description of your CEV.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.description(input.into());
        self
    }
    /// <p>An optional description of your CEV.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_description(input);
        self
    }
    /// <p>An optional description of your CEV.</p>
    pub fn get_description(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_description()
    }
    /// <p>The CEV manifest, which is a JSON document that describes the installation .zip files stored in Amazon S3. Specify the name/value pairs in a file or a quoted string. RDS Custom applies the patches in the order in which they are listed.</p>
    /// <p>The following JSON fields are valid:</p>
    /// <dl>
    /// <dt>
    /// MediaImportTemplateVersion
    /// </dt>
    /// <dd>
    /// <p>Version of the CEV manifest. The date is in the format <code>YYYY-MM-DD</code>.</p>
    /// </dd>
    /// <dt>
    /// databaseInstallationFileNames
    /// </dt>
    /// <dd>
    /// <p>Ordered list of installation files for the CEV.</p>
    /// </dd>
    /// <dt>
    /// opatchFileNames
    /// </dt>
    /// <dd>
    /// <p>Ordered list of OPatch installers used for the Oracle DB engine.</p>
    /// </dd>
    /// <dt>
    /// psuRuPatchFileNames
    /// </dt>
    /// <dd>
    /// <p>The PSU and RU patches for this CEV.</p>
    /// </dd>
    /// <dt>
    /// OtherPatchFileNames
    /// </dt>
    /// <dd>
    /// <p>The patches that are not in the list of PSU and RU patches. Amazon RDS applies these patches after applying the PSU and RU patches.</p>
    /// </dd>
    /// </dl>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/custom-cev.html#custom-cev.preparing.manifest"> Creating the CEV manifest</a> in the <i>Amazon RDS User Guide</i>.</p>
    pub fn manifest(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.manifest(input.into());
        self
    }
    /// <p>The CEV manifest, which is a JSON document that describes the installation .zip files stored in Amazon S3. Specify the name/value pairs in a file or a quoted string. RDS Custom applies the patches in the order in which they are listed.</p>
    /// <p>The following JSON fields are valid:</p>
    /// <dl>
    /// <dt>
    /// MediaImportTemplateVersion
    /// </dt>
    /// <dd>
    /// <p>Version of the CEV manifest. The date is in the format <code>YYYY-MM-DD</code>.</p>
    /// </dd>
    /// <dt>
    /// databaseInstallationFileNames
    /// </dt>
    /// <dd>
    /// <p>Ordered list of installation files for the CEV.</p>
    /// </dd>
    /// <dt>
    /// opatchFileNames
    /// </dt>
    /// <dd>
    /// <p>Ordered list of OPatch installers used for the Oracle DB engine.</p>
    /// </dd>
    /// <dt>
    /// psuRuPatchFileNames
    /// </dt>
    /// <dd>
    /// <p>The PSU and RU patches for this CEV.</p>
    /// </dd>
    /// <dt>
    /// OtherPatchFileNames
    /// </dt>
    /// <dd>
    /// <p>The patches that are not in the list of PSU and RU patches. Amazon RDS applies these patches after applying the PSU and RU patches.</p>
    /// </dd>
    /// </dl>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/custom-cev.html#custom-cev.preparing.manifest"> Creating the CEV manifest</a> in the <i>Amazon RDS User Guide</i>.</p>
    pub fn set_manifest(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_manifest(input);
        self
    }
    /// <p>The CEV manifest, which is a JSON document that describes the installation .zip files stored in Amazon S3. Specify the name/value pairs in a file or a quoted string. RDS Custom applies the patches in the order in which they are listed.</p>
    /// <p>The following JSON fields are valid:</p>
    /// <dl>
    /// <dt>
    /// MediaImportTemplateVersion
    /// </dt>
    /// <dd>
    /// <p>Version of the CEV manifest. The date is in the format <code>YYYY-MM-DD</code>.</p>
    /// </dd>
    /// <dt>
    /// databaseInstallationFileNames
    /// </dt>
    /// <dd>
    /// <p>Ordered list of installation files for the CEV.</p>
    /// </dd>
    /// <dt>
    /// opatchFileNames
    /// </dt>
    /// <dd>
    /// <p>Ordered list of OPatch installers used for the Oracle DB engine.</p>
    /// </dd>
    /// <dt>
    /// psuRuPatchFileNames
    /// </dt>
    /// <dd>
    /// <p>The PSU and RU patches for this CEV.</p>
    /// </dd>
    /// <dt>
    /// OtherPatchFileNames
    /// </dt>
    /// <dd>
    /// <p>The patches that are not in the list of PSU and RU patches. Amazon RDS applies these patches after applying the PSU and RU patches.</p>
    /// </dd>
    /// </dl>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/custom-cev.html#custom-cev.preparing.manifest"> Creating the CEV manifest</a> in the <i>Amazon RDS User Guide</i>.</p>
    pub fn get_manifest(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_manifest()
    }
    /// Appends an item to `Tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>A list of tags. For more information, see <a href="https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/USER_Tagging.html">Tagging Amazon RDS Resources</a> in the <i>Amazon RDS User Guide.</i> </p>
    pub fn tags(mut self, input: crate::types::Tag) -> Self {
        self.inner = self.inner.tags(input);
        self
    }
    /// <p>A list of tags. For more information, see <a href="https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/USER_Tagging.html">Tagging Amazon RDS Resources</a> in the <i>Amazon RDS User Guide.</i> </p>
    pub fn set_tags(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>) -> Self {
        self.inner = self.inner.set_tags(input);
        self
    }
    /// <p>A list of tags. For more information, see <a href="https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/USER_Tagging.html">Tagging Amazon RDS Resources</a> in the <i>Amazon RDS User Guide.</i> </p>
    pub fn get_tags(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Tag>> {
        self.inner.get_tags()
    }
}
