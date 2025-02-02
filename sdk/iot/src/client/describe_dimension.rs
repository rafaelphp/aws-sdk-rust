// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeDimension`](crate::operation::describe_dimension::builders::DescribeDimensionFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`name(impl Into<String>)`](crate::operation::describe_dimension::builders::DescribeDimensionFluentBuilder::name) / [`set_name(Option<String>)`](crate::operation::describe_dimension::builders::DescribeDimensionFluentBuilder::set_name): <p>The unique identifier for the dimension.</p>
    /// - On success, responds with [`DescribeDimensionOutput`](crate::operation::describe_dimension::DescribeDimensionOutput) with field(s):
    ///   - [`name(Option<String>)`](crate::operation::describe_dimension::DescribeDimensionOutput::name): <p>The unique identifier for the dimension.</p>
    ///   - [`arn(Option<String>)`](crate::operation::describe_dimension::DescribeDimensionOutput::arn): <p>The Amazon Resource Name (ARN) for the dimension.</p>
    ///   - [`r#type(Option<DimensionType>)`](crate::operation::describe_dimension::DescribeDimensionOutput::type): <p>The type of the dimension.</p>
    ///   - [`string_values(Option<Vec<String>>)`](crate::operation::describe_dimension::DescribeDimensionOutput::string_values): <p>The value or list of values used to scope the dimension. For example, for topic filters, this is the pattern used to match the MQTT topic name.</p>
    ///   - [`creation_date(Option<DateTime>)`](crate::operation::describe_dimension::DescribeDimensionOutput::creation_date): <p>The date the dimension was created.</p>
    ///   - [`last_modified_date(Option<DateTime>)`](crate::operation::describe_dimension::DescribeDimensionOutput::last_modified_date): <p>The date the dimension was last modified.</p>
    /// - On failure, responds with [`SdkError<DescribeDimensionError>`](crate::operation::describe_dimension::DescribeDimensionError)
    pub fn describe_dimension(&self) -> crate::operation::describe_dimension::builders::DescribeDimensionFluentBuilder {
        crate::operation::describe_dimension::builders::DescribeDimensionFluentBuilder::new(self.handle.clone())
    }
}
