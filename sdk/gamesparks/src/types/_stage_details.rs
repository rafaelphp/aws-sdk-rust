// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Properties that provide details of a stage.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct StageDetails {
    /// <p>The name of the stage.</p>
    pub name: ::std::option::Option<::std::string::String>,
    /// <p>The game key associated with the stage.</p>
    /// <p> The game key is a unique identifier that the game client uses to connect to the GameSparks backend. </p>
    pub game_key: ::std::option::Option<::std::string::String>,
    /// <p>The Amazon Resource Name (ARN) of the stage.</p>
    pub arn: ::std::option::Option<::std::string::String>,
    /// <p>The Amazon Resource Name (ARN) of the role used to run the game runtimes deployed to the stage.</p>
    pub role: ::std::option::Option<::std::string::String>,
    /// <p>The description of the stage.</p>
    pub description: ::std::option::Option<::std::string::String>,
    /// <p>The timestamp of when the stage was created.</p>
    pub created: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// <p>The timestamp of when the stage was last updated.</p>
    pub last_updated: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// <p>The state of the stage.</p>
    pub state: ::std::option::Option<crate::types::StageState>,
    /// <p>The tags associated with the stage.</p>
    pub tags: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>,
    /// <p>The Amazon CloudWatch log group for game runtimes deployed to the stage.</p>
    pub log_group: ::std::option::Option<::std::string::String>,
}
impl StageDetails {
    /// <p>The name of the stage.</p>
    pub fn name(&self) -> ::std::option::Option<&str> {
        self.name.as_deref()
    }
    /// <p>The game key associated with the stage.</p>
    /// <p> The game key is a unique identifier that the game client uses to connect to the GameSparks backend. </p>
    pub fn game_key(&self) -> ::std::option::Option<&str> {
        self.game_key.as_deref()
    }
    /// <p>The Amazon Resource Name (ARN) of the stage.</p>
    pub fn arn(&self) -> ::std::option::Option<&str> {
        self.arn.as_deref()
    }
    /// <p>The Amazon Resource Name (ARN) of the role used to run the game runtimes deployed to the stage.</p>
    pub fn role(&self) -> ::std::option::Option<&str> {
        self.role.as_deref()
    }
    /// <p>The description of the stage.</p>
    pub fn description(&self) -> ::std::option::Option<&str> {
        self.description.as_deref()
    }
    /// <p>The timestamp of when the stage was created.</p>
    pub fn created(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.created.as_ref()
    }
    /// <p>The timestamp of when the stage was last updated.</p>
    pub fn last_updated(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.last_updated.as_ref()
    }
    /// <p>The state of the stage.</p>
    pub fn state(&self) -> ::std::option::Option<&crate::types::StageState> {
        self.state.as_ref()
    }
    /// <p>The tags associated with the stage.</p>
    pub fn tags(&self) -> ::std::option::Option<&::std::collections::HashMap<::std::string::String, ::std::string::String>> {
        self.tags.as_ref()
    }
    /// <p>The Amazon CloudWatch log group for game runtimes deployed to the stage.</p>
    pub fn log_group(&self) -> ::std::option::Option<&str> {
        self.log_group.as_deref()
    }
}
impl StageDetails {
    /// Creates a new builder-style object to manufacture [`StageDetails`](crate::types::StageDetails).
    pub fn builder() -> crate::types::builders::StageDetailsBuilder {
        crate::types::builders::StageDetailsBuilder::default()
    }
}

/// A builder for [`StageDetails`](crate::types::StageDetails).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct StageDetailsBuilder {
    pub(crate) name: ::std::option::Option<::std::string::String>,
    pub(crate) game_key: ::std::option::Option<::std::string::String>,
    pub(crate) arn: ::std::option::Option<::std::string::String>,
    pub(crate) role: ::std::option::Option<::std::string::String>,
    pub(crate) description: ::std::option::Option<::std::string::String>,
    pub(crate) created: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) last_updated: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) state: ::std::option::Option<crate::types::StageState>,
    pub(crate) tags: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>,
    pub(crate) log_group: ::std::option::Option<::std::string::String>,
}
impl StageDetailsBuilder {
    /// <p>The name of the stage.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the stage.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// <p>The name of the stage.</p>
    pub fn get_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.name
    }
    /// <p>The game key associated with the stage.</p>
    /// <p> The game key is a unique identifier that the game client uses to connect to the GameSparks backend. </p>
    pub fn game_key(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.game_key = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The game key associated with the stage.</p>
    /// <p> The game key is a unique identifier that the game client uses to connect to the GameSparks backend. </p>
    pub fn set_game_key(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.game_key = input;
        self
    }
    /// <p>The game key associated with the stage.</p>
    /// <p> The game key is a unique identifier that the game client uses to connect to the GameSparks backend. </p>
    pub fn get_game_key(&self) -> &::std::option::Option<::std::string::String> {
        &self.game_key
    }
    /// <p>The Amazon Resource Name (ARN) of the stage.</p>
    pub fn arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the stage.</p>
    pub fn set_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.arn = input;
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the stage.</p>
    pub fn get_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.arn
    }
    /// <p>The Amazon Resource Name (ARN) of the role used to run the game runtimes deployed to the stage.</p>
    pub fn role(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.role = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the role used to run the game runtimes deployed to the stage.</p>
    pub fn set_role(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.role = input;
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the role used to run the game runtimes deployed to the stage.</p>
    pub fn get_role(&self) -> &::std::option::Option<::std::string::String> {
        &self.role
    }
    /// <p>The description of the stage.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.description = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The description of the stage.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.description = input;
        self
    }
    /// <p>The description of the stage.</p>
    pub fn get_description(&self) -> &::std::option::Option<::std::string::String> {
        &self.description
    }
    /// <p>The timestamp of when the stage was created.</p>
    pub fn created(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.created = ::std::option::Option::Some(input);
        self
    }
    /// <p>The timestamp of when the stage was created.</p>
    pub fn set_created(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.created = input;
        self
    }
    /// <p>The timestamp of when the stage was created.</p>
    pub fn get_created(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        &self.created
    }
    /// <p>The timestamp of when the stage was last updated.</p>
    pub fn last_updated(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.last_updated = ::std::option::Option::Some(input);
        self
    }
    /// <p>The timestamp of when the stage was last updated.</p>
    pub fn set_last_updated(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.last_updated = input;
        self
    }
    /// <p>The timestamp of when the stage was last updated.</p>
    pub fn get_last_updated(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        &self.last_updated
    }
    /// <p>The state of the stage.</p>
    pub fn state(mut self, input: crate::types::StageState) -> Self {
        self.state = ::std::option::Option::Some(input);
        self
    }
    /// <p>The state of the stage.</p>
    pub fn set_state(mut self, input: ::std::option::Option<crate::types::StageState>) -> Self {
        self.state = input;
        self
    }
    /// <p>The state of the stage.</p>
    pub fn get_state(&self) -> &::std::option::Option<crate::types::StageState> {
        &self.state
    }
    /// Adds a key-value pair to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>The tags associated with the stage.</p>
    pub fn tags(mut self, k: impl ::std::convert::Into<::std::string::String>, v: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut hash_map = self.tags.unwrap_or_default();
        hash_map.insert(k.into(), v.into());
        self.tags = ::std::option::Option::Some(hash_map);
        self
    }
    /// <p>The tags associated with the stage.</p>
    pub fn set_tags(mut self, input: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>) -> Self {
        self.tags = input;
        self
    }
    /// <p>The tags associated with the stage.</p>
    pub fn get_tags(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>> {
        &self.tags
    }
    /// <p>The Amazon CloudWatch log group for game runtimes deployed to the stage.</p>
    pub fn log_group(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.log_group = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon CloudWatch log group for game runtimes deployed to the stage.</p>
    pub fn set_log_group(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.log_group = input;
        self
    }
    /// <p>The Amazon CloudWatch log group for game runtimes deployed to the stage.</p>
    pub fn get_log_group(&self) -> &::std::option::Option<::std::string::String> {
        &self.log_group
    }
    /// Consumes the builder and constructs a [`StageDetails`](crate::types::StageDetails).
    pub fn build(self) -> crate::types::StageDetails {
        crate::types::StageDetails {
            name: self.name,
            game_key: self.game_key,
            arn: self.arn,
            role: self.role,
            description: self.description,
            created: self.created,
            last_updated: self.last_updated,
            state: self.state,
            tags: self.tags,
            log_group: self.log_group,
        }
    }
}
