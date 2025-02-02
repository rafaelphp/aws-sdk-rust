// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListEulas`](crate::operation::list_eulas::builders::ListEulasFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_eulas::builders::ListEulasFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`eula_ids(impl Into<String>)`](crate::operation::list_eulas::builders::ListEulasFluentBuilder::eula_ids) / [`set_eula_ids(Option<Vec<String>>)`](crate::operation::list_eulas::builders::ListEulasFluentBuilder::set_eula_ids): <p>The list of EULA IDs that should be returned</p>
    ///   - [`next_token(impl Into<String>)`](crate::operation::list_eulas::builders::ListEulasFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_eulas::builders::ListEulasFluentBuilder::set_next_token): <p>The token for the next set of results, or null if there are no more results.</p>
    /// - On success, responds with [`ListEulasOutput`](crate::operation::list_eulas::ListEulasOutput) with field(s):
    ///   - [`eulas(Option<Vec<Eula>>)`](crate::operation::list_eulas::ListEulasOutput::eulas): <p>A collection of EULA resources.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::list_eulas::ListEulasOutput::next_token): <p>The token for the next set of results, or null if there are no more results.</p>
    /// - On failure, responds with [`SdkError<ListEulasError>`](crate::operation::list_eulas::ListEulasError)
    pub fn list_eulas(&self) -> crate::operation::list_eulas::builders::ListEulasFluentBuilder {
        crate::operation::list_eulas::builders::ListEulasFluentBuilder::new(self.handle.clone())
    }
}
