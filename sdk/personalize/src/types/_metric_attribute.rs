// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Contains information on a metric that a metric attribution reports on. For more information, see <a href="https://docs.aws.amazon.com/personalize/latest/dg/measuring-recommendation-impact.html">Measuring impact of recommendations</a>.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct MetricAttribute {
    /// <p>The metric's event type.</p>
    pub event_type: ::std::option::Option<::std::string::String>,
    /// <p>The metric's name. The name helps you identify the metric in Amazon CloudWatch or Amazon S3.</p>
    pub metric_name: ::std::option::Option<::std::string::String>,
    /// <p>The attribute's expression. Available functions are <code>SUM()</code> or <code>SAMPLECOUNT()</code>. For SUM() functions, provide the dataset type (either Interactions or Items) and column to sum as a parameter. For example SUM(Items.PRICE).</p>
    pub expression: ::std::option::Option<::std::string::String>,
}
impl MetricAttribute {
    /// <p>The metric's event type.</p>
    pub fn event_type(&self) -> ::std::option::Option<&str> {
        self.event_type.as_deref()
    }
    /// <p>The metric's name. The name helps you identify the metric in Amazon CloudWatch or Amazon S3.</p>
    pub fn metric_name(&self) -> ::std::option::Option<&str> {
        self.metric_name.as_deref()
    }
    /// <p>The attribute's expression. Available functions are <code>SUM()</code> or <code>SAMPLECOUNT()</code>. For SUM() functions, provide the dataset type (either Interactions or Items) and column to sum as a parameter. For example SUM(Items.PRICE).</p>
    pub fn expression(&self) -> ::std::option::Option<&str> {
        self.expression.as_deref()
    }
}
impl MetricAttribute {
    /// Creates a new builder-style object to manufacture [`MetricAttribute`](crate::types::MetricAttribute).
    pub fn builder() -> crate::types::builders::MetricAttributeBuilder {
        crate::types::builders::MetricAttributeBuilder::default()
    }
}

/// A builder for [`MetricAttribute`](crate::types::MetricAttribute).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct MetricAttributeBuilder {
    pub(crate) event_type: ::std::option::Option<::std::string::String>,
    pub(crate) metric_name: ::std::option::Option<::std::string::String>,
    pub(crate) expression: ::std::option::Option<::std::string::String>,
}
impl MetricAttributeBuilder {
    /// <p>The metric's event type.</p>
    pub fn event_type(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.event_type = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The metric's event type.</p>
    pub fn set_event_type(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.event_type = input;
        self
    }
    /// <p>The metric's event type.</p>
    pub fn get_event_type(&self) -> &::std::option::Option<::std::string::String> {
        &self.event_type
    }
    /// <p>The metric's name. The name helps you identify the metric in Amazon CloudWatch or Amazon S3.</p>
    pub fn metric_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.metric_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The metric's name. The name helps you identify the metric in Amazon CloudWatch or Amazon S3.</p>
    pub fn set_metric_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.metric_name = input;
        self
    }
    /// <p>The metric's name. The name helps you identify the metric in Amazon CloudWatch or Amazon S3.</p>
    pub fn get_metric_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.metric_name
    }
    /// <p>The attribute's expression. Available functions are <code>SUM()</code> or <code>SAMPLECOUNT()</code>. For SUM() functions, provide the dataset type (either Interactions or Items) and column to sum as a parameter. For example SUM(Items.PRICE).</p>
    pub fn expression(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.expression = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The attribute's expression. Available functions are <code>SUM()</code> or <code>SAMPLECOUNT()</code>. For SUM() functions, provide the dataset type (either Interactions or Items) and column to sum as a parameter. For example SUM(Items.PRICE).</p>
    pub fn set_expression(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.expression = input;
        self
    }
    /// <p>The attribute's expression. Available functions are <code>SUM()</code> or <code>SAMPLECOUNT()</code>. For SUM() functions, provide the dataset type (either Interactions or Items) and column to sum as a parameter. For example SUM(Items.PRICE).</p>
    pub fn get_expression(&self) -> &::std::option::Option<::std::string::String> {
        &self.expression
    }
    /// Consumes the builder and constructs a [`MetricAttribute`](crate::types::MetricAttribute).
    pub fn build(self) -> crate::types::MetricAttribute {
        crate::types::MetricAttribute {
            event_type: self.event_type,
            metric_name: self.metric_name,
            expression: self.expression,
        }
    }
}
