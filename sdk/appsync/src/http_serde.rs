// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn deser_payload_get_introspection_schema_get_introspection_schema_output_schema(body: &[u8]) -> std::result::Result<std::option::Option<aws_smithy_types::Blob>, crate::error::GetIntrospectionSchemaError> {
    (!body.is_empty()).then(||{
        Ok(aws_smithy_types::Blob::new(body))
    }).transpose()
}

