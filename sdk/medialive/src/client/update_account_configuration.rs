// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdateAccountConfiguration`](crate::operation::update_account_configuration::builders::UpdateAccountConfigurationFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`account_configuration(AccountConfiguration)`](crate::operation::update_account_configuration::builders::UpdateAccountConfigurationFluentBuilder::account_configuration) / [`set_account_configuration(Option<AccountConfiguration>)`](crate::operation::update_account_configuration::builders::UpdateAccountConfigurationFluentBuilder::set_account_configuration): Placeholder documentation for AccountConfiguration
    /// - On success, responds with [`UpdateAccountConfigurationOutput`](crate::operation::update_account_configuration::UpdateAccountConfigurationOutput) with field(s):
    ///   - [`account_configuration(Option<AccountConfiguration>)`](crate::operation::update_account_configuration::UpdateAccountConfigurationOutput::account_configuration): Placeholder documentation for AccountConfiguration
    /// - On failure, responds with [`SdkError<UpdateAccountConfigurationError>`](crate::operation::update_account_configuration::UpdateAccountConfigurationError)
    pub fn update_account_configuration(&self) -> crate::operation::update_account_configuration::builders::UpdateAccountConfigurationFluentBuilder {
        crate::operation::update_account_configuration::builders::UpdateAccountConfigurationFluentBuilder::new(self.handle.clone())
    }
}
