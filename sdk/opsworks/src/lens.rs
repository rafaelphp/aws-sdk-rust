// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn reflens_structure_crate_output_describe_ecs_clusters_output_next_token(input: &crate::output::DescribeEcsClustersOutput) -> std::option::Option<& std::string::String> {
                    let input = match &input.next_token {
                            None => return None,
                            Some(t) => t
                        };
Some(input)
                }

pub(crate) fn lens_structure_crate_output_describe_ecs_clusters_output_ecs_clusters(input: crate::output::DescribeEcsClustersOutput) -> std::option::Option<std::vec::Vec<crate::model::EcsCluster>> {
                    let input = match input.ecs_clusters {
                            None => return None,
                            Some(t) => t
                        };
Some(input)
                }

