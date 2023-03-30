// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_copy_package_versions_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CopyPackageVersionsInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.allow_overwrite {
        object.key("allowOverwrite").boolean(*var_1);
    }
    if let Some(var_2) = &input.include_from_upstream {
        object.key("includeFromUpstream").boolean(*var_2);
    }
    if let Some(var_3) = &input.version_revisions {
        #[allow(unused_mut)]
        let mut object_4 = object.key("versionRevisions").start_object();
        for (key_5, value_6) in var_3 {
             {
                object_4.key(key_5.as_str()).string(value_6.as_str());
            }
        }
        object_4.finish();
    }
    if let Some(var_7) = &input.versions {
        let mut array_8 = object.key("versions").start_array();
        for item_9 in var_7 {
             {
                array_8.value().string(item_9.as_str());
            }
        }
        array_8.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_domain_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreateDomainInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_10) = &input.encryption_key {
        object.key("encryptionKey").string(var_10.as_str());
    }
    if let Some(var_11) = &input.tags {
        let mut array_12 = object.key("tags").start_array();
        for item_13 in var_11 {
             {
                #[allow(unused_mut)]
                let mut object_14 = array_12.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag(&mut object_14, item_13)?;
                object_14.finish();
            }
        }
        array_12.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_repository_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreateRepositoryInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_15) = &input.description {
        object.key("description").string(var_15.as_str());
    }
    if let Some(var_16) = &input.tags {
        let mut array_17 = object.key("tags").start_array();
        for item_18 in var_16 {
             {
                #[allow(unused_mut)]
                let mut object_19 = array_17.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag(&mut object_19, item_18)?;
                object_19.finish();
            }
        }
        array_17.finish();
    }
    if let Some(var_20) = &input.upstreams {
        let mut array_21 = object.key("upstreams").start_array();
        for item_22 in var_20 {
             {
                #[allow(unused_mut)]
                let mut object_23 = array_21.value().start_object();
                crate::json_ser::serialize_structure_crate_model_upstream_repository(&mut object_23, item_22)?;
                object_23.finish();
            }
        }
        array_21.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_delete_package_versions_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DeletePackageVersionsInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_24) = &input.expected_status {
        object.key("expectedStatus").string(var_24.as_str());
    }
    if let Some(var_25) = &input.versions {
        let mut array_26 = object.key("versions").start_array();
        for item_27 in var_25 {
             {
                array_26.value().string(item_27.as_str());
            }
        }
        array_26.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_dispose_package_versions_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DisposePackageVersionsInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_28) = &input.expected_status {
        object.key("expectedStatus").string(var_28.as_str());
    }
    if let Some(var_29) = &input.version_revisions {
        #[allow(unused_mut)]
        let mut object_30 = object.key("versionRevisions").start_object();
        for (key_31, value_32) in var_29 {
             {
                object_30.key(key_31.as_str()).string(value_32.as_str());
            }
        }
        object_30.finish();
    }
    if let Some(var_33) = &input.versions {
        let mut array_34 = object.key("versions").start_array();
        for item_35 in var_33 {
             {
                array_34.value().string(item_35.as_str());
            }
        }
        array_34.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_domains_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::ListDomainsInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_36) = &input.max_results {
        object.key("maxResults").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_36).into()));
    }
    if let Some(var_37) = &input.next_token {
        object.key("nextToken").string(var_37.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_put_domain_permissions_policy_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::PutDomainPermissionsPolicyInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_38) = &input.domain {
        object.key("domain").string(var_38.as_str());
    }
    if let Some(var_39) = &input.domain_owner {
        object.key("domainOwner").string(var_39.as_str());
    }
    if let Some(var_40) = &input.policy_document {
        object.key("policyDocument").string(var_40.as_str());
    }
    if let Some(var_41) = &input.policy_revision {
        object.key("policyRevision").string(var_41.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_put_package_origin_configuration_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::PutPackageOriginConfigurationInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_42) = &input.restrictions {
        #[allow(unused_mut)]
        let mut object_43 = object.key("restrictions").start_object();
        crate::json_ser::serialize_structure_crate_model_package_origin_restrictions(&mut object_43, var_42)?;
        object_43.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_put_repository_permissions_policy_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::PutRepositoryPermissionsPolicyInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_44) = &input.policy_document {
        object.key("policyDocument").string(var_44.as_str());
    }
    if let Some(var_45) = &input.policy_revision {
        object.key("policyRevision").string(var_45.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_tag_resource_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::TagResourceInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_46) = &input.tags {
        let mut array_47 = object.key("tags").start_array();
        for item_48 in var_46 {
             {
                #[allow(unused_mut)]
                let mut object_49 = array_47.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag(&mut object_49, item_48)?;
                object_49.finish();
            }
        }
        array_47.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_untag_resource_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::UntagResourceInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_50) = &input.tag_keys {
        let mut array_51 = object.key("tagKeys").start_array();
        for item_52 in var_50 {
             {
                array_51.value().string(item_52.as_str());
            }
        }
        array_51.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_package_versions_status_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::UpdatePackageVersionsStatusInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_53) = &input.expected_status {
        object.key("expectedStatus").string(var_53.as_str());
    }
    if let Some(var_54) = &input.target_status {
        object.key("targetStatus").string(var_54.as_str());
    }
    if let Some(var_55) = &input.version_revisions {
        #[allow(unused_mut)]
        let mut object_56 = object.key("versionRevisions").start_object();
        for (key_57, value_58) in var_55 {
             {
                object_56.key(key_57.as_str()).string(value_58.as_str());
            }
        }
        object_56.finish();
    }
    if let Some(var_59) = &input.versions {
        let mut array_60 = object.key("versions").start_array();
        for item_61 in var_59 {
             {
                array_60.value().string(item_61.as_str());
            }
        }
        array_60.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_repository_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::UpdateRepositoryInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_62) = &input.description {
        object.key("description").string(var_62.as_str());
    }
    if let Some(var_63) = &input.upstreams {
        let mut array_64 = object.key("upstreams").start_array();
        for item_65 in var_63 {
             {
                #[allow(unused_mut)]
                let mut object_66 = array_64.value().start_object();
                crate::json_ser::serialize_structure_crate_model_upstream_repository(&mut object_66, item_65)?;
                object_66.finish();
            }
        }
        array_64.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_tag(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::Tag) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_67) = &input.key {
        object.key("key").string(var_67.as_str());
    }
    if let Some(var_68) = &input.value {
        object.key("value").string(var_68.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_upstream_repository(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::UpstreamRepository) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_69) = &input.repository_name {
        object.key("repositoryName").string(var_69.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_package_origin_restrictions(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::PackageOriginRestrictions) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_70) = &input.publish {
        object.key("publish").string(var_70.as_str());
    }
    if let Some(var_71) = &input.upstream {
        object.key("upstream").string(var_71.as_str());
    }
    Ok(())
}

