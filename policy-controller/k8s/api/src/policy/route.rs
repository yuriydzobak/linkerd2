use crate::labels;
use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, CustomResource, Deserialize, Serialize, JsonSchema)]
#[kube(
    group = "policy.linkerd.io",
    version = "v1alpha1",
    kind = "HttpRequestLabeler",
    namespaced
)]
pub struct HttpRequestLabelerSpec {
    pub rules: Vec<HttpRequestLabelerRule>,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, JsonSchema)]
pub struct HttpRequestLabelerRule {}
