// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_add_profile_permission_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::AddProfilePermissionInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.action {
        object.key("action").string(var_1.as_str());
    }
    if let Some(var_2) = &input.principal {
        object.key("principal").string(var_2.as_str());
    }
    if let Some(var_3) = &input.profile_version {
        object.key("profileVersion").string(var_3.as_str());
    }
    if let Some(var_4) = &input.revision_id {
        object.key("revisionId").string(var_4.as_str());
    }
    if let Some(var_5) = &input.statement_id {
        object.key("statementId").string(var_5.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_put_signing_profile_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::PutSigningProfileInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_6) = &input.overrides {
        #[allow(unused_mut)]
        let mut object_7 = object.key("overrides").start_object();
        crate::json_ser::serialize_structure_crate_model_signing_platform_overrides(&mut object_7, var_6)?;
        object_7.finish();
    }
    if let Some(var_8) = &input.platform_id {
        object.key("platformId").string(var_8.as_str());
    }
    if let Some(var_9) = &input.signature_validity_period {
        #[allow(unused_mut)]
        let mut object_10 = object.key("signatureValidityPeriod").start_object();
        crate::json_ser::serialize_structure_crate_model_signature_validity_period(&mut object_10, var_9)?;
        object_10.finish();
    }
    if let Some(var_11) = &input.signing_material {
        #[allow(unused_mut)]
        let mut object_12 = object.key("signingMaterial").start_object();
        crate::json_ser::serialize_structure_crate_model_signing_material(&mut object_12, var_11)?;
        object_12.finish();
    }
    if let Some(var_13) = &input.signing_parameters {
        #[allow(unused_mut)]
        let mut object_14 = object.key("signingParameters").start_object();
        for (key_15, value_16) in var_13 {
             {
                object_14.key(key_15.as_str()).string(value_16.as_str());
            }
        }
        object_14.finish();
    }
    if let Some(var_17) = &input.tags {
        #[allow(unused_mut)]
        let mut object_18 = object.key("tags").start_object();
        for (key_19, value_20) in var_17 {
             {
                object_18.key(key_19.as_str()).string(value_20.as_str());
            }
        }
        object_18.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_revoke_signature_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::RevokeSignatureInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_21) = &input.job_owner {
        object.key("jobOwner").string(var_21.as_str());
    }
    if let Some(var_22) = &input.reason {
        object.key("reason").string(var_22.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_revoke_signing_profile_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::RevokeSigningProfileInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_23) = &input.effective_time {
        object.key("effectiveTime").date_time(var_23, aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    if let Some(var_24) = &input.profile_version {
        object.key("profileVersion").string(var_24.as_str());
    }
    if let Some(var_25) = &input.reason {
        object.key("reason").string(var_25.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_start_signing_job_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::StartSigningJobInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_26) = &input.client_request_token {
        object.key("clientRequestToken").string(var_26.as_str());
    }
    if let Some(var_27) = &input.destination {
        #[allow(unused_mut)]
        let mut object_28 = object.key("destination").start_object();
        crate::json_ser::serialize_structure_crate_model_destination(&mut object_28, var_27)?;
        object_28.finish();
    }
    if let Some(var_29) = &input.profile_name {
        object.key("profileName").string(var_29.as_str());
    }
    if let Some(var_30) = &input.profile_owner {
        object.key("profileOwner").string(var_30.as_str());
    }
    if let Some(var_31) = &input.source {
        #[allow(unused_mut)]
        let mut object_32 = object.key("source").start_object();
        crate::json_ser::serialize_structure_crate_model_source(&mut object_32, var_31)?;
        object_32.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_tag_resource_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::TagResourceInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_33) = &input.tags {
        #[allow(unused_mut)]
        let mut object_34 = object.key("tags").start_object();
        for (key_35, value_36) in var_33 {
             {
                object_34.key(key_35.as_str()).string(value_36.as_str());
            }
        }
        object_34.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_signing_platform_overrides(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::SigningPlatformOverrides) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_37) = &input.signing_configuration {
        #[allow(unused_mut)]
        let mut object_38 = object.key("signingConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_signing_configuration_overrides(&mut object_38, var_37)?;
        object_38.finish();
    }
    if let Some(var_39) = &input.signing_image_format {
        object.key("signingImageFormat").string(var_39.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_signature_validity_period(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::SignatureValidityPeriod) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if input.value != 0 {
        object.key("value").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((input.value).into()));
    }
    if let Some(var_40) = &input.r#type {
        object.key("type").string(var_40.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_signing_material(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::SigningMaterial) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_41) = &input.certificate_arn {
        object.key("certificateArn").string(var_41.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_destination(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::Destination) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_42) = &input.s3 {
        #[allow(unused_mut)]
        let mut object_43 = object.key("s3").start_object();
        crate::json_ser::serialize_structure_crate_model_s3_destination(&mut object_43, var_42)?;
        object_43.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_source(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::Source) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_44) = &input.s3 {
        #[allow(unused_mut)]
        let mut object_45 = object.key("s3").start_object();
        crate::json_ser::serialize_structure_crate_model_s3_source(&mut object_45, var_44)?;
        object_45.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_signing_configuration_overrides(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::SigningConfigurationOverrides) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_46) = &input.encryption_algorithm {
        object.key("encryptionAlgorithm").string(var_46.as_str());
    }
    if let Some(var_47) = &input.hash_algorithm {
        object.key("hashAlgorithm").string(var_47.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_s3_destination(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::S3Destination) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_48) = &input.bucket_name {
        object.key("bucketName").string(var_48.as_str());
    }
    if let Some(var_49) = &input.prefix {
        object.key("prefix").string(var_49.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_s3_source(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::S3Source) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_50) = &input.bucket_name {
        object.key("bucketName").string(var_50.as_str());
    }
    if let Some(var_51) = &input.key {
        object.key("key").string(var_51.as_str());
    }
    if let Some(var_52) = &input.version {
        object.key("version").string(var_52.as_str());
    }
    Ok(())
}

