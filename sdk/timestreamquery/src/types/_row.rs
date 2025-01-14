// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Represents a single row in the query results.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct Row {
    /// <p>List of data points in a single row of the result set.</p>
    pub data: ::std::option::Option<::std::vec::Vec<crate::types::Datum>>,
}
impl Row {
    /// <p>List of data points in a single row of the result set.</p>
    pub fn data(&self) -> ::std::option::Option<&[crate::types::Datum]> {
        self.data.as_deref()
    }
}
impl Row {
    /// Creates a new builder-style object to manufacture [`Row`](crate::types::Row).
    pub fn builder() -> crate::types::builders::RowBuilder {
        crate::types::builders::RowBuilder::default()
    }
}

/// A builder for [`Row`](crate::types::Row).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct RowBuilder {
    pub(crate) data: ::std::option::Option<::std::vec::Vec<crate::types::Datum>>,
}
impl RowBuilder {
    /// Appends an item to `data`.
    ///
    /// To override the contents of this collection use [`set_data`](Self::set_data).
    ///
    /// <p>List of data points in a single row of the result set.</p>
    pub fn data(mut self, input: crate::types::Datum) -> Self {
        let mut v = self.data.unwrap_or_default();
        v.push(input);
        self.data = ::std::option::Option::Some(v);
        self
    }
    /// <p>List of data points in a single row of the result set.</p>
    pub fn set_data(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Datum>>) -> Self {
        self.data = input;
        self
    }
    /// <p>List of data points in a single row of the result set.</p>
    pub fn get_data(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Datum>> {
        &self.data
    }
    /// Consumes the builder and constructs a [`Row`](crate::types::Row).
    pub fn build(self) -> crate::types::Row {
        crate::types::Row { data: self.data }
    }
}
