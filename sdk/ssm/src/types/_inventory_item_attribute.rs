// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Attributes are the entries within the inventory item content. It contains name and value.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct InventoryItemAttribute {
    /// <p>Name of the inventory item attribute.</p>
    pub name: ::std::option::Option<::std::string::String>,
    /// <p>The data type of the inventory item attribute. </p>
    pub data_type: ::std::option::Option<crate::types::InventoryAttributeDataType>,
}
impl InventoryItemAttribute {
    /// <p>Name of the inventory item attribute.</p>
    pub fn name(&self) -> ::std::option::Option<&str> {
        self.name.as_deref()
    }
    /// <p>The data type of the inventory item attribute. </p>
    pub fn data_type(&self) -> ::std::option::Option<&crate::types::InventoryAttributeDataType> {
        self.data_type.as_ref()
    }
}
impl InventoryItemAttribute {
    /// Creates a new builder-style object to manufacture [`InventoryItemAttribute`](crate::types::InventoryItemAttribute).
    pub fn builder() -> crate::types::builders::InventoryItemAttributeBuilder {
        crate::types::builders::InventoryItemAttributeBuilder::default()
    }
}

/// A builder for [`InventoryItemAttribute`](crate::types::InventoryItemAttribute).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct InventoryItemAttributeBuilder {
    pub(crate) name: ::std::option::Option<::std::string::String>,
    pub(crate) data_type: ::std::option::Option<crate::types::InventoryAttributeDataType>,
}
impl InventoryItemAttributeBuilder {
    /// <p>Name of the inventory item attribute.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Name of the inventory item attribute.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// <p>Name of the inventory item attribute.</p>
    pub fn get_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.name
    }
    /// <p>The data type of the inventory item attribute. </p>
    pub fn data_type(mut self, input: crate::types::InventoryAttributeDataType) -> Self {
        self.data_type = ::std::option::Option::Some(input);
        self
    }
    /// <p>The data type of the inventory item attribute. </p>
    pub fn set_data_type(mut self, input: ::std::option::Option<crate::types::InventoryAttributeDataType>) -> Self {
        self.data_type = input;
        self
    }
    /// <p>The data type of the inventory item attribute. </p>
    pub fn get_data_type(&self) -> &::std::option::Option<crate::types::InventoryAttributeDataType> {
        &self.data_type
    }
    /// Consumes the builder and constructs a [`InventoryItemAttribute`](crate::types::InventoryItemAttribute).
    pub fn build(self) -> crate::types::InventoryItemAttribute {
        crate::types::InventoryItemAttribute {
            name: self.name,
            data_type: self.data_type,
        }
    }
}
