use crate::resource::*;
use crate::rust::collections::HashMap;
use crate::rust::str;
use crate::rust::string::String;
use crate::rust::string::ToString;
use sbor::*;

/// Method authorization rules for a component
#[derive(Debug, Clone, PartialEq, Describe, TypeId, Encode, Decode)]
pub struct ComponentAuthorization(HashMap<String, MethodAuth>);

impl ComponentAuthorization {
    pub fn new() -> Self {
        ComponentAuthorization(HashMap::new())
    }

    pub fn contains_method(&self, method_name: &str) -> bool {
        self.0.contains_key(method_name)
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn insert(&mut self, method_name: &str, method_auth: MethodAuth) {
        self.0.insert(method_name.to_string(), method_auth);
    }

    pub fn to_map(self) -> HashMap<String, MethodAuth> {
        self.0
    }
}

#[macro_export]
macro_rules! component_authorization {
  {$($k: expr => $v: expr),* $(,)?} => {
    {
      let mut authorization = ::scrypto::resource::ComponentAuthorization::new();

      $(
        authorization.insert($k, $v);
      )*

      authorization
    }
  };
}
