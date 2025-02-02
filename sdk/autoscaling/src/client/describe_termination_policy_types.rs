// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeTerminationPolicyTypes`](crate::operation::describe_termination_policy_types::builders::DescribeTerminationPolicyTypesFluentBuilder) operation.
    ///
    /// - The fluent builder takes no input, just [`send`](crate::operation::describe_termination_policy_types::builders::DescribeTerminationPolicyTypesFluentBuilder::send) it.
    /// - On success, responds with [`DescribeTerminationPolicyTypesOutput`](crate::operation::describe_termination_policy_types::DescribeTerminationPolicyTypesOutput) with field(s):
    ///   - [`termination_policy_types(Option<Vec<String>>)`](crate::operation::describe_termination_policy_types::DescribeTerminationPolicyTypesOutput::termination_policy_types): <p>The termination policies supported by Amazon EC2 Auto Scaling: <code>OldestInstance</code>, <code>OldestLaunchConfiguration</code>, <code>NewestInstance</code>, <code>ClosestToNextInstanceHour</code>, <code>Default</code>, <code>OldestLaunchTemplate</code>, and <code>AllocationStrategy</code>.</p>
    /// - On failure, responds with [`SdkError<DescribeTerminationPolicyTypesError>`](crate::operation::describe_termination_policy_types::DescribeTerminationPolicyTypesError)
    pub fn describe_termination_policy_types(
        &self,
    ) -> crate::operation::describe_termination_policy_types::builders::DescribeTerminationPolicyTypesFluentBuilder {
        crate::operation::describe_termination_policy_types::builders::DescribeTerminationPolicyTypesFluentBuilder::new(self.handle.clone())
    }
}
