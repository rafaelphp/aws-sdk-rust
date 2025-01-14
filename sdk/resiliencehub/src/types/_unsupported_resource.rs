// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Defines a resource that is not supported by Resilience Hub.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct UnsupportedResource {
    /// <p>The logical resource identifier for the unsupported resource.</p>
    pub logical_resource_id: ::std::option::Option<crate::types::LogicalResourceId>,
    /// <p>The physical resource identifier for the unsupported resource.</p>
    pub physical_resource_id: ::std::option::Option<crate::types::PhysicalResourceId>,
    /// <p>The type of resource.</p>
    pub resource_type: ::std::option::Option<::std::string::String>,
    /// <p>The status of the unsupported resource.</p>
    pub unsupported_resource_status: ::std::option::Option<::std::string::String>,
}
impl UnsupportedResource {
    /// <p>The logical resource identifier for the unsupported resource.</p>
    pub fn logical_resource_id(&self) -> ::std::option::Option<&crate::types::LogicalResourceId> {
        self.logical_resource_id.as_ref()
    }
    /// <p>The physical resource identifier for the unsupported resource.</p>
    pub fn physical_resource_id(&self) -> ::std::option::Option<&crate::types::PhysicalResourceId> {
        self.physical_resource_id.as_ref()
    }
    /// <p>The type of resource.</p>
    pub fn resource_type(&self) -> ::std::option::Option<&str> {
        self.resource_type.as_deref()
    }
    /// <p>The status of the unsupported resource.</p>
    pub fn unsupported_resource_status(&self) -> ::std::option::Option<&str> {
        self.unsupported_resource_status.as_deref()
    }
}
impl UnsupportedResource {
    /// Creates a new builder-style object to manufacture [`UnsupportedResource`](crate::types::UnsupportedResource).
    pub fn builder() -> crate::types::builders::UnsupportedResourceBuilder {
        crate::types::builders::UnsupportedResourceBuilder::default()
    }
}

/// A builder for [`UnsupportedResource`](crate::types::UnsupportedResource).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct UnsupportedResourceBuilder {
    pub(crate) logical_resource_id: ::std::option::Option<crate::types::LogicalResourceId>,
    pub(crate) physical_resource_id: ::std::option::Option<crate::types::PhysicalResourceId>,
    pub(crate) resource_type: ::std::option::Option<::std::string::String>,
    pub(crate) unsupported_resource_status: ::std::option::Option<::std::string::String>,
}
impl UnsupportedResourceBuilder {
    /// <p>The logical resource identifier for the unsupported resource.</p>
    pub fn logical_resource_id(mut self, input: crate::types::LogicalResourceId) -> Self {
        self.logical_resource_id = ::std::option::Option::Some(input);
        self
    }
    /// <p>The logical resource identifier for the unsupported resource.</p>
    pub fn set_logical_resource_id(mut self, input: ::std::option::Option<crate::types::LogicalResourceId>) -> Self {
        self.logical_resource_id = input;
        self
    }
    /// <p>The logical resource identifier for the unsupported resource.</p>
    pub fn get_logical_resource_id(&self) -> &::std::option::Option<crate::types::LogicalResourceId> {
        &self.logical_resource_id
    }
    /// <p>The physical resource identifier for the unsupported resource.</p>
    pub fn physical_resource_id(mut self, input: crate::types::PhysicalResourceId) -> Self {
        self.physical_resource_id = ::std::option::Option::Some(input);
        self
    }
    /// <p>The physical resource identifier for the unsupported resource.</p>
    pub fn set_physical_resource_id(mut self, input: ::std::option::Option<crate::types::PhysicalResourceId>) -> Self {
        self.physical_resource_id = input;
        self
    }
    /// <p>The physical resource identifier for the unsupported resource.</p>
    pub fn get_physical_resource_id(&self) -> &::std::option::Option<crate::types::PhysicalResourceId> {
        &self.physical_resource_id
    }
    /// <p>The type of resource.</p>
    pub fn resource_type(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.resource_type = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The type of resource.</p>
    pub fn set_resource_type(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.resource_type = input;
        self
    }
    /// <p>The type of resource.</p>
    pub fn get_resource_type(&self) -> &::std::option::Option<::std::string::String> {
        &self.resource_type
    }
    /// <p>The status of the unsupported resource.</p>
    pub fn unsupported_resource_status(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.unsupported_resource_status = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The status of the unsupported resource.</p>
    pub fn set_unsupported_resource_status(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.unsupported_resource_status = input;
        self
    }
    /// <p>The status of the unsupported resource.</p>
    pub fn get_unsupported_resource_status(&self) -> &::std::option::Option<::std::string::String> {
        &self.unsupported_resource_status
    }
    /// Consumes the builder and constructs a [`UnsupportedResource`](crate::types::UnsupportedResource).
    pub fn build(self) -> crate::types::UnsupportedResource {
        crate::types::UnsupportedResource {
            logical_resource_id: self.logical_resource_id,
            physical_resource_id: self.physical_resource_id,
            resource_type: self.resource_type,
            unsupported_resource_status: self.unsupported_resource_status,
        }
    }
}
