// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn reflens_structure_crate_output_list_group_resources_output_next_token(input: &crate::output::ListGroupResourcesOutput) -> std::option::Option<& std::string::String> {
                    let input = match &input.next_token {
                            None => return None,
                            Some(t) => t
                        };
Some(input)
                }

pub(crate) fn reflens_structure_crate_output_list_groups_output_next_token(input: &crate::output::ListGroupsOutput) -> std::option::Option<& std::string::String> {
                    let input = match &input.next_token {
                            None => return None,
                            Some(t) => t
                        };
Some(input)
                }

pub(crate) fn reflens_structure_crate_output_search_resources_output_next_token(input: &crate::output::SearchResourcesOutput) -> std::option::Option<& std::string::String> {
                    let input = match &input.next_token {
                            None => return None,
                            Some(t) => t
                        };
Some(input)
                }

pub(crate) fn lens_structure_crate_output_list_group_resources_output_resource_identifiers(input: crate::output::ListGroupResourcesOutput) -> std::option::Option<std::vec::Vec<crate::model::ResourceIdentifier>> {
                    let input = match input.resource_identifiers {
                            None => return None,
                            Some(t) => t
                        };
Some(input)
                }

pub(crate) fn lens_structure_crate_output_list_groups_output_group_identifiers(input: crate::output::ListGroupsOutput) -> std::option::Option<std::vec::Vec<crate::model::GroupIdentifier>> {
                    let input = match input.group_identifiers {
                            None => return None,
                            Some(t) => t
                        };
Some(input)
                }

pub(crate) fn lens_structure_crate_output_search_resources_output_resource_identifiers(input: crate::output::SearchResourcesOutput) -> std::option::Option<std::vec::Vec<crate::model::ResourceIdentifier>> {
                    let input = match input.resource_identifiers {
                            None => return None,
                            Some(t) => t
                        };
Some(input)
                }

