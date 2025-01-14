// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreateLongTermPricing`](crate::operation::create_long_term_pricing::builders::CreateLongTermPricingFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`long_term_pricing_type(LongTermPricingType)`](crate::operation::create_long_term_pricing::builders::CreateLongTermPricingFluentBuilder::long_term_pricing_type) / [`set_long_term_pricing_type(Option<LongTermPricingType>)`](crate::operation::create_long_term_pricing::builders::CreateLongTermPricingFluentBuilder::set_long_term_pricing_type): <p>The type of long-term pricing option you want for the device, either 1-year or 3-year long-term pricing.</p>
    ///   - [`is_long_term_pricing_auto_renew(bool)`](crate::operation::create_long_term_pricing::builders::CreateLongTermPricingFluentBuilder::is_long_term_pricing_auto_renew) / [`set_is_long_term_pricing_auto_renew(Option<bool>)`](crate::operation::create_long_term_pricing::builders::CreateLongTermPricingFluentBuilder::set_is_long_term_pricing_auto_renew): <p>Specifies whether the current long-term pricing type for the device should be renewed.</p>
    ///   - [`snowball_type(SnowballType)`](crate::operation::create_long_term_pricing::builders::CreateLongTermPricingFluentBuilder::snowball_type) / [`set_snowball_type(Option<SnowballType>)`](crate::operation::create_long_term_pricing::builders::CreateLongTermPricingFluentBuilder::set_snowball_type): <p>The type of Snow Family devices to use for the long-term pricing job.</p>
    /// - On success, responds with [`CreateLongTermPricingOutput`](crate::operation::create_long_term_pricing::CreateLongTermPricingOutput) with field(s):
    ///   - [`long_term_pricing_id(Option<String>)`](crate::operation::create_long_term_pricing::CreateLongTermPricingOutput::long_term_pricing_id): <p>The ID of the long-term pricing type for the device.</p>
    /// - On failure, responds with [`SdkError<CreateLongTermPricingError>`](crate::operation::create_long_term_pricing::CreateLongTermPricingError)
    pub fn create_long_term_pricing(&self) -> crate::operation::create_long_term_pricing::builders::CreateLongTermPricingFluentBuilder {
        crate::operation::create_long_term_pricing::builders::CreateLongTermPricingFluentBuilder::new(self.handle.clone())
    }
}
