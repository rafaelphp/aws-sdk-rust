// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreateRule`](crate::operation::create_rule::builders::CreateRuleFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`rule_id(impl Into<String>)`](crate::operation::create_rule::builders::CreateRuleFluentBuilder::rule_id) / [`set_rule_id(Option<String>)`](crate::operation::create_rule::builders::CreateRuleFluentBuilder::set_rule_id): <p>The rule ID.</p>
    ///   - [`detector_id(impl Into<String>)`](crate::operation::create_rule::builders::CreateRuleFluentBuilder::detector_id) / [`set_detector_id(Option<String>)`](crate::operation::create_rule::builders::CreateRuleFluentBuilder::set_detector_id): <p>The detector ID for the rule's parent detector.</p>
    ///   - [`description(impl Into<String>)`](crate::operation::create_rule::builders::CreateRuleFluentBuilder::description) / [`set_description(Option<String>)`](crate::operation::create_rule::builders::CreateRuleFluentBuilder::set_description): <p>The rule description.</p>
    ///   - [`expression(impl Into<String>)`](crate::operation::create_rule::builders::CreateRuleFluentBuilder::expression) / [`set_expression(Option<String>)`](crate::operation::create_rule::builders::CreateRuleFluentBuilder::set_expression): <p>The rule expression.</p>
    ///   - [`language(Language)`](crate::operation::create_rule::builders::CreateRuleFluentBuilder::language) / [`set_language(Option<Language>)`](crate::operation::create_rule::builders::CreateRuleFluentBuilder::set_language): <p>The language of the rule.</p>
    ///   - [`outcomes(impl Into<String>)`](crate::operation::create_rule::builders::CreateRuleFluentBuilder::outcomes) / [`set_outcomes(Option<Vec<String>>)`](crate::operation::create_rule::builders::CreateRuleFluentBuilder::set_outcomes): <p>The outcome or outcomes returned when the rule expression matches.</p>
    ///   - [`tags(Tag)`](crate::operation::create_rule::builders::CreateRuleFluentBuilder::tags) / [`set_tags(Option<Vec<Tag>>)`](crate::operation::create_rule::builders::CreateRuleFluentBuilder::set_tags): <p>A collection of key and value pairs.</p>
    /// - On success, responds with [`CreateRuleOutput`](crate::operation::create_rule::CreateRuleOutput) with field(s):
    ///   - [`rule(Option<Rule>)`](crate::operation::create_rule::CreateRuleOutput::rule): <p>The created rule.</p>
    /// - On failure, responds with [`SdkError<CreateRuleError>`](crate::operation::create_rule::CreateRuleError)
    pub fn create_rule(&self) -> crate::operation::create_rule::builders::CreateRuleFluentBuilder {
        crate::operation::create_rule::builders::CreateRuleFluentBuilder::new(self.handle.clone())
    }
}
