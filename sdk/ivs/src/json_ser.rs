// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_batch_get_channel_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::BatchGetChannelInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.arns {
        let mut array_2 = object.key("arns").start_array();
        for item_3 in var_1 {
             {
                array_2.value().string(item_3.as_str());
            }
        }
        array_2.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_batch_get_stream_key_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::BatchGetStreamKeyInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_4) = &input.arns {
        let mut array_5 = object.key("arns").start_array();
        for item_6 in var_4 {
             {
                array_5.value().string(item_6.as_str());
            }
        }
        array_5.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_channel_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreateChannelInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if input.authorized {
        object.key("authorized").boolean(input.authorized);
    }
    if let Some(var_7) = &input.latency_mode {
        object.key("latencyMode").string(var_7.as_str());
    }
    if let Some(var_8) = &input.name {
        object.key("name").string(var_8.as_str());
    }
    if let Some(var_9) = &input.recording_configuration_arn {
        object.key("recordingConfigurationArn").string(var_9.as_str());
    }
    if let Some(var_10) = &input.tags {
        #[allow(unused_mut)]
        let mut object_11 = object.key("tags").start_object();
        for (key_12, value_13) in var_10 {
             {
                object_11.key(key_12.as_str()).string(value_13.as_str());
            }
        }
        object_11.finish();
    }
    if let Some(var_14) = &input.r#type {
        object.key("type").string(var_14.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_recording_configuration_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreateRecordingConfigurationInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_15) = &input.destination_configuration {
        #[allow(unused_mut)]
        let mut object_16 = object.key("destinationConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_destination_configuration(&mut object_16, var_15)?;
        object_16.finish();
    }
    if let Some(var_17) = &input.name {
        object.key("name").string(var_17.as_str());
    }
    if input.recording_reconnect_window_seconds != 0 {
        object.key("recordingReconnectWindowSeconds").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((input.recording_reconnect_window_seconds).into()));
    }
    if let Some(var_18) = &input.tags {
        #[allow(unused_mut)]
        let mut object_19 = object.key("tags").start_object();
        for (key_20, value_21) in var_18 {
             {
                object_19.key(key_20.as_str()).string(value_21.as_str());
            }
        }
        object_19.finish();
    }
    if let Some(var_22) = &input.thumbnail_configuration {
        #[allow(unused_mut)]
        let mut object_23 = object.key("thumbnailConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_thumbnail_configuration(&mut object_23, var_22)?;
        object_23.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_stream_key_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreateStreamKeyInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_24) = &input.channel_arn {
        object.key("channelArn").string(var_24.as_str());
    }
    if let Some(var_25) = &input.tags {
        #[allow(unused_mut)]
        let mut object_26 = object.key("tags").start_object();
        for (key_27, value_28) in var_25 {
             {
                object_26.key(key_27.as_str()).string(value_28.as_str());
            }
        }
        object_26.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_delete_channel_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DeleteChannelInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_29) = &input.arn {
        object.key("arn").string(var_29.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_delete_playback_key_pair_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DeletePlaybackKeyPairInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_30) = &input.arn {
        object.key("arn").string(var_30.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_delete_recording_configuration_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DeleteRecordingConfigurationInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_31) = &input.arn {
        object.key("arn").string(var_31.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_delete_stream_key_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DeleteStreamKeyInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_32) = &input.arn {
        object.key("arn").string(var_32.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_get_channel_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::GetChannelInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_33) = &input.arn {
        object.key("arn").string(var_33.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_get_playback_key_pair_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::GetPlaybackKeyPairInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_34) = &input.arn {
        object.key("arn").string(var_34.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_get_recording_configuration_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::GetRecordingConfigurationInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_35) = &input.arn {
        object.key("arn").string(var_35.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_get_stream_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::GetStreamInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_36) = &input.channel_arn {
        object.key("channelArn").string(var_36.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_get_stream_key_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::GetStreamKeyInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_37) = &input.arn {
        object.key("arn").string(var_37.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_get_stream_session_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::GetStreamSessionInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_38) = &input.channel_arn {
        object.key("channelArn").string(var_38.as_str());
    }
    if let Some(var_39) = &input.stream_id {
        object.key("streamId").string(var_39.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_import_playback_key_pair_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::ImportPlaybackKeyPairInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_40) = &input.name {
        object.key("name").string(var_40.as_str());
    }
    if let Some(var_41) = &input.public_key_material {
        object.key("publicKeyMaterial").string(var_41.as_str());
    }
    if let Some(var_42) = &input.tags {
        #[allow(unused_mut)]
        let mut object_43 = object.key("tags").start_object();
        for (key_44, value_45) in var_42 {
             {
                object_43.key(key_44.as_str()).string(value_45.as_str());
            }
        }
        object_43.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_channels_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::ListChannelsInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_46) = &input.filter_by_name {
        object.key("filterByName").string(var_46.as_str());
    }
    if let Some(var_47) = &input.filter_by_recording_configuration_arn {
        object.key("filterByRecordingConfigurationArn").string(var_47.as_str());
    }
    if input.max_results != 0 {
        object.key("maxResults").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((input.max_results).into()));
    }
    if let Some(var_48) = &input.next_token {
        object.key("nextToken").string(var_48.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_playback_key_pairs_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::ListPlaybackKeyPairsInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if input.max_results != 0 {
        object.key("maxResults").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((input.max_results).into()));
    }
    if let Some(var_49) = &input.next_token {
        object.key("nextToken").string(var_49.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_recording_configurations_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::ListRecordingConfigurationsInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if input.max_results != 0 {
        object.key("maxResults").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((input.max_results).into()));
    }
    if let Some(var_50) = &input.next_token {
        object.key("nextToken").string(var_50.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_stream_keys_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::ListStreamKeysInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_51) = &input.channel_arn {
        object.key("channelArn").string(var_51.as_str());
    }
    if input.max_results != 0 {
        object.key("maxResults").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((input.max_results).into()));
    }
    if let Some(var_52) = &input.next_token {
        object.key("nextToken").string(var_52.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_streams_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::ListStreamsInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_53) = &input.filter_by {
        #[allow(unused_mut)]
        let mut object_54 = object.key("filterBy").start_object();
        crate::json_ser::serialize_structure_crate_model_stream_filters(&mut object_54, var_53)?;
        object_54.finish();
    }
    if input.max_results != 0 {
        object.key("maxResults").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((input.max_results).into()));
    }
    if let Some(var_55) = &input.next_token {
        object.key("nextToken").string(var_55.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_stream_sessions_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::ListStreamSessionsInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_56) = &input.channel_arn {
        object.key("channelArn").string(var_56.as_str());
    }
    if input.max_results != 0 {
        object.key("maxResults").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((input.max_results).into()));
    }
    if let Some(var_57) = &input.next_token {
        object.key("nextToken").string(var_57.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_put_metadata_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::PutMetadataInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_58) = &input.channel_arn {
        object.key("channelArn").string(var_58.as_str());
    }
    if let Some(var_59) = &input.metadata {
        object.key("metadata").string(var_59.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_stop_stream_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::StopStreamInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_60) = &input.channel_arn {
        object.key("channelArn").string(var_60.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_tag_resource_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::TagResourceInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_61) = &input.tags {
        #[allow(unused_mut)]
        let mut object_62 = object.key("tags").start_object();
        for (key_63, value_64) in var_61 {
             {
                object_62.key(key_63.as_str()).string(value_64.as_str());
            }
        }
        object_62.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_channel_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::UpdateChannelInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_65) = &input.arn {
        object.key("arn").string(var_65.as_str());
    }
    if input.authorized {
        object.key("authorized").boolean(input.authorized);
    }
    if let Some(var_66) = &input.latency_mode {
        object.key("latencyMode").string(var_66.as_str());
    }
    if let Some(var_67) = &input.name {
        object.key("name").string(var_67.as_str());
    }
    if let Some(var_68) = &input.recording_configuration_arn {
        object.key("recordingConfigurationArn").string(var_68.as_str());
    }
    if let Some(var_69) = &input.r#type {
        object.key("type").string(var_69.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_destination_configuration(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::DestinationConfiguration) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_70) = &input.s3 {
        #[allow(unused_mut)]
        let mut object_71 = object.key("s3").start_object();
        crate::json_ser::serialize_structure_crate_model_s3_destination_configuration(&mut object_71, var_70)?;
        object_71.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_thumbnail_configuration(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::ThumbnailConfiguration) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_72) = &input.recording_mode {
        object.key("recordingMode").string(var_72.as_str());
    }
    if input.target_interval_seconds != 0 {
        object.key("targetIntervalSeconds").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((input.target_interval_seconds).into()));
    }
    Ok(())
}

pub fn serialize_structure_crate_model_stream_filters(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::StreamFilters) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_73) = &input.health {
        object.key("health").string(var_73.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_s3_destination_configuration(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::S3DestinationConfiguration) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_74) = &input.bucket_name {
        object.key("bucketName").string(var_74.as_str());
    }
    Ok(())
}

