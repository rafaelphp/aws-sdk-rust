// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_create_task_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreateTaskInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.client_token {
        object.key("clientToken").string(var_1.as_str());
    }
    if let Some(var_2) = &input.command {
        #[allow(unused_mut)]
        let mut object_3 = object.key("command").start_object();
        crate::json_ser::serialize_union_crate_model_command(&mut object_3, var_2)?;
        object_3.finish();
    }
    if let Some(var_4) = &input.description {
        object.key("description").string(var_4.as_str());
    }
    if let Some(var_5) = &input.tags {
        #[allow(unused_mut)]
        let mut object_6 = object.key("tags").start_object();
        for (key_7, value_8) in var_5 {
             {
                object_6.key(key_7.as_str()).string(value_8.as_str());
            }
        }
        object_6.finish();
    }
    if let Some(var_9) = &input.targets {
        let mut array_10 = object.key("targets").start_array();
        for item_11 in var_9 {
             {
                array_10.value().string(item_11.as_str());
            }
        }
        array_10.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_describe_device_ec2_instances_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DescribeDeviceEc2InstancesInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_12) = &input.instance_ids {
        let mut array_13 = object.key("instanceIds").start_array();
        for item_14 in var_12 {
             {
                array_13.value().string(item_14.as_str());
            }
        }
        array_13.finish();
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

pub fn serialize_union_crate_model_command(object_3: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::Command) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    match input {
        crate::model::Command::Unlock(inner) => {
             {
                #[allow(unused_mut)]
                let mut object_19 = object_3.key("unlock").start_object();
                crate::json_ser::serialize_structure_crate_model_unlock(&mut object_19, inner)?;
                object_19.finish();
            }
        },
        crate::model::Command::Reboot(inner) => {
             {
                #[allow(unused_mut)]
                let mut object_20 = object_3.key("reboot").start_object();
                crate::json_ser::serialize_structure_crate_model_reboot(&mut object_20, inner)?;
                object_20.finish();
            }
        },
        crate::model::Command::Unknown => return Err(aws_smithy_http::operation::error::SerializationError::unknown_variant("Command"))
    }
    Ok(())
}

pub fn serialize_structure_crate_model_unlock(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::Unlock) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    let (_, _) = (object, input);
    Ok(())
}

pub fn serialize_structure_crate_model_reboot(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::Reboot) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    let (_, _) = (object, input);
    Ok(())
}

