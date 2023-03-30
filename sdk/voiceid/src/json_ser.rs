// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_create_domain_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreateDomainInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.name {
        object.key("Name").string(var_1.as_str());
    }
    if let Some(var_2) = &input.description {
        object.key("Description").string(var_2.as_str());
    }
    if let Some(var_3) = &input.server_side_encryption_configuration {
        #[allow(unused_mut)]
        let mut object_4 = object.key("ServerSideEncryptionConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_server_side_encryption_configuration(&mut object_4, var_3)?;
        object_4.finish();
    }
    if let Some(var_5) = &input.client_token {
        object.key("ClientToken").string(var_5.as_str());
    }
    if let Some(var_6) = &input.tags {
        let mut array_7 = object.key("Tags").start_array();
        for item_8 in var_6 {
             {
                #[allow(unused_mut)]
                let mut object_9 = array_7.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag(&mut object_9, item_8)?;
                object_9.finish();
            }
        }
        array_7.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_delete_domain_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DeleteDomainInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_10) = &input.domain_id {
        object.key("DomainId").string(var_10.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_delete_fraudster_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DeleteFraudsterInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_11) = &input.domain_id {
        object.key("DomainId").string(var_11.as_str());
    }
    if let Some(var_12) = &input.fraudster_id {
        object.key("FraudsterId").string(var_12.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_delete_speaker_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DeleteSpeakerInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_13) = &input.domain_id {
        object.key("DomainId").string(var_13.as_str());
    }
    if let Some(var_14) = &input.speaker_id {
        object.key("SpeakerId").string(var_14.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_describe_domain_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DescribeDomainInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_15) = &input.domain_id {
        object.key("DomainId").string(var_15.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_describe_fraudster_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DescribeFraudsterInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_16) = &input.domain_id {
        object.key("DomainId").string(var_16.as_str());
    }
    if let Some(var_17) = &input.fraudster_id {
        object.key("FraudsterId").string(var_17.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_describe_fraudster_registration_job_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DescribeFraudsterRegistrationJobInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_18) = &input.domain_id {
        object.key("DomainId").string(var_18.as_str());
    }
    if let Some(var_19) = &input.job_id {
        object.key("JobId").string(var_19.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_describe_speaker_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DescribeSpeakerInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_20) = &input.domain_id {
        object.key("DomainId").string(var_20.as_str());
    }
    if let Some(var_21) = &input.speaker_id {
        object.key("SpeakerId").string(var_21.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_describe_speaker_enrollment_job_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DescribeSpeakerEnrollmentJobInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_22) = &input.domain_id {
        object.key("DomainId").string(var_22.as_str());
    }
    if let Some(var_23) = &input.job_id {
        object.key("JobId").string(var_23.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_evaluate_session_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::EvaluateSessionInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_24) = &input.domain_id {
        object.key("DomainId").string(var_24.as_str());
    }
    if let Some(var_25) = &input.session_name_or_id {
        object.key("SessionNameOrId").string(var_25.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_domains_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::ListDomainsInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_26) = &input.max_results {
        object.key("MaxResults").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_26).into()));
    }
    if let Some(var_27) = &input.next_token {
        object.key("NextToken").string(var_27.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_fraudster_registration_jobs_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::ListFraudsterRegistrationJobsInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_28) = &input.domain_id {
        object.key("DomainId").string(var_28.as_str());
    }
    if let Some(var_29) = &input.job_status {
        object.key("JobStatus").string(var_29.as_str());
    }
    if let Some(var_30) = &input.max_results {
        object.key("MaxResults").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_30).into()));
    }
    if let Some(var_31) = &input.next_token {
        object.key("NextToken").string(var_31.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_speaker_enrollment_jobs_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::ListSpeakerEnrollmentJobsInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_32) = &input.domain_id {
        object.key("DomainId").string(var_32.as_str());
    }
    if let Some(var_33) = &input.job_status {
        object.key("JobStatus").string(var_33.as_str());
    }
    if let Some(var_34) = &input.max_results {
        object.key("MaxResults").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_34).into()));
    }
    if let Some(var_35) = &input.next_token {
        object.key("NextToken").string(var_35.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_speakers_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::ListSpeakersInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_36) = &input.domain_id {
        object.key("DomainId").string(var_36.as_str());
    }
    if let Some(var_37) = &input.max_results {
        object.key("MaxResults").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_37).into()));
    }
    if let Some(var_38) = &input.next_token {
        object.key("NextToken").string(var_38.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_tags_for_resource_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::ListTagsForResourceInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_39) = &input.resource_arn {
        object.key("ResourceArn").string(var_39.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_opt_out_speaker_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::OptOutSpeakerInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_40) = &input.domain_id {
        object.key("DomainId").string(var_40.as_str());
    }
    if let Some(var_41) = &input.speaker_id {
        object.key("SpeakerId").string(var_41.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_start_fraudster_registration_job_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::StartFraudsterRegistrationJobInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_42) = &input.client_token {
        object.key("ClientToken").string(var_42.as_str());
    }
    if let Some(var_43) = &input.job_name {
        object.key("JobName").string(var_43.as_str());
    }
    if let Some(var_44) = &input.domain_id {
        object.key("DomainId").string(var_44.as_str());
    }
    if let Some(var_45) = &input.data_access_role_arn {
        object.key("DataAccessRoleArn").string(var_45.as_str());
    }
    if let Some(var_46) = &input.registration_config {
        #[allow(unused_mut)]
        let mut object_47 = object.key("RegistrationConfig").start_object();
        crate::json_ser::serialize_structure_crate_model_registration_config(&mut object_47, var_46)?;
        object_47.finish();
    }
    if let Some(var_48) = &input.input_data_config {
        #[allow(unused_mut)]
        let mut object_49 = object.key("InputDataConfig").start_object();
        crate::json_ser::serialize_structure_crate_model_input_data_config(&mut object_49, var_48)?;
        object_49.finish();
    }
    if let Some(var_50) = &input.output_data_config {
        #[allow(unused_mut)]
        let mut object_51 = object.key("OutputDataConfig").start_object();
        crate::json_ser::serialize_structure_crate_model_output_data_config(&mut object_51, var_50)?;
        object_51.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_start_speaker_enrollment_job_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::StartSpeakerEnrollmentJobInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_52) = &input.client_token {
        object.key("ClientToken").string(var_52.as_str());
    }
    if let Some(var_53) = &input.job_name {
        object.key("JobName").string(var_53.as_str());
    }
    if let Some(var_54) = &input.domain_id {
        object.key("DomainId").string(var_54.as_str());
    }
    if let Some(var_55) = &input.data_access_role_arn {
        object.key("DataAccessRoleArn").string(var_55.as_str());
    }
    if let Some(var_56) = &input.enrollment_config {
        #[allow(unused_mut)]
        let mut object_57 = object.key("EnrollmentConfig").start_object();
        crate::json_ser::serialize_structure_crate_model_enrollment_config(&mut object_57, var_56)?;
        object_57.finish();
    }
    if let Some(var_58) = &input.input_data_config {
        #[allow(unused_mut)]
        let mut object_59 = object.key("InputDataConfig").start_object();
        crate::json_ser::serialize_structure_crate_model_input_data_config(&mut object_59, var_58)?;
        object_59.finish();
    }
    if let Some(var_60) = &input.output_data_config {
        #[allow(unused_mut)]
        let mut object_61 = object.key("OutputDataConfig").start_object();
        crate::json_ser::serialize_structure_crate_model_output_data_config(&mut object_61, var_60)?;
        object_61.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_tag_resource_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::TagResourceInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_62) = &input.resource_arn {
        object.key("ResourceArn").string(var_62.as_str());
    }
    if let Some(var_63) = &input.tags {
        let mut array_64 = object.key("Tags").start_array();
        for item_65 in var_63 {
             {
                #[allow(unused_mut)]
                let mut object_66 = array_64.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag(&mut object_66, item_65)?;
                object_66.finish();
            }
        }
        array_64.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_untag_resource_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::UntagResourceInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_67) = &input.resource_arn {
        object.key("ResourceArn").string(var_67.as_str());
    }
    if let Some(var_68) = &input.tag_keys {
        let mut array_69 = object.key("TagKeys").start_array();
        for item_70 in var_68 {
             {
                array_69.value().string(item_70.as_str());
            }
        }
        array_69.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_domain_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::UpdateDomainInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_71) = &input.domain_id {
        object.key("DomainId").string(var_71.as_str());
    }
    if let Some(var_72) = &input.name {
        object.key("Name").string(var_72.as_str());
    }
    if let Some(var_73) = &input.description {
        object.key("Description").string(var_73.as_str());
    }
    if let Some(var_74) = &input.server_side_encryption_configuration {
        #[allow(unused_mut)]
        let mut object_75 = object.key("ServerSideEncryptionConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_server_side_encryption_configuration(&mut object_75, var_74)?;
        object_75.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_server_side_encryption_configuration(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::ServerSideEncryptionConfiguration) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_76) = &input.kms_key_id {
        object.key("KmsKeyId").string(var_76.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_tag(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::Tag) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_77) = &input.key {
        object.key("Key").string(var_77.as_str());
    }
    if let Some(var_78) = &input.value {
        object.key("Value").string(var_78.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_registration_config(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::RegistrationConfig) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_79) = &input.duplicate_registration_action {
        object.key("DuplicateRegistrationAction").string(var_79.as_str());
    }
    if let Some(var_80) = &input.fraudster_similarity_threshold {
        object.key("FraudsterSimilarityThreshold").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_80).into()));
    }
    Ok(())
}

pub fn serialize_structure_crate_model_input_data_config(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::InputDataConfig) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_81) = &input.s3_uri {
        object.key("S3Uri").string(var_81.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_output_data_config(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::OutputDataConfig) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_82) = &input.s3_uri {
        object.key("S3Uri").string(var_82.as_str());
    }
    if let Some(var_83) = &input.kms_key_id {
        object.key("KmsKeyId").string(var_83.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_enrollment_config(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::EnrollmentConfig) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_84) = &input.existing_enrollment_action {
        object.key("ExistingEnrollmentAction").string(var_84.as_str());
    }
    if let Some(var_85) = &input.fraud_detection_config {
        #[allow(unused_mut)]
        let mut object_86 = object.key("FraudDetectionConfig").start_object();
        crate::json_ser::serialize_structure_crate_model_enrollment_job_fraud_detection_config(&mut object_86, var_85)?;
        object_86.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_enrollment_job_fraud_detection_config(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::EnrollmentJobFraudDetectionConfig) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_87) = &input.fraud_detection_action {
        object.key("FraudDetectionAction").string(var_87.as_str());
    }
    if let Some(var_88) = &input.risk_threshold {
        object.key("RiskThreshold").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_88).into()));
    }
    Ok(())
}

