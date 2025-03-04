use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Node {
    pub env: String,
    pub namespace: String,
    pub deployment: String,
    pub pod: String,
    pub r#type: String,
}

impl Node {
    pub fn url(&self, token: &str) -> String {
        format!(
            "wss://value.weike.fm/ws/api/k8s/{}/pods/log?container=app&follow=true&previous=false&timestamps=true&prefix=false&tailLines=500&proj_id=1&namespace={}&label=app={},cicd_env=stable,name={},type={},version=stable&token={}",
            self.env, self.namespace, self.deployment, self.pod, self.r#type, token
        )
    }
}
