// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p> Describes an execution job for an Amplify app. </p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct Job {
    /// <p> Describes the summary for an execution job for an Amplify app. </p>
    pub summary: ::std::option::Option<crate::types::JobSummary>,
    /// <p> The execution steps for an execution job, for an Amplify app. </p>
    pub steps: ::std::option::Option<::std::vec::Vec<crate::types::Step>>,
}
impl Job {
    /// <p> Describes the summary for an execution job for an Amplify app. </p>
    pub fn summary(&self) -> ::std::option::Option<&crate::types::JobSummary> {
        self.summary.as_ref()
    }
    /// <p> The execution steps for an execution job, for an Amplify app. </p>
    pub fn steps(&self) -> ::std::option::Option<&[crate::types::Step]> {
        self.steps.as_deref()
    }
}
impl Job {
    /// Creates a new builder-style object to manufacture [`Job`](crate::types::Job).
    pub fn builder() -> crate::types::builders::JobBuilder {
        crate::types::builders::JobBuilder::default()
    }
}

/// A builder for [`Job`](crate::types::Job).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct JobBuilder {
    pub(crate) summary: ::std::option::Option<crate::types::JobSummary>,
    pub(crate) steps: ::std::option::Option<::std::vec::Vec<crate::types::Step>>,
}
impl JobBuilder {
    /// <p> Describes the summary for an execution job for an Amplify app. </p>
    pub fn summary(mut self, input: crate::types::JobSummary) -> Self {
        self.summary = ::std::option::Option::Some(input);
        self
    }
    /// <p> Describes the summary for an execution job for an Amplify app. </p>
    pub fn set_summary(mut self, input: ::std::option::Option<crate::types::JobSummary>) -> Self {
        self.summary = input;
        self
    }
    /// <p> Describes the summary for an execution job for an Amplify app. </p>
    pub fn get_summary(&self) -> &::std::option::Option<crate::types::JobSummary> {
        &self.summary
    }
    /// Appends an item to `steps`.
    ///
    /// To override the contents of this collection use [`set_steps`](Self::set_steps).
    ///
    /// <p> The execution steps for an execution job, for an Amplify app. </p>
    pub fn steps(mut self, input: crate::types::Step) -> Self {
        let mut v = self.steps.unwrap_or_default();
        v.push(input);
        self.steps = ::std::option::Option::Some(v);
        self
    }
    /// <p> The execution steps for an execution job, for an Amplify app. </p>
    pub fn set_steps(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Step>>) -> Self {
        self.steps = input;
        self
    }
    /// <p> The execution steps for an execution job, for an Amplify app. </p>
    pub fn get_steps(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Step>> {
        &self.steps
    }
    /// Consumes the builder and constructs a [`Job`](crate::types::Job).
    pub fn build(self) -> crate::types::Job {
        crate::types::Job {
            summary: self.summary,
            steps: self.steps,
        }
    }
}
