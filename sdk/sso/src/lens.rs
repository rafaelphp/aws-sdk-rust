// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn reflens_structure_crate_output_list_account_roles_output_next_token(input: &crate::output::ListAccountRolesOutput) -> std::option::Option<& std::string::String> {
                    let input = match &input.next_token {
                            None => return None,
                            Some(t) => t
                        };
Some(input)
                }

pub(crate) fn reflens_structure_crate_output_list_accounts_output_next_token(input: &crate::output::ListAccountsOutput) -> std::option::Option<& std::string::String> {
                    let input = match &input.next_token {
                            None => return None,
                            Some(t) => t
                        };
Some(input)
                }

pub(crate) fn lens_structure_crate_output_list_account_roles_output_role_list(input: crate::output::ListAccountRolesOutput) -> std::option::Option<std::vec::Vec<crate::model::RoleInfo>> {
                    let input = match input.role_list {
                            None => return None,
                            Some(t) => t
                        };
Some(input)
                }

pub(crate) fn lens_structure_crate_output_list_accounts_output_account_list(input: crate::output::ListAccountsOutput) -> std::option::Option<std::vec::Vec<crate::model::AccountInfo>> {
                    let input = match input.account_list {
                            None => return None,
                            Some(t) => t
                        };
Some(input)
                }

