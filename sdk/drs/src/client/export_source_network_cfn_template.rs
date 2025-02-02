// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ExportSourceNetworkCfnTemplate`](crate::operation::export_source_network_cfn_template::builders::ExportSourceNetworkCfnTemplateFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`source_network_id(impl Into<String>)`](crate::operation::export_source_network_cfn_template::builders::ExportSourceNetworkCfnTemplateFluentBuilder::source_network_id) / [`set_source_network_id(Option<String>)`](crate::operation::export_source_network_cfn_template::builders::ExportSourceNetworkCfnTemplateFluentBuilder::set_source_network_id): <p>The Source Network ID to export its CloudFormation template to an S3 bucket.</p>
    /// - On success, responds with [`ExportSourceNetworkCfnTemplateOutput`](crate::operation::export_source_network_cfn_template::ExportSourceNetworkCfnTemplateOutput) with field(s):
    ///   - [`s3_destination_url(Option<String>)`](crate::operation::export_source_network_cfn_template::ExportSourceNetworkCfnTemplateOutput::s3_destination_url): <p>S3 bucket URL where the Source Network CloudFormation template was exported to.</p>
    /// - On failure, responds with [`SdkError<ExportSourceNetworkCfnTemplateError>`](crate::operation::export_source_network_cfn_template::ExportSourceNetworkCfnTemplateError)
    pub fn export_source_network_cfn_template(
        &self,
    ) -> crate::operation::export_source_network_cfn_template::builders::ExportSourceNetworkCfnTemplateFluentBuilder {
        crate::operation::export_source_network_cfn_template::builders::ExportSourceNetworkCfnTemplateFluentBuilder::new(self.handle.clone())
    }
}
