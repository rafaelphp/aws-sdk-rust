// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreateList`](crate::operation::create_list::builders::CreateListFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`name(impl Into<String>)`](crate::operation::create_list::builders::CreateListFluentBuilder::name) / [`set_name(Option<String>)`](crate::operation::create_list::builders::CreateListFluentBuilder::set_name): <p> The name of the list. </p>
    ///   - [`elements(impl Into<String>)`](crate::operation::create_list::builders::CreateListFluentBuilder::elements) / [`set_elements(Option<Vec<String>>)`](crate::operation::create_list::builders::CreateListFluentBuilder::set_elements): <p> The names of the elements, if providing. You can also create an empty list and add elements later using the <a href="https://docs.aws.amazon.com/frauddetector/latest/api/API_Updatelist.html">UpdateList</a> API. </p>
    ///   - [`variable_type(impl Into<String>)`](crate::operation::create_list::builders::CreateListFluentBuilder::variable_type) / [`set_variable_type(Option<String>)`](crate::operation::create_list::builders::CreateListFluentBuilder::set_variable_type): <p> The variable type of the list. You can only assign the variable type with String data type. For more information, see <a href="https://docs.aws.amazon.com/frauddetector/latest/ug/create-a-variable.html#variable-types">Variable types</a>. </p>
    ///   - [`description(impl Into<String>)`](crate::operation::create_list::builders::CreateListFluentBuilder::description) / [`set_description(Option<String>)`](crate::operation::create_list::builders::CreateListFluentBuilder::set_description): <p> The description of the list. </p>
    ///   - [`tags(Tag)`](crate::operation::create_list::builders::CreateListFluentBuilder::tags) / [`set_tags(Option<Vec<Tag>>)`](crate::operation::create_list::builders::CreateListFluentBuilder::set_tags): <p> A collection of the key and value pairs. </p>
    /// - On success, responds with [`CreateListOutput`](crate::operation::create_list::CreateListOutput)
    /// - On failure, responds with [`SdkError<CreateListError>`](crate::operation::create_list::CreateListError)
    pub fn create_list(&self) -> crate::operation::create_list::builders::CreateListFluentBuilder {
        crate::operation::create_list::builders::CreateListFluentBuilder::new(self.handle.clone())
    }
}
