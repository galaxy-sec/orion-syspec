use derive_getters::Getters;
use derive_more::Display;
use serde_derive::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct VarDefinition<T>
where
    T: serde::Serialize + Clone,
{
    name: String,
    value: T,
    //#[serde(skip_serializing_if = "Option::is_none")]
    //constr: Option<ValueConstraint>,
}
impl<T> VarDefinition<T>
where
    T: serde::Serialize + Clone,
{
    pub(crate) fn var_value(&self) -> VarValue<T> {
        VarValue {
            value: self.value.clone(),
        }
    }

    pub(crate) fn name(&self) -> &str {
        self.name.as_str()
    }
}

#[derive(Getters, Clone, Debug, Serialize, Deserialize, PartialEq, Display)]
#[serde(transparent)]
pub struct VarValue<T>
where
    T: serde::Serialize,
{
    #[display("{value}")]
    value: T,
}

impl From<(&str, &str)> for VarDefinition<String> {
    fn from(value: (&str, &str)) -> Self {
        VarDefinition {
            name: value.0.to_string(),
            value: value.1.to_string(),
        }
    }
}
impl From<(&str, bool)> for VarDefinition<bool> {
    fn from(value: (&str, bool)) -> Self {
        VarDefinition {
            name: value.0.to_string(),
            value: value.1,
        }
    }
}
impl From<(&str, u64)> for VarDefinition<u64> {
    fn from(value: (&str, u64)) -> Self {
        VarDefinition {
            name: value.0.to_string(),
            value: value.1,
        }
    }
}
impl From<(&str, f64)> for VarDefinition<f64> {
    fn from(value: (&str, f64)) -> Self {
        VarDefinition {
            name: value.0.to_string(),
            value: value.1,
        }
    }
}

impl From<&str> for VarValue<String> {
    fn from(value: &str) -> Self {
        VarValue {
            value: value.to_string(),
        }
    }
}

impl From<String> for VarValue<String> {
    fn from(value: String) -> Self {
        VarValue { value }
    }
}
impl From<bool> for VarValue<bool> {
    fn from(value: bool) -> Self {
        VarValue { value }
    }
}
impl From<u64> for VarValue<u64> {
    fn from(value: u64) -> Self {
        VarValue { value }
    }
}
impl From<f64> for VarValue<f64> {
    fn from(value: f64) -> Self {
        VarValue { value }
    }
}
