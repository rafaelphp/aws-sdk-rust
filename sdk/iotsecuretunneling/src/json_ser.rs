// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_close_tunnel_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CloseTunnelInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.tunnel_id {
        object.key("tunnelId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.delete {
        object.key("delete").boolean(*var_2);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_describe_tunnel_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DescribeTunnelInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_3) = &input.tunnel_id {
        object.key("tunnelId").string(var_3.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_tags_for_resource_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::ListTagsForResourceInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_4) = &input.resource_arn {
        object.key("resourceArn").string(var_4.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_tunnels_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::ListTunnelsInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_5) = &input.thing_name {
        object.key("thingName").string(var_5.as_str());
    }
    if let Some(var_6) = &input.max_results {
        object.key("maxResults").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_6).into()));
    }
    if let Some(var_7) = &input.next_token {
        object.key("nextToken").string(var_7.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_open_tunnel_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::OpenTunnelInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_8) = &input.description {
        object.key("description").string(var_8.as_str());
    }
    if let Some(var_9) = &input.tags {
        let mut array_10 = object.key("tags").start_array();
        for item_11 in var_9 {
             {
                #[allow(unused_mut)]
                let mut object_12 = array_10.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag(&mut object_12, item_11)?;
                object_12.finish();
            }
        }
        array_10.finish();
    }
    if let Some(var_13) = &input.destination_config {
        #[allow(unused_mut)]
        let mut object_14 = object.key("destinationConfig").start_object();
        crate::json_ser::serialize_structure_crate_model_destination_config(&mut object_14, var_13)?;
        object_14.finish();
    }
    if let Some(var_15) = &input.timeout_config {
        #[allow(unused_mut)]
        let mut object_16 = object.key("timeoutConfig").start_object();
        crate::json_ser::serialize_structure_crate_model_timeout_config(&mut object_16, var_15)?;
        object_16.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_rotate_tunnel_access_token_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::RotateTunnelAccessTokenInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_17) = &input.tunnel_id {
        object.key("tunnelId").string(var_17.as_str());
    }
    if let Some(var_18) = &input.client_mode {
        object.key("clientMode").string(var_18.as_str());
    }
    if let Some(var_19) = &input.destination_config {
        #[allow(unused_mut)]
        let mut object_20 = object.key("destinationConfig").start_object();
        crate::json_ser::serialize_structure_crate_model_destination_config(&mut object_20, var_19)?;
        object_20.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_tag_resource_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::TagResourceInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_21) = &input.resource_arn {
        object.key("resourceArn").string(var_21.as_str());
    }
    if let Some(var_22) = &input.tags {
        let mut array_23 = object.key("tags").start_array();
        for item_24 in var_22 {
             {
                #[allow(unused_mut)]
                let mut object_25 = array_23.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag(&mut object_25, item_24)?;
                object_25.finish();
            }
        }
        array_23.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_untag_resource_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::UntagResourceInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_26) = &input.resource_arn {
        object.key("resourceArn").string(var_26.as_str());
    }
    if let Some(var_27) = &input.tag_keys {
        let mut array_28 = object.key("tagKeys").start_array();
        for item_29 in var_27 {
             {
                array_28.value().string(item_29.as_str());
            }
        }
        array_28.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_tag(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::Tag) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_30) = &input.key {
        object.key("key").string(var_30.as_str());
    }
    if let Some(var_31) = &input.value {
        object.key("value").string(var_31.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_destination_config(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::DestinationConfig) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_32) = &input.thing_name {
        object.key("thingName").string(var_32.as_str());
    }
    if let Some(var_33) = &input.services {
        let mut array_34 = object.key("services").start_array();
        for item_35 in var_33 {
             {
                array_34.value().string(item_35.as_str());
            }
        }
        array_34.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_timeout_config(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::TimeoutConfig) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_36) = &input.max_lifetime_timeout_minutes {
        object.key("maxLifetimeTimeoutMinutes").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_36).into()));
    }
    Ok(())
}

