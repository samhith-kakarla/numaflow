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
pub struct GetContainerReq {
    #[serde(rename = "env")]
    pub env: Vec<k8s_openapi::api::core::v1::EnvVar>,
    #[serde(rename = "image")]
    pub image: String,
    #[serde(rename = "imagePullPolicy")]
    pub image_pull_policy: String,
    #[serde(rename = "isbSvcType")]
    pub isb_svc_type: String,
    #[serde(rename = "resources")]
    pub resources: k8s_openapi::api::core::v1::ResourceRequirements,
    #[serde(rename = "volumeMounts")]
    pub volume_mounts: Vec<k8s_openapi::api::core::v1::VolumeMount>,
}

impl GetContainerReq {
    pub fn new(
        env: Vec<k8s_openapi::api::core::v1::EnvVar>,
        image: String,
        image_pull_policy: String,
        isb_svc_type: String,
        resources: k8s_openapi::api::core::v1::ResourceRequirements,
        volume_mounts: Vec<k8s_openapi::api::core::v1::VolumeMount>,
    ) -> GetContainerReq {
        GetContainerReq {
            env,
            image,
            image_pull_policy,
            isb_svc_type,
            resources,
            volume_mounts,
        }
    }
}