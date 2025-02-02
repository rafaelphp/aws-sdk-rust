// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Information about an additional property that describes a resource, that you can optionally include in the view. This lets you view that property in search results, and filter your search results based on the value of the property.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct IncludedProperty {
    /// <p>The name of the property that is included in this view.</p>
    /// <p>You can specify the following property names for this field:</p>
    /// <ul>
    /// <li> <p> <code>Tags</code> </p> </li>
    /// </ul>
    pub name: ::std::option::Option<::std::string::String>,
}
impl IncludedProperty {
    /// <p>The name of the property that is included in this view.</p>
    /// <p>You can specify the following property names for this field:</p>
    /// <ul>
    /// <li> <p> <code>Tags</code> </p> </li>
    /// </ul>
    pub fn name(&self) -> ::std::option::Option<&str> {
        self.name.as_deref()
    }
}
impl IncludedProperty {
    /// Creates a new builder-style object to manufacture [`IncludedProperty`](crate::types::IncludedProperty).
    pub fn builder() -> crate::types::builders::IncludedPropertyBuilder {
        crate::types::builders::IncludedPropertyBuilder::default()
    }
}

/// A builder for [`IncludedProperty`](crate::types::IncludedProperty).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct IncludedPropertyBuilder {
    pub(crate) name: ::std::option::Option<::std::string::String>,
}
impl IncludedPropertyBuilder {
    /// <p>The name of the property that is included in this view.</p>
    /// <p>You can specify the following property names for this field:</p>
    /// <ul>
    /// <li> <p> <code>Tags</code> </p> </li>
    /// </ul>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the property that is included in this view.</p>
    /// <p>You can specify the following property names for this field:</p>
    /// <ul>
    /// <li> <p> <code>Tags</code> </p> </li>
    /// </ul>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// <p>The name of the property that is included in this view.</p>
    /// <p>You can specify the following property names for this field:</p>
    /// <ul>
    /// <li> <p> <code>Tags</code> </p> </li>
    /// </ul>
    pub fn get_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.name
    }
    /// Consumes the builder and constructs a [`IncludedProperty`](crate::types::IncludedProperty).
    pub fn build(self) -> crate::types::IncludedProperty {
        crate::types::IncludedProperty { name: self.name }
    }
}
