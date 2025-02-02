// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`AssociateAppBlockBuilderAppBlock`](crate::operation::associate_app_block_builder_app_block::builders::AssociateAppBlockBuilderAppBlockFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`app_block_arn(impl Into<String>)`](crate::operation::associate_app_block_builder_app_block::builders::AssociateAppBlockBuilderAppBlockFluentBuilder::app_block_arn) / [`set_app_block_arn(Option<String>)`](crate::operation::associate_app_block_builder_app_block::builders::AssociateAppBlockBuilderAppBlockFluentBuilder::set_app_block_arn): <p>The ARN of the app block.</p>
    ///   - [`app_block_builder_name(impl Into<String>)`](crate::operation::associate_app_block_builder_app_block::builders::AssociateAppBlockBuilderAppBlockFluentBuilder::app_block_builder_name) / [`set_app_block_builder_name(Option<String>)`](crate::operation::associate_app_block_builder_app_block::builders::AssociateAppBlockBuilderAppBlockFluentBuilder::set_app_block_builder_name): <p>The name of the app block builder.</p>
    /// - On success, responds with [`AssociateAppBlockBuilderAppBlockOutput`](crate::operation::associate_app_block_builder_app_block::AssociateAppBlockBuilderAppBlockOutput) with field(s):
    ///   - [`app_block_builder_app_block_association(Option<AppBlockBuilderAppBlockAssociation>)`](crate::operation::associate_app_block_builder_app_block::AssociateAppBlockBuilderAppBlockOutput::app_block_builder_app_block_association): <p>The list of app block builders associated with app blocks.</p>
    /// - On failure, responds with [`SdkError<AssociateAppBlockBuilderAppBlockError>`](crate::operation::associate_app_block_builder_app_block::AssociateAppBlockBuilderAppBlockError)
    pub fn associate_app_block_builder_app_block(
        &self,
    ) -> crate::operation::associate_app_block_builder_app_block::builders::AssociateAppBlockBuilderAppBlockFluentBuilder {
        crate::operation::associate_app_block_builder_app_block::builders::AssociateAppBlockBuilderAppBlockFluentBuilder::new(self.handle.clone())
    }
}
