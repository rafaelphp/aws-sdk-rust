// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn reflens_structure_crate_output_get_server_details_output_next_token(input: &crate::output::GetServerDetailsOutput) -> std::option::Option<& std::string::String> {
                    let input = match &input.next_token {
                            None => return None,
                            Some(t) => t
                        };
Some(input)
                }

pub(crate) fn reflens_structure_crate_output_list_application_components_output_next_token(input: &crate::output::ListApplicationComponentsOutput) -> std::option::Option<& std::string::String> {
                    let input = match &input.next_token {
                            None => return None,
                            Some(t) => t
                        };
Some(input)
                }

pub(crate) fn reflens_structure_crate_output_list_collectors_output_next_token(input: &crate::output::ListCollectorsOutput) -> std::option::Option<& std::string::String> {
                    let input = match &input.next_token {
                            None => return None,
                            Some(t) => t
                        };
Some(input)
                }

pub(crate) fn reflens_structure_crate_output_list_import_file_task_output_next_token(input: &crate::output::ListImportFileTaskOutput) -> std::option::Option<& std::string::String> {
                    let input = match &input.next_token {
                            None => return None,
                            Some(t) => t
                        };
Some(input)
                }

pub(crate) fn reflens_structure_crate_output_list_servers_output_next_token(input: &crate::output::ListServersOutput) -> std::option::Option<& std::string::String> {
                    let input = match &input.next_token {
                            None => return None,
                            Some(t) => t
                        };
Some(input)
                }

pub(crate) fn lens_structure_crate_output_get_server_details_output_associated_applications(input: crate::output::GetServerDetailsOutput) -> std::option::Option<std::vec::Vec<crate::model::AssociatedApplication>> {
                    let input = match input.associated_applications {
                            None => return None,
                            Some(t) => t
                        };
Some(input)
                }

pub(crate) fn lens_structure_crate_output_list_application_components_output_application_component_infos(input: crate::output::ListApplicationComponentsOutput) -> std::option::Option<std::vec::Vec<crate::model::ApplicationComponentDetail>> {
                    let input = match input.application_component_infos {
                            None => return None,
                            Some(t) => t
                        };
Some(input)
                }

pub(crate) fn lens_structure_crate_output_list_collectors_output_collectors(input: crate::output::ListCollectorsOutput) -> std::option::Option<std::vec::Vec<crate::model::Collector>> {
                    let input = match input.collectors {
                            None => return None,
                            Some(t) => t
                        };
Some(input)
                }

pub(crate) fn lens_structure_crate_output_list_import_file_task_output_task_infos(input: crate::output::ListImportFileTaskOutput) -> std::option::Option<std::vec::Vec<crate::model::ImportFileTaskInformation>> {
                    let input = match input.task_infos {
                            None => return None,
                            Some(t) => t
                        };
Some(input)
                }

pub(crate) fn lens_structure_crate_output_list_servers_output_server_infos(input: crate::output::ListServersOutput) -> std::option::Option<std::vec::Vec<crate::model::ServerDetail>> {
                    let input = match input.server_infos {
                            None => return None,
                            Some(t) => t
                        };
Some(input)
                }

