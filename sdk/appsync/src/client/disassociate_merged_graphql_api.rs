// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DisassociateMergedGraphqlApi`](crate::operation::disassociate_merged_graphql_api::builders::DisassociateMergedGraphqlApiFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`source_api_identifier(impl Into<String>)`](crate::operation::disassociate_merged_graphql_api::builders::DisassociateMergedGraphqlApiFluentBuilder::source_api_identifier) / [`set_source_api_identifier(Option<String>)`](crate::operation::disassociate_merged_graphql_api::builders::DisassociateMergedGraphqlApiFluentBuilder::set_source_api_identifier): <p>The identifier of the AppSync Source API. This is generated by the AppSync service. In most cases, source APIs (especially in your account) only require the API ID value or ARN of the source API. However, source APIs from other accounts (cross-account use cases) strictly require the full resource ARN of the source API.</p>
    ///   - [`association_id(impl Into<String>)`](crate::operation::disassociate_merged_graphql_api::builders::DisassociateMergedGraphqlApiFluentBuilder::association_id) / [`set_association_id(Option<String>)`](crate::operation::disassociate_merged_graphql_api::builders::DisassociateMergedGraphqlApiFluentBuilder::set_association_id): <p>The ID generated by the AppSync service for the source API association.</p>
    /// - On success, responds with [`DisassociateMergedGraphqlApiOutput`](crate::operation::disassociate_merged_graphql_api::DisassociateMergedGraphqlApiOutput) with field(s):
    ///   - [`source_api_association_status(Option<SourceApiAssociationStatus>)`](crate::operation::disassociate_merged_graphql_api::DisassociateMergedGraphqlApiOutput::source_api_association_status): <p>The state of the source API association.</p>
    /// - On failure, responds with [`SdkError<DisassociateMergedGraphqlApiError>`](crate::operation::disassociate_merged_graphql_api::DisassociateMergedGraphqlApiError)
    pub fn disassociate_merged_graphql_api(
        &self,
    ) -> crate::operation::disassociate_merged_graphql_api::builders::DisassociateMergedGraphqlApiFluentBuilder {
        crate::operation::disassociate_merged_graphql_api::builders::DisassociateMergedGraphqlApiFluentBuilder::new(self.handle.clone())
    }
}
