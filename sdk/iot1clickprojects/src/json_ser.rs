// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_associate_device_with_placement_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::AssociateDeviceWithPlacementInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.device_id {
        object.key("deviceId").string(var_1.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_placement_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreatePlacementInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_2) = &input.attributes {
        #[allow(unused_mut)]
        let mut object_3 = object.key("attributes").start_object();
        for (key_4, value_5) in var_2 {
             {
                object_3.key(key_4.as_str()).string(value_5.as_str());
            }
        }
        object_3.finish();
    }
    if let Some(var_6) = &input.placement_name {
        object.key("placementName").string(var_6.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_project_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreateProjectInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_7) = &input.description {
        object.key("description").string(var_7.as_str());
    }
    if let Some(var_8) = &input.placement_template {
        #[allow(unused_mut)]
        let mut object_9 = object.key("placementTemplate").start_object();
        crate::json_ser::serialize_structure_crate_model_placement_template(&mut object_9, var_8)?;
        object_9.finish();
    }
    if let Some(var_10) = &input.project_name {
        object.key("projectName").string(var_10.as_str());
    }
    if let Some(var_11) = &input.tags {
        #[allow(unused_mut)]
        let mut object_12 = object.key("tags").start_object();
        for (key_13, value_14) in var_11 {
             {
                object_12.key(key_13.as_str()).string(value_14.as_str());
            }
        }
        object_12.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_tag_resource_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::TagResourceInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_15) = &input.tags {
        #[allow(unused_mut)]
        let mut object_16 = object.key("tags").start_object();
        for (key_17, value_18) in var_15 {
             {
                object_16.key(key_17.as_str()).string(value_18.as_str());
            }
        }
        object_16.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_placement_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::UpdatePlacementInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_19) = &input.attributes {
        #[allow(unused_mut)]
        let mut object_20 = object.key("attributes").start_object();
        for (key_21, value_22) in var_19 {
             {
                object_20.key(key_21.as_str()).string(value_22.as_str());
            }
        }
        object_20.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_project_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::UpdateProjectInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_23) = &input.description {
        object.key("description").string(var_23.as_str());
    }
    if let Some(var_24) = &input.placement_template {
        #[allow(unused_mut)]
        let mut object_25 = object.key("placementTemplate").start_object();
        crate::json_ser::serialize_structure_crate_model_placement_template(&mut object_25, var_24)?;
        object_25.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_placement_template(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::PlacementTemplate) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_26) = &input.default_attributes {
        #[allow(unused_mut)]
        let mut object_27 = object.key("defaultAttributes").start_object();
        for (key_28, value_29) in var_26 {
             {
                object_27.key(key_28.as_str()).string(value_29.as_str());
            }
        }
        object_27.finish();
    }
    if let Some(var_30) = &input.device_templates {
        #[allow(unused_mut)]
        let mut object_31 = object.key("deviceTemplates").start_object();
        for (key_32, value_33) in var_30 {
             {
                #[allow(unused_mut)]
                let mut object_34 = object_31.key(key_32.as_str()).start_object();
                crate::json_ser::serialize_structure_crate_model_device_template(&mut object_34, value_33)?;
                object_34.finish();
            }
        }
        object_31.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_device_template(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::DeviceTemplate) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_35) = &input.device_type {
        object.key("deviceType").string(var_35.as_str());
    }
    if let Some(var_36) = &input.callback_overrides {
        #[allow(unused_mut)]
        let mut object_37 = object.key("callbackOverrides").start_object();
        for (key_38, value_39) in var_36 {
             {
                object_37.key(key_38.as_str()).string(value_39.as_str());
            }
        }
        object_37.finish();
    }
    Ok(())
}

