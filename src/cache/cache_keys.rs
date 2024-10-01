pub enum CacheKeys {
    Cluster(String),
    Namespace(String),
}

impl CacheKeys {
    pub fn key(&self) -> String {
        match self {
            CacheKeys::Cluster(key) => format!("cluster:{}", key),
            CacheKeys::Namespace(key) => format!("namespace:{}", key),
        }
    }
}

