// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn reflens_structure_crate_output_get_resource_policies_output_next_token(input: &crate::output::GetResourcePoliciesOutput) -> std::option::Option<& std::string::String> {
                    let input = match &input.next_token {
                            None => return None,
                            Some(t) => t
                        };
Some(input)
                }

pub(crate) fn reflens_structure_crate_output_list_incident_records_output_next_token(input: &crate::output::ListIncidentRecordsOutput) -> std::option::Option<& std::string::String> {
                    let input = match &input.next_token {
                            None => return None,
                            Some(t) => t
                        };
Some(input)
                }

pub(crate) fn reflens_structure_crate_output_list_related_items_output_next_token(input: &crate::output::ListRelatedItemsOutput) -> std::option::Option<& std::string::String> {
                    let input = match &input.next_token {
                            None => return None,
                            Some(t) => t
                        };
Some(input)
                }

pub(crate) fn reflens_structure_crate_output_list_replication_sets_output_next_token(input: &crate::output::ListReplicationSetsOutput) -> std::option::Option<& std::string::String> {
                    let input = match &input.next_token {
                            None => return None,
                            Some(t) => t
                        };
Some(input)
                }

pub(crate) fn reflens_structure_crate_output_list_response_plans_output_next_token(input: &crate::output::ListResponsePlansOutput) -> std::option::Option<& std::string::String> {
                    let input = match &input.next_token {
                            None => return None,
                            Some(t) => t
                        };
Some(input)
                }

pub(crate) fn reflens_structure_crate_output_list_timeline_events_output_next_token(input: &crate::output::ListTimelineEventsOutput) -> std::option::Option<& std::string::String> {
                    let input = match &input.next_token {
                            None => return None,
                            Some(t) => t
                        };
Some(input)
                }

pub(crate) fn lens_structure_crate_output_get_resource_policies_output_resource_policies(input: crate::output::GetResourcePoliciesOutput) -> std::option::Option<std::vec::Vec<crate::model::ResourcePolicy>> {
                    let input = match input.resource_policies {
                            None => return None,
                            Some(t) => t
                        };
Some(input)
                }

pub(crate) fn lens_structure_crate_output_list_incident_records_output_incident_record_summaries(input: crate::output::ListIncidentRecordsOutput) -> std::option::Option<std::vec::Vec<crate::model::IncidentRecordSummary>> {
                    let input = match input.incident_record_summaries {
                            None => return None,
                            Some(t) => t
                        };
Some(input)
                }

pub(crate) fn lens_structure_crate_output_list_related_items_output_related_items(input: crate::output::ListRelatedItemsOutput) -> std::option::Option<std::vec::Vec<crate::model::RelatedItem>> {
                    let input = match input.related_items {
                            None => return None,
                            Some(t) => t
                        };
Some(input)
                }

pub(crate) fn lens_structure_crate_output_list_replication_sets_output_replication_set_arns(input: crate::output::ListReplicationSetsOutput) -> std::option::Option<std::vec::Vec<std::string::String>> {
                    let input = match input.replication_set_arns {
                            None => return None,
                            Some(t) => t
                        };
Some(input)
                }

pub(crate) fn lens_structure_crate_output_list_response_plans_output_response_plan_summaries(input: crate::output::ListResponsePlansOutput) -> std::option::Option<std::vec::Vec<crate::model::ResponsePlanSummary>> {
                    let input = match input.response_plan_summaries {
                            None => return None,
                            Some(t) => t
                        };
Some(input)
                }

pub(crate) fn lens_structure_crate_output_list_timeline_events_output_event_summaries(input: crate::output::ListTimelineEventsOutput) -> std::option::Option<std::vec::Vec<crate::model::EventSummary>> {
                    let input = match input.event_summaries {
                            None => return None,
                            Some(t) => t
                        };
Some(input)
                }

