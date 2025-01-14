// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_import_tr34_key_block(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::ImportTr34KeyBlock,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.certificate_authority_public_key_identifier {
        object.key("CertificateAuthorityPublicKeyIdentifier").string(var_1.as_str());
    }
    if let Some(var_2) = &input.signing_key_certificate {
        object.key("SigningKeyCertificate").string(var_2.as_str());
    }
    if let Some(var_3) = &input.import_token {
        object.key("ImportToken").string(var_3.as_str());
    }
    if let Some(var_4) = &input.wrapped_key_block {
        object.key("WrappedKeyBlock").string(var_4.as_str());
    }
    if let Some(var_5) = &input.key_block_format {
        object.key("KeyBlockFormat").string(var_5.as_str());
    }
    if let Some(var_6) = &input.random_nonce {
        object.key("RandomNonce").string(var_6.as_str());
    }
    Ok(())
}
