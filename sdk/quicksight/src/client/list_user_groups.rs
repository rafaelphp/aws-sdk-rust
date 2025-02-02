// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListUserGroups`](crate::operation::list_user_groups::builders::ListUserGroupsFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_user_groups::builders::ListUserGroupsFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`user_name(impl Into<String>)`](crate::operation::list_user_groups::builders::ListUserGroupsFluentBuilder::user_name) / [`set_user_name(Option<String>)`](crate::operation::list_user_groups::builders::ListUserGroupsFluentBuilder::set_user_name): <p>The Amazon QuickSight user name that you want to list group memberships for.</p>
    ///   - [`aws_account_id(impl Into<String>)`](crate::operation::list_user_groups::builders::ListUserGroupsFluentBuilder::aws_account_id) / [`set_aws_account_id(Option<String>)`](crate::operation::list_user_groups::builders::ListUserGroupsFluentBuilder::set_aws_account_id): <p>The Amazon Web Services account ID that the user is in. Currently, you use the ID for the Amazon Web Services account that contains your Amazon QuickSight account.</p>
    ///   - [`namespace(impl Into<String>)`](crate::operation::list_user_groups::builders::ListUserGroupsFluentBuilder::namespace) / [`set_namespace(Option<String>)`](crate::operation::list_user_groups::builders::ListUserGroupsFluentBuilder::set_namespace): <p>The namespace. Currently, you should set this to <code>default</code>.</p>
    ///   - [`next_token(impl Into<String>)`](crate::operation::list_user_groups::builders::ListUserGroupsFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_user_groups::builders::ListUserGroupsFluentBuilder::set_next_token): <p>A pagination token that can be used in a subsequent request.</p>
    ///   - [`max_results(i32)`](crate::operation::list_user_groups::builders::ListUserGroupsFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_user_groups::builders::ListUserGroupsFluentBuilder::set_max_results): <p>The maximum number of results to return from this request.</p>
    /// - On success, responds with [`ListUserGroupsOutput`](crate::operation::list_user_groups::ListUserGroupsOutput) with field(s):
    ///   - [`group_list(Option<Vec<Group>>)`](crate::operation::list_user_groups::ListUserGroupsOutput::group_list): <p>The list of groups the user is a member of.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::list_user_groups::ListUserGroupsOutput::next_token): <p>A pagination token that can be used in a subsequent request.</p>
    ///   - [`request_id(Option<String>)`](crate::operation::list_user_groups::ListUserGroupsOutput::request_id): <p>The Amazon Web Services request ID for this operation.</p>
    ///   - [`status(i32)`](crate::operation::list_user_groups::ListUserGroupsOutput::status): <p>The HTTP status of the request.</p>
    /// - On failure, responds with [`SdkError<ListUserGroupsError>`](crate::operation::list_user_groups::ListUserGroupsError)
    pub fn list_user_groups(&self) -> crate::operation::list_user_groups::builders::ListUserGroupsFluentBuilder {
        crate::operation::list_user_groups::builders::ListUserGroupsFluentBuilder::new(self.handle.clone())
    }
}
