use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::transform::TypescriptableGraphQLType;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EnumValue {
    pub name: String,
    pub description: Option<String>,
    pub is_deprecated: bool,
    pub deprecation_reason: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TypeKind {
    Scalar,
    Object,
    Interface,
    Union,
    Enum,
    InputObject,
    List,
    NonNull,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "kind", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TypeRef {
    Scalar {
        name: String,
    },
    Object {
        name: String,
    },
    Interface {
        name: String,
    },
    Union {
        name: String,
    },
    Enum {
        name: String,
    },
    InputObject {
        name: String,
    },
    #[serde(rename_all = "camelCase")]
    NonNull {
        of_type: Box<TypeRef>,
    },
    #[serde(rename_all = "camelCase")]
    List {
        of_type: Box<TypeRef>,
    },
}

impl Display for TypeRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ts = self.as_typescript().map_err(|_| std::fmt::Error)?;
        write!(f, "{}", ts)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "kind", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Type {
    #[serde(rename_all = "camelCase")]
    Scalar {
        name: String,
        description: Option<String>,
    },
    #[serde(rename_all = "camelCase")]
    Object {
        name: String,
        description: Option<String>,
        fields: Vec<Field>,
        interfaces: Vec<Type>,
    },
    #[serde(rename_all = "camelCase")]
    Interface {
        name: String,
        description: Option<String>,
        fields: Vec<Field>,
        possible_types: Option<Box<Type>>,
    },
    #[serde(rename_all = "camelCase")]
    Union {
        name: String,
        description: Option<String>,
        possible_types: Option<Box<Type>>,
    },
    #[serde(rename_all = "camelCase")]
    Enum {
        name: String,
        description: Option<String>,
        enum_values: Vec<EnumValue>,
    },
    #[serde(rename_all = "camelCase")]
    InputObject {
        name: String,
        description: Option<String>,
        input_fields: Vec<InputValue>,
    },
    #[serde(rename_all = "camelCase")]
    NonNull { of_type: TypeRef },
    #[serde(rename_all = "camelCase")]
    List { of_type: TypeRef },
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", TypeRef::from(self.to_owned()))
    }
}

impl From<Type> for TypeRef {
    fn from(other: Type) -> Self {
        match other {
            Type::Scalar { name, .. } => TypeRef::Scalar { name },
            Type::Object { name, .. } => TypeRef::Object { name },
            Type::Interface { name, .. } => TypeRef::Interface { name },
            Type::Union { name, .. } => TypeRef::Union { name },
            Type::Enum { name, .. } => TypeRef::Enum { name },
            Type::InputObject { name, .. } => TypeRef::InputObject { name },
            Type::NonNull { of_type } => TypeRef::NonNull {
                of_type: Box::new(of_type),
            },
            Type::List { of_type } => TypeRef::List {
                of_type: Box::new(of_type),
            },
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct InputValue {
    pub name: String,
    pub description: Option<String>,
    #[serde(rename = "type")]
    pub of_type: TypeRef,
    pub default_value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Field {
    pub name: String,
    pub description: Option<String>,
    pub args: Vec<InputValue>,
    #[serde(rename = "type")]
    pub of_type: TypeRef,
    pub is_deprecated: bool,
    pub deprecation_reason: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DirectiveLocation {
    Query,
    Mutation,
    Subscription,
    Field,
    FragmentDefinition,
    FragmentSpread,
    InlineFragment,
    Schema,
    Scalar,
    Object,
    FieldDefinition,
    ArgumentDefinition,
    Interface,
    Union,
    Enum,
    EnumValue,
    InputObject,
    InputFieldDefinition,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Directive {
    pub description: Option<String>,
    pub name: String,
    pub locations: Vec<DirectiveLocation>,
    pub args: Vec<InputValue>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RootType {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Schema {
    pub types: Vec<Type>,
    pub query_type: RootType,
    pub mutation_type: Option<RootType>,
    pub subscription_type: Option<RootType>,
    pub directives: Vec<Directive>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Data {
    #[serde(rename = "__schema")]
    pub schema: Schema,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IntrospectionResponse {
    pub data: Data,
}