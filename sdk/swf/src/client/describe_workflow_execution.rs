// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeWorkflowExecution`](crate::operation::describe_workflow_execution::builders::DescribeWorkflowExecutionFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`domain(impl Into<String>)`](crate::operation::describe_workflow_execution::builders::DescribeWorkflowExecutionFluentBuilder::domain) / [`set_domain(Option<String>)`](crate::operation::describe_workflow_execution::builders::DescribeWorkflowExecutionFluentBuilder::set_domain): <p>The name of the domain containing the workflow execution.</p>
    ///   - [`execution(WorkflowExecution)`](crate::operation::describe_workflow_execution::builders::DescribeWorkflowExecutionFluentBuilder::execution) / [`set_execution(Option<WorkflowExecution>)`](crate::operation::describe_workflow_execution::builders::DescribeWorkflowExecutionFluentBuilder::set_execution): <p>The workflow execution to describe.</p>
    /// - On success, responds with [`DescribeWorkflowExecutionOutput`](crate::operation::describe_workflow_execution::DescribeWorkflowExecutionOutput) with field(s):
    ///   - [`execution_info(Option<WorkflowExecutionInfo>)`](crate::operation::describe_workflow_execution::DescribeWorkflowExecutionOutput::execution_info): <p>Information about the workflow execution.</p>
    ///   - [`execution_configuration(Option<WorkflowExecutionConfiguration>)`](crate::operation::describe_workflow_execution::DescribeWorkflowExecutionOutput::execution_configuration): <p>The configuration settings for this workflow execution including timeout values, tasklist etc.</p>
    ///   - [`open_counts(Option<WorkflowExecutionOpenCounts>)`](crate::operation::describe_workflow_execution::DescribeWorkflowExecutionOutput::open_counts): <p>The number of tasks for this workflow execution. This includes open and closed tasks of all types.</p>
    ///   - [`latest_activity_task_timestamp(Option<DateTime>)`](crate::operation::describe_workflow_execution::DescribeWorkflowExecutionOutput::latest_activity_task_timestamp): <p>The time when the last activity task was scheduled for this workflow execution. You can use this information to determine if the workflow has not made progress for an unusually long period of time and might require a corrective action.</p>
    ///   - [`latest_execution_context(Option<String>)`](crate::operation::describe_workflow_execution::DescribeWorkflowExecutionOutput::latest_execution_context): <p>The latest executionContext provided by the decider for this workflow execution. A decider can provide an executionContext (a free-form string) when closing a decision task using <code>RespondDecisionTaskCompleted</code>.</p>
    /// - On failure, responds with [`SdkError<DescribeWorkflowExecutionError>`](crate::operation::describe_workflow_execution::DescribeWorkflowExecutionError)
    pub fn describe_workflow_execution(&self) -> crate::operation::describe_workflow_execution::builders::DescribeWorkflowExecutionFluentBuilder {
        crate::operation::describe_workflow_execution::builders::DescribeWorkflowExecutionFluentBuilder::new(self.handle.clone())
    }
}
