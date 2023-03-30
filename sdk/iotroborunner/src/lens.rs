// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn reflens_structure_crate_output_list_destinations_output_next_token(input: &crate::output::ListDestinationsOutput) -> std::option::Option<& std::string::String> {
                    let input = match &input.next_token {
                            None => return None,
                            Some(t) => t
                        };
Some(input)
                }

pub(crate) fn reflens_structure_crate_output_list_sites_output_next_token(input: &crate::output::ListSitesOutput) -> std::option::Option<& std::string::String> {
                    let input = match &input.next_token {
                            None => return None,
                            Some(t) => t
                        };
Some(input)
                }

pub(crate) fn reflens_structure_crate_output_list_worker_fleets_output_next_token(input: &crate::output::ListWorkerFleetsOutput) -> std::option::Option<& std::string::String> {
                    let input = match &input.next_token {
                            None => return None,
                            Some(t) => t
                        };
Some(input)
                }

pub(crate) fn reflens_structure_crate_output_list_workers_output_next_token(input: &crate::output::ListWorkersOutput) -> std::option::Option<& std::string::String> {
                    let input = match &input.next_token {
                            None => return None,
                            Some(t) => t
                        };
Some(input)
                }

pub(crate) fn lens_structure_crate_output_list_destinations_output_destinations(input: crate::output::ListDestinationsOutput) -> std::option::Option<std::vec::Vec<crate::model::Destination>> {
                    let input = match input.destinations {
                            None => return None,
                            Some(t) => t
                        };
Some(input)
                }

pub(crate) fn lens_structure_crate_output_list_sites_output_sites(input: crate::output::ListSitesOutput) -> std::option::Option<std::vec::Vec<crate::model::Site>> {
                    let input = match input.sites {
                            None => return None,
                            Some(t) => t
                        };
Some(input)
                }

pub(crate) fn lens_structure_crate_output_list_worker_fleets_output_worker_fleets(input: crate::output::ListWorkerFleetsOutput) -> std::option::Option<std::vec::Vec<crate::model::WorkerFleet>> {
                    let input = match input.worker_fleets {
                            None => return None,
                            Some(t) => t
                        };
Some(input)
                }

pub(crate) fn lens_structure_crate_output_list_workers_output_workers(input: crate::output::ListWorkersOutput) -> std::option::Option<std::vec::Vec<crate::model::Worker>> {
                    let input = match input.workers {
                            None => return None,
                            Some(t) => t
                        };
Some(input)
                }

