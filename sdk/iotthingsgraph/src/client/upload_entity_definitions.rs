// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UploadEntityDefinitions`](crate::operation::upload_entity_definitions::builders::UploadEntityDefinitionsFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`document(DefinitionDocument)`](crate::operation::upload_entity_definitions::builders::UploadEntityDefinitionsFluentBuilder::document) / [`set_document(Option<DefinitionDocument>)`](crate::operation::upload_entity_definitions::builders::UploadEntityDefinitionsFluentBuilder::set_document): <p>The <code>DefinitionDocument</code> that defines the updated entities.</p>
    ///   - [`sync_with_public_namespace(bool)`](crate::operation::upload_entity_definitions::builders::UploadEntityDefinitionsFluentBuilder::sync_with_public_namespace) / [`set_sync_with_public_namespace(bool)`](crate::operation::upload_entity_definitions::builders::UploadEntityDefinitionsFluentBuilder::set_sync_with_public_namespace): <p>A Boolean that specifies whether to synchronize with the latest version of the public namespace. If set to <code>true</code>, the upload will create a new namespace version.</p>
    ///   - [`deprecate_existing_entities(bool)`](crate::operation::upload_entity_definitions::builders::UploadEntityDefinitionsFluentBuilder::deprecate_existing_entities) / [`set_deprecate_existing_entities(bool)`](crate::operation::upload_entity_definitions::builders::UploadEntityDefinitionsFluentBuilder::set_deprecate_existing_entities): <p>A Boolean that specifies whether to deprecate all entities in the latest version before uploading the new <code>DefinitionDocument</code>. If set to <code>true</code>, the upload will create a new namespace version.</p>
    /// - On success, responds with [`UploadEntityDefinitionsOutput`](crate::operation::upload_entity_definitions::UploadEntityDefinitionsOutput) with field(s):
    ///   - [`upload_id(Option<String>)`](crate::operation::upload_entity_definitions::UploadEntityDefinitionsOutput::upload_id): <p>The ID that specifies the upload action. You can use this to track the status of the upload.</p>
    /// - On failure, responds with [`SdkError<UploadEntityDefinitionsError>`](crate::operation::upload_entity_definitions::UploadEntityDefinitionsError)
    #[deprecated(note = "since: 2022-08-30")]
    pub fn upload_entity_definitions(&self) -> crate::operation::upload_entity_definitions::builders::UploadEntityDefinitionsFluentBuilder {
        crate::operation::upload_entity_definitions::builders::UploadEntityDefinitionsFluentBuilder::new(self.handle.clone())
    }
}
