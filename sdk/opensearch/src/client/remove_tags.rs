// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`RemoveTags`](crate::operation::remove_tags::builders::RemoveTagsFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`arn(impl Into<String>)`](crate::operation::remove_tags::builders::RemoveTagsFluentBuilder::arn) / [`set_arn(Option<String>)`](crate::operation::remove_tags::builders::RemoveTagsFluentBuilder::set_arn): <p>The Amazon Resource Name (ARN) of the domain from which you want to delete the specified tags.</p>
    ///   - [`tag_keys(impl Into<String>)`](crate::operation::remove_tags::builders::RemoveTagsFluentBuilder::tag_keys) / [`set_tag_keys(Option<Vec<String>>)`](crate::operation::remove_tags::builders::RemoveTagsFluentBuilder::set_tag_keys): <p>The list of tag keys to remove from the domain.</p>
    /// - On success, responds with [`RemoveTagsOutput`](crate::operation::remove_tags::RemoveTagsOutput)
    /// - On failure, responds with [`SdkError<RemoveTagsError>`](crate::operation::remove_tags::RemoveTagsError)
    pub fn remove_tags(&self) -> crate::operation::remove_tags::builders::RemoveTagsFluentBuilder {
        crate::operation::remove_tags::builders::RemoveTagsFluentBuilder::new(self.handle.clone())
    }
}
