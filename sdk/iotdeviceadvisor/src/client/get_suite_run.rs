// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetSuiteRun`](crate::operation::get_suite_run::builders::GetSuiteRunFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`suite_definition_id(impl Into<String>)`](crate::operation::get_suite_run::builders::GetSuiteRunFluentBuilder::suite_definition_id) / [`set_suite_definition_id(Option<String>)`](crate::operation::get_suite_run::builders::GetSuiteRunFluentBuilder::set_suite_definition_id): <p>Suite definition ID for the test suite run.</p>
    ///   - [`suite_run_id(impl Into<String>)`](crate::operation::get_suite_run::builders::GetSuiteRunFluentBuilder::suite_run_id) / [`set_suite_run_id(Option<String>)`](crate::operation::get_suite_run::builders::GetSuiteRunFluentBuilder::set_suite_run_id): <p>Suite run ID for the test suite run.</p>
    /// - On success, responds with [`GetSuiteRunOutput`](crate::operation::get_suite_run::GetSuiteRunOutput) with field(s):
    ///   - [`suite_definition_id(Option<String>)`](crate::operation::get_suite_run::GetSuiteRunOutput::suite_definition_id): <p>Suite definition ID for the test suite run.</p>
    ///   - [`suite_definition_version(Option<String>)`](crate::operation::get_suite_run::GetSuiteRunOutput::suite_definition_version): <p>Suite definition version for the test suite run.</p>
    ///   - [`suite_run_id(Option<String>)`](crate::operation::get_suite_run::GetSuiteRunOutput::suite_run_id): <p>Suite run ID for the test suite run.</p>
    ///   - [`suite_run_arn(Option<String>)`](crate::operation::get_suite_run::GetSuiteRunOutput::suite_run_arn): <p>The ARN of the suite run.</p>
    ///   - [`suite_run_configuration(Option<SuiteRunConfiguration>)`](crate::operation::get_suite_run::GetSuiteRunOutput::suite_run_configuration): <p>Suite run configuration for the test suite run.</p>
    ///   - [`test_result(Option<TestResult>)`](crate::operation::get_suite_run::GetSuiteRunOutput::test_result): <p>Test results for the test suite run.</p>
    ///   - [`start_time(Option<DateTime>)`](crate::operation::get_suite_run::GetSuiteRunOutput::start_time): <p>Date (in Unix epoch time) when the test suite run started.</p>
    ///   - [`end_time(Option<DateTime>)`](crate::operation::get_suite_run::GetSuiteRunOutput::end_time): <p>Date (in Unix epoch time) when the test suite run ended.</p>
    ///   - [`status(Option<SuiteRunStatus>)`](crate::operation::get_suite_run::GetSuiteRunOutput::status): <p>Status for the test suite run.</p>
    ///   - [`error_reason(Option<String>)`](crate::operation::get_suite_run::GetSuiteRunOutput::error_reason): <p>Error reason for any test suite run failure.</p>
    ///   - [`tags(Option<HashMap<String, String>>)`](crate::operation::get_suite_run::GetSuiteRunOutput::tags): <p>The tags attached to the suite run.</p>
    /// - On failure, responds with [`SdkError<GetSuiteRunError>`](crate::operation::get_suite_run::GetSuiteRunError)
    pub fn get_suite_run(&self) -> crate::operation::get_suite_run::builders::GetSuiteRunFluentBuilder {
        crate::operation::get_suite_run::builders::GetSuiteRunFluentBuilder::new(self.handle.clone())
    }
}
