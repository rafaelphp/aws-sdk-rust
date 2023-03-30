// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_associate_custom_domain_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::AssociateCustomDomainInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.service_arn {
        object.key("ServiceArn").string(var_1.as_str());
    }
    if let Some(var_2) = &input.domain_name {
        object.key("DomainName").string(var_2.as_str());
    }
    if let Some(var_3) = &input.enable_www_subdomain {
        object.key("EnableWWWSubdomain").boolean(*var_3);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_auto_scaling_configuration_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreateAutoScalingConfigurationInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_4) = &input.auto_scaling_configuration_name {
        object.key("AutoScalingConfigurationName").string(var_4.as_str());
    }
    if let Some(var_5) = &input.max_concurrency {
        object.key("MaxConcurrency").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_5).into()));
    }
    if let Some(var_6) = &input.min_size {
        object.key("MinSize").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_6).into()));
    }
    if let Some(var_7) = &input.max_size {
        object.key("MaxSize").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_7).into()));
    }
    if let Some(var_8) = &input.tags {
        let mut array_9 = object.key("Tags").start_array();
        for item_10 in var_8 {
             {
                #[allow(unused_mut)]
                let mut object_11 = array_9.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag(&mut object_11, item_10)?;
                object_11.finish();
            }
        }
        array_9.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_connection_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreateConnectionInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_12) = &input.connection_name {
        object.key("ConnectionName").string(var_12.as_str());
    }
    if let Some(var_13) = &input.provider_type {
        object.key("ProviderType").string(var_13.as_str());
    }
    if let Some(var_14) = &input.tags {
        let mut array_15 = object.key("Tags").start_array();
        for item_16 in var_14 {
             {
                #[allow(unused_mut)]
                let mut object_17 = array_15.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag(&mut object_17, item_16)?;
                object_17.finish();
            }
        }
        array_15.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_observability_configuration_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreateObservabilityConfigurationInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_18) = &input.observability_configuration_name {
        object.key("ObservabilityConfigurationName").string(var_18.as_str());
    }
    if let Some(var_19) = &input.trace_configuration {
        #[allow(unused_mut)]
        let mut object_20 = object.key("TraceConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_trace_configuration(&mut object_20, var_19)?;
        object_20.finish();
    }
    if let Some(var_21) = &input.tags {
        let mut array_22 = object.key("Tags").start_array();
        for item_23 in var_21 {
             {
                #[allow(unused_mut)]
                let mut object_24 = array_22.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag(&mut object_24, item_23)?;
                object_24.finish();
            }
        }
        array_22.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_service_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreateServiceInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_25) = &input.service_name {
        object.key("ServiceName").string(var_25.as_str());
    }
    if let Some(var_26) = &input.source_configuration {
        #[allow(unused_mut)]
        let mut object_27 = object.key("SourceConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_source_configuration(&mut object_27, var_26)?;
        object_27.finish();
    }
    if let Some(var_28) = &input.instance_configuration {
        #[allow(unused_mut)]
        let mut object_29 = object.key("InstanceConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_instance_configuration(&mut object_29, var_28)?;
        object_29.finish();
    }
    if let Some(var_30) = &input.tags {
        let mut array_31 = object.key("Tags").start_array();
        for item_32 in var_30 {
             {
                #[allow(unused_mut)]
                let mut object_33 = array_31.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag(&mut object_33, item_32)?;
                object_33.finish();
            }
        }
        array_31.finish();
    }
    if let Some(var_34) = &input.encryption_configuration {
        #[allow(unused_mut)]
        let mut object_35 = object.key("EncryptionConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_encryption_configuration(&mut object_35, var_34)?;
        object_35.finish();
    }
    if let Some(var_36) = &input.health_check_configuration {
        #[allow(unused_mut)]
        let mut object_37 = object.key("HealthCheckConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_health_check_configuration(&mut object_37, var_36)?;
        object_37.finish();
    }
    if let Some(var_38) = &input.auto_scaling_configuration_arn {
        object.key("AutoScalingConfigurationArn").string(var_38.as_str());
    }
    if let Some(var_39) = &input.network_configuration {
        #[allow(unused_mut)]
        let mut object_40 = object.key("NetworkConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_network_configuration(&mut object_40, var_39)?;
        object_40.finish();
    }
    if let Some(var_41) = &input.observability_configuration {
        #[allow(unused_mut)]
        let mut object_42 = object.key("ObservabilityConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_service_observability_configuration(&mut object_42, var_41)?;
        object_42.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_vpc_connector_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreateVpcConnectorInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_43) = &input.vpc_connector_name {
        object.key("VpcConnectorName").string(var_43.as_str());
    }
    if let Some(var_44) = &input.subnets {
        let mut array_45 = object.key("Subnets").start_array();
        for item_46 in var_44 {
             {
                array_45.value().string(item_46.as_str());
            }
        }
        array_45.finish();
    }
    if let Some(var_47) = &input.security_groups {
        let mut array_48 = object.key("SecurityGroups").start_array();
        for item_49 in var_47 {
             {
                array_48.value().string(item_49.as_str());
            }
        }
        array_48.finish();
    }
    if let Some(var_50) = &input.tags {
        let mut array_51 = object.key("Tags").start_array();
        for item_52 in var_50 {
             {
                #[allow(unused_mut)]
                let mut object_53 = array_51.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag(&mut object_53, item_52)?;
                object_53.finish();
            }
        }
        array_51.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_vpc_ingress_connection_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreateVpcIngressConnectionInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_54) = &input.service_arn {
        object.key("ServiceArn").string(var_54.as_str());
    }
    if let Some(var_55) = &input.vpc_ingress_connection_name {
        object.key("VpcIngressConnectionName").string(var_55.as_str());
    }
    if let Some(var_56) = &input.ingress_vpc_configuration {
        #[allow(unused_mut)]
        let mut object_57 = object.key("IngressVpcConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_ingress_vpc_configuration(&mut object_57, var_56)?;
        object_57.finish();
    }
    if let Some(var_58) = &input.tags {
        let mut array_59 = object.key("Tags").start_array();
        for item_60 in var_58 {
             {
                #[allow(unused_mut)]
                let mut object_61 = array_59.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag(&mut object_61, item_60)?;
                object_61.finish();
            }
        }
        array_59.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_delete_auto_scaling_configuration_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DeleteAutoScalingConfigurationInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_62) = &input.auto_scaling_configuration_arn {
        object.key("AutoScalingConfigurationArn").string(var_62.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_delete_connection_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DeleteConnectionInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_63) = &input.connection_arn {
        object.key("ConnectionArn").string(var_63.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_delete_observability_configuration_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DeleteObservabilityConfigurationInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_64) = &input.observability_configuration_arn {
        object.key("ObservabilityConfigurationArn").string(var_64.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_delete_service_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DeleteServiceInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_65) = &input.service_arn {
        object.key("ServiceArn").string(var_65.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_delete_vpc_connector_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DeleteVpcConnectorInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_66) = &input.vpc_connector_arn {
        object.key("VpcConnectorArn").string(var_66.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_delete_vpc_ingress_connection_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DeleteVpcIngressConnectionInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_67) = &input.vpc_ingress_connection_arn {
        object.key("VpcIngressConnectionArn").string(var_67.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_describe_auto_scaling_configuration_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DescribeAutoScalingConfigurationInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_68) = &input.auto_scaling_configuration_arn {
        object.key("AutoScalingConfigurationArn").string(var_68.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_describe_custom_domains_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DescribeCustomDomainsInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_69) = &input.service_arn {
        object.key("ServiceArn").string(var_69.as_str());
    }
    if let Some(var_70) = &input.next_token {
        object.key("NextToken").string(var_70.as_str());
    }
    if let Some(var_71) = &input.max_results {
        object.key("MaxResults").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_71).into()));
    }
    Ok(())
}

pub fn serialize_structure_crate_input_describe_observability_configuration_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DescribeObservabilityConfigurationInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_72) = &input.observability_configuration_arn {
        object.key("ObservabilityConfigurationArn").string(var_72.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_describe_service_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DescribeServiceInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_73) = &input.service_arn {
        object.key("ServiceArn").string(var_73.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_describe_vpc_connector_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DescribeVpcConnectorInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_74) = &input.vpc_connector_arn {
        object.key("VpcConnectorArn").string(var_74.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_describe_vpc_ingress_connection_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DescribeVpcIngressConnectionInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_75) = &input.vpc_ingress_connection_arn {
        object.key("VpcIngressConnectionArn").string(var_75.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_disassociate_custom_domain_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DisassociateCustomDomainInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_76) = &input.service_arn {
        object.key("ServiceArn").string(var_76.as_str());
    }
    if let Some(var_77) = &input.domain_name {
        object.key("DomainName").string(var_77.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_auto_scaling_configurations_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::ListAutoScalingConfigurationsInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_78) = &input.auto_scaling_configuration_name {
        object.key("AutoScalingConfigurationName").string(var_78.as_str());
    }
    if input.latest_only {
        object.key("LatestOnly").boolean(input.latest_only);
    }
    if let Some(var_79) = &input.max_results {
        object.key("MaxResults").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_79).into()));
    }
    if let Some(var_80) = &input.next_token {
        object.key("NextToken").string(var_80.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_connections_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::ListConnectionsInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_81) = &input.connection_name {
        object.key("ConnectionName").string(var_81.as_str());
    }
    if let Some(var_82) = &input.max_results {
        object.key("MaxResults").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_82).into()));
    }
    if let Some(var_83) = &input.next_token {
        object.key("NextToken").string(var_83.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_observability_configurations_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::ListObservabilityConfigurationsInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_84) = &input.observability_configuration_name {
        object.key("ObservabilityConfigurationName").string(var_84.as_str());
    }
    if input.latest_only {
        object.key("LatestOnly").boolean(input.latest_only);
    }
    if let Some(var_85) = &input.max_results {
        object.key("MaxResults").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_85).into()));
    }
    if let Some(var_86) = &input.next_token {
        object.key("NextToken").string(var_86.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_operations_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::ListOperationsInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_87) = &input.service_arn {
        object.key("ServiceArn").string(var_87.as_str());
    }
    if let Some(var_88) = &input.next_token {
        object.key("NextToken").string(var_88.as_str());
    }
    if let Some(var_89) = &input.max_results {
        object.key("MaxResults").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_89).into()));
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_services_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::ListServicesInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_90) = &input.next_token {
        object.key("NextToken").string(var_90.as_str());
    }
    if let Some(var_91) = &input.max_results {
        object.key("MaxResults").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_91).into()));
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_tags_for_resource_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::ListTagsForResourceInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_92) = &input.resource_arn {
        object.key("ResourceArn").string(var_92.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_vpc_connectors_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::ListVpcConnectorsInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_93) = &input.max_results {
        object.key("MaxResults").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_93).into()));
    }
    if let Some(var_94) = &input.next_token {
        object.key("NextToken").string(var_94.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_vpc_ingress_connections_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::ListVpcIngressConnectionsInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_95) = &input.filter {
        #[allow(unused_mut)]
        let mut object_96 = object.key("Filter").start_object();
        crate::json_ser::serialize_structure_crate_model_list_vpc_ingress_connections_filter(&mut object_96, var_95)?;
        object_96.finish();
    }
    if let Some(var_97) = &input.max_results {
        object.key("MaxResults").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_97).into()));
    }
    if let Some(var_98) = &input.next_token {
        object.key("NextToken").string(var_98.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_pause_service_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::PauseServiceInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_99) = &input.service_arn {
        object.key("ServiceArn").string(var_99.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_resume_service_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::ResumeServiceInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_100) = &input.service_arn {
        object.key("ServiceArn").string(var_100.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_start_deployment_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::StartDeploymentInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_101) = &input.service_arn {
        object.key("ServiceArn").string(var_101.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_tag_resource_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::TagResourceInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_102) = &input.resource_arn {
        object.key("ResourceArn").string(var_102.as_str());
    }
    if let Some(var_103) = &input.tags {
        let mut array_104 = object.key("Tags").start_array();
        for item_105 in var_103 {
             {
                #[allow(unused_mut)]
                let mut object_106 = array_104.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag(&mut object_106, item_105)?;
                object_106.finish();
            }
        }
        array_104.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_untag_resource_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::UntagResourceInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_107) = &input.resource_arn {
        object.key("ResourceArn").string(var_107.as_str());
    }
    if let Some(var_108) = &input.tag_keys {
        let mut array_109 = object.key("TagKeys").start_array();
        for item_110 in var_108 {
             {
                array_109.value().string(item_110.as_str());
            }
        }
        array_109.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_service_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::UpdateServiceInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_111) = &input.service_arn {
        object.key("ServiceArn").string(var_111.as_str());
    }
    if let Some(var_112) = &input.source_configuration {
        #[allow(unused_mut)]
        let mut object_113 = object.key("SourceConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_source_configuration(&mut object_113, var_112)?;
        object_113.finish();
    }
    if let Some(var_114) = &input.instance_configuration {
        #[allow(unused_mut)]
        let mut object_115 = object.key("InstanceConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_instance_configuration(&mut object_115, var_114)?;
        object_115.finish();
    }
    if let Some(var_116) = &input.auto_scaling_configuration_arn {
        object.key("AutoScalingConfigurationArn").string(var_116.as_str());
    }
    if let Some(var_117) = &input.health_check_configuration {
        #[allow(unused_mut)]
        let mut object_118 = object.key("HealthCheckConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_health_check_configuration(&mut object_118, var_117)?;
        object_118.finish();
    }
    if let Some(var_119) = &input.network_configuration {
        #[allow(unused_mut)]
        let mut object_120 = object.key("NetworkConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_network_configuration(&mut object_120, var_119)?;
        object_120.finish();
    }
    if let Some(var_121) = &input.observability_configuration {
        #[allow(unused_mut)]
        let mut object_122 = object.key("ObservabilityConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_service_observability_configuration(&mut object_122, var_121)?;
        object_122.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_vpc_ingress_connection_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::UpdateVpcIngressConnectionInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_123) = &input.vpc_ingress_connection_arn {
        object.key("VpcIngressConnectionArn").string(var_123.as_str());
    }
    if let Some(var_124) = &input.ingress_vpc_configuration {
        #[allow(unused_mut)]
        let mut object_125 = object.key("IngressVpcConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_ingress_vpc_configuration(&mut object_125, var_124)?;
        object_125.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_tag(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::Tag) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_126) = &input.key {
        object.key("Key").string(var_126.as_str());
    }
    if let Some(var_127) = &input.value {
        object.key("Value").string(var_127.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_trace_configuration(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::TraceConfiguration) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_128) = &input.vendor {
        object.key("Vendor").string(var_128.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_source_configuration(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::SourceConfiguration) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_129) = &input.code_repository {
        #[allow(unused_mut)]
        let mut object_130 = object.key("CodeRepository").start_object();
        crate::json_ser::serialize_structure_crate_model_code_repository(&mut object_130, var_129)?;
        object_130.finish();
    }
    if let Some(var_131) = &input.image_repository {
        #[allow(unused_mut)]
        let mut object_132 = object.key("ImageRepository").start_object();
        crate::json_ser::serialize_structure_crate_model_image_repository(&mut object_132, var_131)?;
        object_132.finish();
    }
    if let Some(var_133) = &input.auto_deployments_enabled {
        object.key("AutoDeploymentsEnabled").boolean(*var_133);
    }
    if let Some(var_134) = &input.authentication_configuration {
        #[allow(unused_mut)]
        let mut object_135 = object.key("AuthenticationConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_authentication_configuration(&mut object_135, var_134)?;
        object_135.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_instance_configuration(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::InstanceConfiguration) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_136) = &input.cpu {
        object.key("Cpu").string(var_136.as_str());
    }
    if let Some(var_137) = &input.memory {
        object.key("Memory").string(var_137.as_str());
    }
    if let Some(var_138) = &input.instance_role_arn {
        object.key("InstanceRoleArn").string(var_138.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_encryption_configuration(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::EncryptionConfiguration) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_139) = &input.kms_key {
        object.key("KmsKey").string(var_139.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_health_check_configuration(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::HealthCheckConfiguration) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_140) = &input.protocol {
        object.key("Protocol").string(var_140.as_str());
    }
    if let Some(var_141) = &input.path {
        object.key("Path").string(var_141.as_str());
    }
    if let Some(var_142) = &input.interval {
        object.key("Interval").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_142).into()));
    }
    if let Some(var_143) = &input.timeout {
        object.key("Timeout").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_143).into()));
    }
    if let Some(var_144) = &input.healthy_threshold {
        object.key("HealthyThreshold").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_144).into()));
    }
    if let Some(var_145) = &input.unhealthy_threshold {
        object.key("UnhealthyThreshold").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_145).into()));
    }
    Ok(())
}

pub fn serialize_structure_crate_model_network_configuration(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::NetworkConfiguration) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_146) = &input.egress_configuration {
        #[allow(unused_mut)]
        let mut object_147 = object.key("EgressConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_egress_configuration(&mut object_147, var_146)?;
        object_147.finish();
    }
    if let Some(var_148) = &input.ingress_configuration {
        #[allow(unused_mut)]
        let mut object_149 = object.key("IngressConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_ingress_configuration(&mut object_149, var_148)?;
        object_149.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_service_observability_configuration(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::ServiceObservabilityConfiguration) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
     {
        object.key("ObservabilityEnabled").boolean(input.observability_enabled);
    }
    if let Some(var_150) = &input.observability_configuration_arn {
        object.key("ObservabilityConfigurationArn").string(var_150.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_ingress_vpc_configuration(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::IngressVpcConfiguration) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_151) = &input.vpc_id {
        object.key("VpcId").string(var_151.as_str());
    }
    if let Some(var_152) = &input.vpc_endpoint_id {
        object.key("VpcEndpointId").string(var_152.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_list_vpc_ingress_connections_filter(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::ListVpcIngressConnectionsFilter) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_153) = &input.service_arn {
        object.key("ServiceArn").string(var_153.as_str());
    }
    if let Some(var_154) = &input.vpc_endpoint_id {
        object.key("VpcEndpointId").string(var_154.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_code_repository(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::CodeRepository) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_155) = &input.repository_url {
        object.key("RepositoryUrl").string(var_155.as_str());
    }
    if let Some(var_156) = &input.source_code_version {
        #[allow(unused_mut)]
        let mut object_157 = object.key("SourceCodeVersion").start_object();
        crate::json_ser::serialize_structure_crate_model_source_code_version(&mut object_157, var_156)?;
        object_157.finish();
    }
    if let Some(var_158) = &input.code_configuration {
        #[allow(unused_mut)]
        let mut object_159 = object.key("CodeConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_code_configuration(&mut object_159, var_158)?;
        object_159.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_image_repository(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::ImageRepository) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_160) = &input.image_identifier {
        object.key("ImageIdentifier").string(var_160.as_str());
    }
    if let Some(var_161) = &input.image_configuration {
        #[allow(unused_mut)]
        let mut object_162 = object.key("ImageConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_image_configuration(&mut object_162, var_161)?;
        object_162.finish();
    }
    if let Some(var_163) = &input.image_repository_type {
        object.key("ImageRepositoryType").string(var_163.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_authentication_configuration(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::AuthenticationConfiguration) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_164) = &input.connection_arn {
        object.key("ConnectionArn").string(var_164.as_str());
    }
    if let Some(var_165) = &input.access_role_arn {
        object.key("AccessRoleArn").string(var_165.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_egress_configuration(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::EgressConfiguration) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_166) = &input.egress_type {
        object.key("EgressType").string(var_166.as_str());
    }
    if let Some(var_167) = &input.vpc_connector_arn {
        object.key("VpcConnectorArn").string(var_167.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_ingress_configuration(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::IngressConfiguration) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if input.is_publicly_accessible {
        object.key("IsPubliclyAccessible").boolean(input.is_publicly_accessible);
    }
    Ok(())
}

pub fn serialize_structure_crate_model_source_code_version(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::SourceCodeVersion) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_168) = &input.r#type {
        object.key("Type").string(var_168.as_str());
    }
    if let Some(var_169) = &input.value {
        object.key("Value").string(var_169.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_code_configuration(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::CodeConfiguration) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_170) = &input.configuration_source {
        object.key("ConfigurationSource").string(var_170.as_str());
    }
    if let Some(var_171) = &input.code_configuration_values {
        #[allow(unused_mut)]
        let mut object_172 = object.key("CodeConfigurationValues").start_object();
        crate::json_ser::serialize_structure_crate_model_code_configuration_values(&mut object_172, var_171)?;
        object_172.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_image_configuration(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::ImageConfiguration) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_173) = &input.runtime_environment_variables {
        #[allow(unused_mut)]
        let mut object_174 = object.key("RuntimeEnvironmentVariables").start_object();
        for (key_175, value_176) in var_173 {
             {
                object_174.key(key_175.as_str()).string(value_176.as_str());
            }
        }
        object_174.finish();
    }
    if let Some(var_177) = &input.start_command {
        object.key("StartCommand").string(var_177.as_str());
    }
    if let Some(var_178) = &input.port {
        object.key("Port").string(var_178.as_str());
    }
    if let Some(var_179) = &input.runtime_environment_secrets {
        #[allow(unused_mut)]
        let mut object_180 = object.key("RuntimeEnvironmentSecrets").start_object();
        for (key_181, value_182) in var_179 {
             {
                object_180.key(key_181.as_str()).string(value_182.as_str());
            }
        }
        object_180.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_code_configuration_values(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::CodeConfigurationValues) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_183) = &input.runtime {
        object.key("Runtime").string(var_183.as_str());
    }
    if let Some(var_184) = &input.build_command {
        object.key("BuildCommand").string(var_184.as_str());
    }
    if let Some(var_185) = &input.start_command {
        object.key("StartCommand").string(var_185.as_str());
    }
    if let Some(var_186) = &input.port {
        object.key("Port").string(var_186.as_str());
    }
    if let Some(var_187) = &input.runtime_environment_variables {
        #[allow(unused_mut)]
        let mut object_188 = object.key("RuntimeEnvironmentVariables").start_object();
        for (key_189, value_190) in var_187 {
             {
                object_188.key(key_189.as_str()).string(value_190.as_str());
            }
        }
        object_188.finish();
    }
    if let Some(var_191) = &input.runtime_environment_secrets {
        #[allow(unused_mut)]
        let mut object_192 = object.key("RuntimeEnvironmentSecrets").start_object();
        for (key_193, value_194) in var_191 {
             {
                object_192.key(key_193.as_str()).string(value_194.as_str());
            }
        }
        object_192.finish();
    }
    Ok(())
}

