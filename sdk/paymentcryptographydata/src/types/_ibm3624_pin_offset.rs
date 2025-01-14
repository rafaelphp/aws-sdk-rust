// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Pparameters that are required to generate or verify Ibm3624 PIN offset PIN.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct Ibm3624PinOffset {
    /// <p>The encrypted PIN block data. According to ISO 9564 standard, a PIN Block is an encoded representation of a payment card Personal Account Number (PAN) and the cardholder Personal Identification Number (PIN).</p>
    pub encrypted_pin_block: ::std::option::Option<::std::string::String>,
    /// <p>The decimalization table to use for IBM 3624 PIN algorithm. The table is used to convert the algorithm intermediate result from hexadecimal characters to decimal.</p>
    pub decimalization_table: ::std::option::Option<::std::string::String>,
    /// <p>The padding character for validation data.</p>
    pub pin_validation_data_pad_character: ::std::option::Option<::std::string::String>,
    /// <p>The unique data for cardholder identification.</p>
    pub pin_validation_data: ::std::option::Option<::std::string::String>,
}
impl Ibm3624PinOffset {
    /// <p>The encrypted PIN block data. According to ISO 9564 standard, a PIN Block is an encoded representation of a payment card Personal Account Number (PAN) and the cardholder Personal Identification Number (PIN).</p>
    pub fn encrypted_pin_block(&self) -> ::std::option::Option<&str> {
        self.encrypted_pin_block.as_deref()
    }
    /// <p>The decimalization table to use for IBM 3624 PIN algorithm. The table is used to convert the algorithm intermediate result from hexadecimal characters to decimal.</p>
    pub fn decimalization_table(&self) -> ::std::option::Option<&str> {
        self.decimalization_table.as_deref()
    }
    /// <p>The padding character for validation data.</p>
    pub fn pin_validation_data_pad_character(&self) -> ::std::option::Option<&str> {
        self.pin_validation_data_pad_character.as_deref()
    }
    /// <p>The unique data for cardholder identification.</p>
    pub fn pin_validation_data(&self) -> ::std::option::Option<&str> {
        self.pin_validation_data.as_deref()
    }
}
impl Ibm3624PinOffset {
    /// Creates a new builder-style object to manufacture [`Ibm3624PinOffset`](crate::types::Ibm3624PinOffset).
    pub fn builder() -> crate::types::builders::Ibm3624PinOffsetBuilder {
        crate::types::builders::Ibm3624PinOffsetBuilder::default()
    }
}

/// A builder for [`Ibm3624PinOffset`](crate::types::Ibm3624PinOffset).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct Ibm3624PinOffsetBuilder {
    pub(crate) encrypted_pin_block: ::std::option::Option<::std::string::String>,
    pub(crate) decimalization_table: ::std::option::Option<::std::string::String>,
    pub(crate) pin_validation_data_pad_character: ::std::option::Option<::std::string::String>,
    pub(crate) pin_validation_data: ::std::option::Option<::std::string::String>,
}
impl Ibm3624PinOffsetBuilder {
    /// <p>The encrypted PIN block data. According to ISO 9564 standard, a PIN Block is an encoded representation of a payment card Personal Account Number (PAN) and the cardholder Personal Identification Number (PIN).</p>
    pub fn encrypted_pin_block(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.encrypted_pin_block = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The encrypted PIN block data. According to ISO 9564 standard, a PIN Block is an encoded representation of a payment card Personal Account Number (PAN) and the cardholder Personal Identification Number (PIN).</p>
    pub fn set_encrypted_pin_block(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.encrypted_pin_block = input;
        self
    }
    /// <p>The encrypted PIN block data. According to ISO 9564 standard, a PIN Block is an encoded representation of a payment card Personal Account Number (PAN) and the cardholder Personal Identification Number (PIN).</p>
    pub fn get_encrypted_pin_block(&self) -> &::std::option::Option<::std::string::String> {
        &self.encrypted_pin_block
    }
    /// <p>The decimalization table to use for IBM 3624 PIN algorithm. The table is used to convert the algorithm intermediate result from hexadecimal characters to decimal.</p>
    pub fn decimalization_table(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.decimalization_table = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The decimalization table to use for IBM 3624 PIN algorithm. The table is used to convert the algorithm intermediate result from hexadecimal characters to decimal.</p>
    pub fn set_decimalization_table(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.decimalization_table = input;
        self
    }
    /// <p>The decimalization table to use for IBM 3624 PIN algorithm. The table is used to convert the algorithm intermediate result from hexadecimal characters to decimal.</p>
    pub fn get_decimalization_table(&self) -> &::std::option::Option<::std::string::String> {
        &self.decimalization_table
    }
    /// <p>The padding character for validation data.</p>
    pub fn pin_validation_data_pad_character(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.pin_validation_data_pad_character = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The padding character for validation data.</p>
    pub fn set_pin_validation_data_pad_character(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.pin_validation_data_pad_character = input;
        self
    }
    /// <p>The padding character for validation data.</p>
    pub fn get_pin_validation_data_pad_character(&self) -> &::std::option::Option<::std::string::String> {
        &self.pin_validation_data_pad_character
    }
    /// <p>The unique data for cardholder identification.</p>
    pub fn pin_validation_data(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.pin_validation_data = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The unique data for cardholder identification.</p>
    pub fn set_pin_validation_data(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.pin_validation_data = input;
        self
    }
    /// <p>The unique data for cardholder identification.</p>
    pub fn get_pin_validation_data(&self) -> &::std::option::Option<::std::string::String> {
        &self.pin_validation_data
    }
    /// Consumes the builder and constructs a [`Ibm3624PinOffset`](crate::types::Ibm3624PinOffset).
    pub fn build(self) -> crate::types::Ibm3624PinOffset {
        crate::types::Ibm3624PinOffset {
            encrypted_pin_block: self.encrypted_pin_block,
            decimalization_table: self.decimalization_table,
            pin_validation_data_pad_character: self.pin_validation_data_pad_character,
            pin_validation_data: self.pin_validation_data,
        }
    }
}
