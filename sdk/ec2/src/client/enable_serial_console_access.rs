// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`EnableSerialConsoleAccess`](crate::operation::enable_serial_console_access::builders::EnableSerialConsoleAccessFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`dry_run(bool)`](crate::operation::enable_serial_console_access::builders::EnableSerialConsoleAccessFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::enable_serial_console_access::builders::EnableSerialConsoleAccessFluentBuilder::set_dry_run): <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    /// - On success, responds with [`EnableSerialConsoleAccessOutput`](crate::operation::enable_serial_console_access::EnableSerialConsoleAccessOutput) with field(s):
    ///   - [`serial_console_access_enabled(Option<bool>)`](crate::operation::enable_serial_console_access::EnableSerialConsoleAccessOutput::serial_console_access_enabled): <p>If <code>true</code>, access to the EC2 serial console of all instances is enabled for your account. If <code>false</code>, access to the EC2 serial console of all instances is disabled for your account.</p>
    /// - On failure, responds with [`SdkError<EnableSerialConsoleAccessError>`](crate::operation::enable_serial_console_access::EnableSerialConsoleAccessError)
    pub fn enable_serial_console_access(&self) -> crate::operation::enable_serial_console_access::builders::EnableSerialConsoleAccessFluentBuilder {
        crate::operation::enable_serial_console_access::builders::EnableSerialConsoleAccessFluentBuilder::new(self.handle.clone())
    }
}
