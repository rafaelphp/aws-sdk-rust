// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_complete_attachment_upload_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CompleteAttachmentUploadInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.attachment_ids {
        let mut array_2 = object.key("AttachmentIds").start_array();
        for item_3 in var_1 {
             {
                array_2.value().string(item_3.as_str());
            }
        }
        array_2.finish();
    }
    if let Some(var_4) = &input.client_token {
        object.key("ClientToken").string(var_4.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_participant_connection_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreateParticipantConnectionInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_5) = &input.connect_participant {
        object.key("ConnectParticipant").boolean(*var_5);
    }
    if let Some(var_6) = &input.r#type {
        let mut array_7 = object.key("Type").start_array();
        for item_8 in var_6 {
             {
                array_7.value().string(item_8.as_str());
            }
        }
        array_7.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_disconnect_participant_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DisconnectParticipantInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_9) = &input.client_token {
        object.key("ClientToken").string(var_9.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_get_attachment_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::GetAttachmentInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_10) = &input.attachment_id {
        object.key("AttachmentId").string(var_10.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_get_transcript_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::GetTranscriptInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_11) = &input.contact_id {
        object.key("ContactId").string(var_11.as_str());
    }
    if let Some(var_12) = &input.max_results {
        object.key("MaxResults").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_12).into()));
    }
    if let Some(var_13) = &input.next_token {
        object.key("NextToken").string(var_13.as_str());
    }
    if let Some(var_14) = &input.scan_direction {
        object.key("ScanDirection").string(var_14.as_str());
    }
    if let Some(var_15) = &input.sort_order {
        object.key("SortOrder").string(var_15.as_str());
    }
    if let Some(var_16) = &input.start_position {
        #[allow(unused_mut)]
        let mut object_17 = object.key("StartPosition").start_object();
        crate::json_ser::serialize_structure_crate_model_start_position(&mut object_17, var_16)?;
        object_17.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_send_event_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::SendEventInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_18) = &input.client_token {
        object.key("ClientToken").string(var_18.as_str());
    }
    if let Some(var_19) = &input.content {
        object.key("Content").string(var_19.as_str());
    }
    if let Some(var_20) = &input.content_type {
        object.key("ContentType").string(var_20.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_send_message_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::SendMessageInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_21) = &input.client_token {
        object.key("ClientToken").string(var_21.as_str());
    }
    if let Some(var_22) = &input.content {
        object.key("Content").string(var_22.as_str());
    }
    if let Some(var_23) = &input.content_type {
        object.key("ContentType").string(var_23.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_start_attachment_upload_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::StartAttachmentUploadInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_24) = &input.attachment_name {
        object.key("AttachmentName").string(var_24.as_str());
    }
     {
        object.key("AttachmentSizeInBytes").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((input.attachment_size_in_bytes).into()));
    }
    if let Some(var_25) = &input.client_token {
        object.key("ClientToken").string(var_25.as_str());
    }
    if let Some(var_26) = &input.content_type {
        object.key("ContentType").string(var_26.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_start_position(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::StartPosition) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_27) = &input.id {
        object.key("Id").string(var_27.as_str());
    }
    if let Some(var_28) = &input.absolute_time {
        object.key("AbsoluteTime").string(var_28.as_str());
    }
    if input.most_recent != 0 {
        object.key("MostRecent").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((input.most_recent).into()));
    }
    Ok(())
}

