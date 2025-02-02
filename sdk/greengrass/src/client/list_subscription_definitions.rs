// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListSubscriptionDefinitions`](crate::operation::list_subscription_definitions::builders::ListSubscriptionDefinitionsFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`max_results(impl Into<String>)`](crate::operation::list_subscription_definitions::builders::ListSubscriptionDefinitionsFluentBuilder::max_results) / [`set_max_results(Option<String>)`](crate::operation::list_subscription_definitions::builders::ListSubscriptionDefinitionsFluentBuilder::set_max_results): The maximum number of results to be returned per request.
    ///   - [`next_token(impl Into<String>)`](crate::operation::list_subscription_definitions::builders::ListSubscriptionDefinitionsFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_subscription_definitions::builders::ListSubscriptionDefinitionsFluentBuilder::set_next_token): The token for the next set of results, or ''null'' if there are no additional results.
    /// - On success, responds with [`ListSubscriptionDefinitionsOutput`](crate::operation::list_subscription_definitions::ListSubscriptionDefinitionsOutput) with field(s):
    ///   - [`definitions(Option<Vec<DefinitionInformation>>)`](crate::operation::list_subscription_definitions::ListSubscriptionDefinitionsOutput::definitions): Information about a definition.
    ///   - [`next_token(Option<String>)`](crate::operation::list_subscription_definitions::ListSubscriptionDefinitionsOutput::next_token): The token for the next set of results, or ''null'' if there are no additional results.
    /// - On failure, responds with [`SdkError<ListSubscriptionDefinitionsError>`](crate::operation::list_subscription_definitions::ListSubscriptionDefinitionsError)
    pub fn list_subscription_definitions(
        &self,
    ) -> crate::operation::list_subscription_definitions::builders::ListSubscriptionDefinitionsFluentBuilder {
        crate::operation::list_subscription_definitions::builders::ListSubscriptionDefinitionsFluentBuilder::new(self.handle.clone())
    }
}
