// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_payload_complete_multipart_upload_input(payload: &std::option::Option<crate::model::CompletedMultipartUpload>) -> Result<std::vec::Vec<u8>, aws_smithy_http::operation::error::BuildError> {
    let payload = match payload.as_ref() {
                                Some(t) => t,
                                None => return Ok(
        crate::operation_ser::rest_xml_unset_payload()
    )};
    Ok(
        crate::xml_ser::serialize_member_com_amazonaws_s3_synthetic_complete_multipart_upload_input_multipart_upload(payload)?
    )
}

pub fn serialize_payload_create_bucket_input(payload: &std::option::Option<crate::model::CreateBucketConfiguration>) -> Result<std::vec::Vec<u8>, aws_smithy_http::operation::error::BuildError> {
    let payload = match payload.as_ref() {
                                Some(t) => t,
                                None => return Ok(
        crate::operation_ser::rest_xml_unset_payload()
    )};
    Ok(
        crate::xml_ser::serialize_member_com_amazonaws_s3_synthetic_create_bucket_input_create_bucket_configuration(payload)?
    )
}

pub fn serialize_payload_delete_objects_input(payload: &std::option::Option<crate::model::Delete>) -> Result<std::vec::Vec<u8>, aws_smithy_http::operation::error::BuildError> {
    let payload = match payload.as_ref() {
                                Some(t) => t,
                                None => return Ok(
        crate::operation_ser::rest_xml_unset_payload()
    )};
    Ok(
        crate::xml_ser::serialize_member_com_amazonaws_s3_synthetic_delete_objects_input_delete(payload)?
    )
}

pub fn serialize_payload_put_bucket_accelerate_configuration_input(payload: &std::option::Option<crate::model::AccelerateConfiguration>) -> Result<std::vec::Vec<u8>, aws_smithy_http::operation::error::BuildError> {
    let payload = match payload.as_ref() {
                                Some(t) => t,
                                None => return Ok(
        crate::operation_ser::rest_xml_unset_payload()
    )};
    Ok(
        crate::xml_ser::serialize_member_com_amazonaws_s3_synthetic_put_bucket_accelerate_configuration_input_accelerate_configuration(payload)?
    )
}

pub fn serialize_payload_put_bucket_acl_input(payload: &std::option::Option<crate::model::AccessControlPolicy>) -> Result<std::vec::Vec<u8>, aws_smithy_http::operation::error::BuildError> {
    let payload = match payload.as_ref() {
                                Some(t) => t,
                                None => return Ok(
        crate::operation_ser::rest_xml_unset_payload()
    )};
    Ok(
        crate::xml_ser::serialize_member_com_amazonaws_s3_synthetic_put_bucket_acl_input_access_control_policy(payload)?
    )
}

pub fn serialize_payload_put_bucket_analytics_configuration_input(payload: &std::option::Option<crate::model::AnalyticsConfiguration>) -> Result<std::vec::Vec<u8>, aws_smithy_http::operation::error::BuildError> {
    let payload = match payload.as_ref() {
                                Some(t) => t,
                                None => return Ok(
        crate::operation_ser::rest_xml_unset_payload()
    )};
    Ok(
        crate::xml_ser::serialize_member_com_amazonaws_s3_synthetic_put_bucket_analytics_configuration_input_analytics_configuration(payload)?
    )
}

pub fn serialize_payload_put_bucket_cors_input(payload: &std::option::Option<crate::model::CorsConfiguration>) -> Result<std::vec::Vec<u8>, aws_smithy_http::operation::error::BuildError> {
    let payload = match payload.as_ref() {
                                Some(t) => t,
                                None => return Ok(
        crate::operation_ser::rest_xml_unset_payload()
    )};
    Ok(
        crate::xml_ser::serialize_member_com_amazonaws_s3_synthetic_put_bucket_cors_input_cors_configuration(payload)?
    )
}

pub fn serialize_payload_put_bucket_encryption_input(payload: &std::option::Option<crate::model::ServerSideEncryptionConfiguration>) -> Result<std::vec::Vec<u8>, aws_smithy_http::operation::error::BuildError> {
    let payload = match payload.as_ref() {
                                Some(t) => t,
                                None => return Ok(
        crate::operation_ser::rest_xml_unset_payload()
    )};
    Ok(
        crate::xml_ser::serialize_member_com_amazonaws_s3_synthetic_put_bucket_encryption_input_server_side_encryption_configuration(payload)?
    )
}

pub fn serialize_payload_put_bucket_intelligent_tiering_configuration_input(payload: &std::option::Option<crate::model::IntelligentTieringConfiguration>) -> Result<std::vec::Vec<u8>, aws_smithy_http::operation::error::BuildError> {
    let payload = match payload.as_ref() {
                                Some(t) => t,
                                None => return Ok(
        crate::operation_ser::rest_xml_unset_payload()
    )};
    Ok(
        crate::xml_ser::serialize_member_com_amazonaws_s3_synthetic_put_bucket_intelligent_tiering_configuration_input_intelligent_tiering_configuration(payload)?
    )
}

pub fn serialize_payload_put_bucket_inventory_configuration_input(payload: &std::option::Option<crate::model::InventoryConfiguration>) -> Result<std::vec::Vec<u8>, aws_smithy_http::operation::error::BuildError> {
    let payload = match payload.as_ref() {
                                Some(t) => t,
                                None => return Ok(
        crate::operation_ser::rest_xml_unset_payload()
    )};
    Ok(
        crate::xml_ser::serialize_member_com_amazonaws_s3_synthetic_put_bucket_inventory_configuration_input_inventory_configuration(payload)?
    )
}

pub fn serialize_payload_put_bucket_lifecycle_configuration_input(payload: &std::option::Option<crate::model::BucketLifecycleConfiguration>) -> Result<std::vec::Vec<u8>, aws_smithy_http::operation::error::BuildError> {
    let payload = match payload.as_ref() {
                                Some(t) => t,
                                None => return Ok(
        crate::operation_ser::rest_xml_unset_payload()
    )};
    Ok(
        crate::xml_ser::serialize_member_com_amazonaws_s3_synthetic_put_bucket_lifecycle_configuration_input_lifecycle_configuration(payload)?
    )
}

pub fn serialize_payload_put_bucket_logging_input(payload: &std::option::Option<crate::model::BucketLoggingStatus>) -> Result<std::vec::Vec<u8>, aws_smithy_http::operation::error::BuildError> {
    let payload = match payload.as_ref() {
                                Some(t) => t,
                                None => return Ok(
        crate::operation_ser::rest_xml_unset_payload()
    )};
    Ok(
        crate::xml_ser::serialize_member_com_amazonaws_s3_synthetic_put_bucket_logging_input_bucket_logging_status(payload)?
    )
}

pub fn serialize_payload_put_bucket_metrics_configuration_input(payload: &std::option::Option<crate::model::MetricsConfiguration>) -> Result<std::vec::Vec<u8>, aws_smithy_http::operation::error::BuildError> {
    let payload = match payload.as_ref() {
                                Some(t) => t,
                                None => return Ok(
        crate::operation_ser::rest_xml_unset_payload()
    )};
    Ok(
        crate::xml_ser::serialize_member_com_amazonaws_s3_synthetic_put_bucket_metrics_configuration_input_metrics_configuration(payload)?
    )
}

pub fn serialize_payload_put_bucket_notification_configuration_input(payload: &std::option::Option<crate::model::NotificationConfiguration>) -> Result<std::vec::Vec<u8>, aws_smithy_http::operation::error::BuildError> {
    let payload = match payload.as_ref() {
                                Some(t) => t,
                                None => return Ok(
        crate::operation_ser::rest_xml_unset_payload()
    )};
    Ok(
        crate::xml_ser::serialize_member_com_amazonaws_s3_synthetic_put_bucket_notification_configuration_input_notification_configuration(payload)?
    )
}

pub fn serialize_payload_put_bucket_ownership_controls_input(payload: &std::option::Option<crate::model::OwnershipControls>) -> Result<std::vec::Vec<u8>, aws_smithy_http::operation::error::BuildError> {
    let payload = match payload.as_ref() {
                                Some(t) => t,
                                None => return Ok(
        crate::operation_ser::rest_xml_unset_payload()
    )};
    Ok(
        crate::xml_ser::serialize_member_com_amazonaws_s3_synthetic_put_bucket_ownership_controls_input_ownership_controls(payload)?
    )
}

pub fn serialize_payload_put_bucket_policy_input(payload: std::option::Option<std::string::String>) -> Result<std::vec::Vec<u8>, aws_smithy_http::operation::error::BuildError> {
    let payload = match payload {
                                Some(t) => t,
                                None => return Ok(
        Vec::new()
    )};
    Ok(
        payload.into_bytes()
    )
}

pub fn serialize_payload_put_bucket_replication_input(payload: &std::option::Option<crate::model::ReplicationConfiguration>) -> Result<std::vec::Vec<u8>, aws_smithy_http::operation::error::BuildError> {
    let payload = match payload.as_ref() {
                                Some(t) => t,
                                None => return Ok(
        crate::operation_ser::rest_xml_unset_payload()
    )};
    Ok(
        crate::xml_ser::serialize_member_com_amazonaws_s3_synthetic_put_bucket_replication_input_replication_configuration(payload)?
    )
}

pub fn serialize_payload_put_bucket_request_payment_input(payload: &std::option::Option<crate::model::RequestPaymentConfiguration>) -> Result<std::vec::Vec<u8>, aws_smithy_http::operation::error::BuildError> {
    let payload = match payload.as_ref() {
                                Some(t) => t,
                                None => return Ok(
        crate::operation_ser::rest_xml_unset_payload()
    )};
    Ok(
        crate::xml_ser::serialize_member_com_amazonaws_s3_synthetic_put_bucket_request_payment_input_request_payment_configuration(payload)?
    )
}

pub fn serialize_payload_put_bucket_tagging_input(payload: &std::option::Option<crate::model::Tagging>) -> Result<std::vec::Vec<u8>, aws_smithy_http::operation::error::BuildError> {
    let payload = match payload.as_ref() {
                                Some(t) => t,
                                None => return Ok(
        crate::operation_ser::rest_xml_unset_payload()
    )};
    Ok(
        crate::xml_ser::serialize_member_com_amazonaws_s3_synthetic_put_bucket_tagging_input_tagging(payload)?
    )
}

pub fn serialize_payload_put_bucket_versioning_input(payload: &std::option::Option<crate::model::VersioningConfiguration>) -> Result<std::vec::Vec<u8>, aws_smithy_http::operation::error::BuildError> {
    let payload = match payload.as_ref() {
                                Some(t) => t,
                                None => return Ok(
        crate::operation_ser::rest_xml_unset_payload()
    )};
    Ok(
        crate::xml_ser::serialize_member_com_amazonaws_s3_synthetic_put_bucket_versioning_input_versioning_configuration(payload)?
    )
}

pub fn serialize_payload_put_bucket_website_input(payload: &std::option::Option<crate::model::WebsiteConfiguration>) -> Result<std::vec::Vec<u8>, aws_smithy_http::operation::error::BuildError> {
    let payload = match payload.as_ref() {
                                Some(t) => t,
                                None => return Ok(
        crate::operation_ser::rest_xml_unset_payload()
    )};
    Ok(
        crate::xml_ser::serialize_member_com_amazonaws_s3_synthetic_put_bucket_website_input_website_configuration(payload)?
    )
}

pub fn serialize_payload_put_object_input(payload: aws_smithy_http::byte_stream::ByteStream) -> Result<aws_smithy_http::byte_stream::ByteStream, aws_smithy_http::operation::error::BuildError> {
    Ok(
        payload
    )
}

pub fn serialize_payload_put_object_acl_input(payload: &std::option::Option<crate::model::AccessControlPolicy>) -> Result<std::vec::Vec<u8>, aws_smithy_http::operation::error::BuildError> {
    let payload = match payload.as_ref() {
                                Some(t) => t,
                                None => return Ok(
        crate::operation_ser::rest_xml_unset_payload()
    )};
    Ok(
        crate::xml_ser::serialize_member_com_amazonaws_s3_synthetic_put_object_acl_input_access_control_policy(payload)?
    )
}

pub fn serialize_payload_put_object_legal_hold_input(payload: &std::option::Option<crate::model::ObjectLockLegalHold>) -> Result<std::vec::Vec<u8>, aws_smithy_http::operation::error::BuildError> {
    let payload = match payload.as_ref() {
                                Some(t) => t,
                                None => return Ok(
        crate::operation_ser::rest_xml_unset_payload()
    )};
    Ok(
        crate::xml_ser::serialize_member_com_amazonaws_s3_synthetic_put_object_legal_hold_input_legal_hold(payload)?
    )
}

pub fn serialize_payload_put_object_lock_configuration_input(payload: &std::option::Option<crate::model::ObjectLockConfiguration>) -> Result<std::vec::Vec<u8>, aws_smithy_http::operation::error::BuildError> {
    let payload = match payload.as_ref() {
                                Some(t) => t,
                                None => return Ok(
        crate::operation_ser::rest_xml_unset_payload()
    )};
    Ok(
        crate::xml_ser::serialize_member_com_amazonaws_s3_synthetic_put_object_lock_configuration_input_object_lock_configuration(payload)?
    )
}

pub fn serialize_payload_put_object_retention_input(payload: &std::option::Option<crate::model::ObjectLockRetention>) -> Result<std::vec::Vec<u8>, aws_smithy_http::operation::error::BuildError> {
    let payload = match payload.as_ref() {
                                Some(t) => t,
                                None => return Ok(
        crate::operation_ser::rest_xml_unset_payload()
    )};
    Ok(
        crate::xml_ser::serialize_member_com_amazonaws_s3_synthetic_put_object_retention_input_retention(payload)?
    )
}

pub fn serialize_payload_put_object_tagging_input(payload: &std::option::Option<crate::model::Tagging>) -> Result<std::vec::Vec<u8>, aws_smithy_http::operation::error::BuildError> {
    let payload = match payload.as_ref() {
                                Some(t) => t,
                                None => return Ok(
        crate::operation_ser::rest_xml_unset_payload()
    )};
    Ok(
        crate::xml_ser::serialize_member_com_amazonaws_s3_synthetic_put_object_tagging_input_tagging(payload)?
    )
}

pub fn serialize_payload_put_public_access_block_input(payload: &std::option::Option<crate::model::PublicAccessBlockConfiguration>) -> Result<std::vec::Vec<u8>, aws_smithy_http::operation::error::BuildError> {
    let payload = match payload.as_ref() {
                                Some(t) => t,
                                None => return Ok(
        crate::operation_ser::rest_xml_unset_payload()
    )};
    Ok(
        crate::xml_ser::serialize_member_com_amazonaws_s3_synthetic_put_public_access_block_input_public_access_block_configuration(payload)?
    )
}

pub fn serialize_payload_restore_object_input(payload: &std::option::Option<crate::model::RestoreRequest>) -> Result<std::vec::Vec<u8>, aws_smithy_http::operation::error::BuildError> {
    let payload = match payload.as_ref() {
                                Some(t) => t,
                                None => return Ok(
        crate::operation_ser::rest_xml_unset_payload()
    )};
    Ok(
        crate::xml_ser::serialize_member_com_amazonaws_s3_synthetic_restore_object_input_restore_request(payload)?
    )
}

pub fn serialize_operation_crate_operation_select_object_content(input: &crate::input::SelectObjectContentInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
     {
        let mut writer = aws_smithy_xml::encode::XmlWriter::new(&mut out);
                                #[allow(unused_mut)]
                                let mut root = writer.start_el("SelectObjectContentRequest").write_ns("http://s3.amazonaws.com/doc/2006-03-01/", None);
        crate::xml_ser::serialize_structure_crate_input_select_object_content_input(input, root)?
    }
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

pub fn serialize_payload_upload_part_input(payload: aws_smithy_http::byte_stream::ByteStream) -> Result<aws_smithy_http::byte_stream::ByteStream, aws_smithy_http::operation::error::BuildError> {
    Ok(
        payload
    )
}

pub fn serialize_payload_write_get_object_response_input(payload: aws_smithy_http::byte_stream::ByteStream) -> Result<aws_smithy_http::byte_stream::ByteStream, aws_smithy_http::operation::error::BuildError> {
    Ok(
        payload
    )
}

pub fn rest_xml_unset_payload() -> std::vec::Vec<u8> {
                    Vec::new()
                }

