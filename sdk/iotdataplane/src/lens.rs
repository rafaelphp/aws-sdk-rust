// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn reflens_structure_crate_output_list_retained_messages_output_next_token(input: &crate::output::ListRetainedMessagesOutput) -> std::option::Option<& std::string::String> {
                    let input = match &input.next_token {
                            None => return None,
                            Some(t) => t
                        };
Some(input)
                }

pub(crate) fn lens_structure_crate_output_list_retained_messages_output_retained_topics(input: crate::output::ListRetainedMessagesOutput) -> std::option::Option<std::vec::Vec<crate::model::RetainedMessageSummary>> {
                    let input = match input.retained_topics {
                            None => return None,
                            Some(t) => t
                        };
Some(input)
                }

