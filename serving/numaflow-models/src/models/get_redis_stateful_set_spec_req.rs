/*
 * Numaflow
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: latest
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetRedisStatefulSetSpecReq {
    #[serde(rename = "ConfConfigMapName")]
    pub conf_config_map_name: String,
    #[serde(rename = "CredentialSecretName")]
    pub credential_secret_name: String,
    #[serde(rename = "DefaultResources")]
    pub default_resources: k8s_openapi::api::core::v1::ResourceRequirements,
    #[serde(rename = "HealthConfigMapName")]
    pub health_config_map_name: String,
    #[serde(rename = "InitContainerImage")]
    pub init_container_image: String,
    #[serde(rename = "Labels")]
    pub labels: ::std::collections::HashMap<String, String>,
    #[serde(rename = "MetricsExporterImage")]
    pub metrics_exporter_image: String,
    #[serde(rename = "PvcNameIfNeeded")]
    pub pvc_name_if_needed: String,
    #[serde(rename = "RedisContainerPort")]
    pub redis_container_port: i32,
    #[serde(rename = "RedisImage")]
    pub redis_image: String,
    #[serde(rename = "RedisMetricsContainerPort")]
    pub redis_metrics_container_port: i32,
    #[serde(rename = "ScriptsConfigMapName")]
    pub scripts_config_map_name: String,
    #[serde(rename = "SentinelContainerPort")]
    pub sentinel_container_port: i32,
    #[serde(rename = "SentinelImage")]
    pub sentinel_image: String,
    #[serde(rename = "ServiceName")]
    pub service_name: String,
    #[serde(rename = "TLSEnabled")]
    pub tls_enabled: bool,
}

impl GetRedisStatefulSetSpecReq {
    pub fn new(
        conf_config_map_name: String,
        credential_secret_name: String,
        default_resources: k8s_openapi::api::core::v1::ResourceRequirements,
        health_config_map_name: String,
        init_container_image: String,
        labels: ::std::collections::HashMap<String, String>,
        metrics_exporter_image: String,
        pvc_name_if_needed: String,
        redis_container_port: i32,
        redis_image: String,
        redis_metrics_container_port: i32,
        scripts_config_map_name: String,
        sentinel_container_port: i32,
        sentinel_image: String,
        service_name: String,
        tls_enabled: bool,
    ) -> GetRedisStatefulSetSpecReq {
        GetRedisStatefulSetSpecReq {
            conf_config_map_name,
            credential_secret_name,
            default_resources,
            health_config_map_name,
            init_container_image,
            labels,
            metrics_exporter_image,
            pvc_name_if_needed,
            redis_container_port,
            redis_image,
            redis_metrics_container_port,
            scripts_config_map_name,
            sentinel_container_port,
            sentinel_image,
            service_name,
            tls_enabled,
        }
    }
}