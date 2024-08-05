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
pub struct GetVertexPodSpecReq {
    #[serde(rename = "DefaultResources")]
    pub default_resources: k8s_openapi::api::core::v1::ResourceRequirements,
    #[serde(rename = "Env")]
    pub env: Vec<k8s_openapi::api::core::v1::EnvVar>,
    #[serde(rename = "ISBSvcType")]
    pub isb_svc_type: String,
    #[serde(rename = "Image")]
    pub image: String,
    #[serde(rename = "PipelineSpec")]
    pub pipeline_spec: Box<crate::models::PipelineSpec>,
    #[serde(rename = "PullPolicy")]
    pub pull_policy: String,
    #[serde(rename = "ServingSourceStreamName")]
    pub serving_source_stream_name: String,
    #[serde(rename = "SideInputsStoreName")]
    pub side_inputs_store_name: String,
}

impl GetVertexPodSpecReq {
    pub fn new(default_resources: k8s_openapi::api::core::v1::ResourceRequirements, env: Vec<k8s_openapi::api::core::v1::EnvVar>, isb_svc_type: String, image: String, pipeline_spec: crate::models::PipelineSpec, pull_policy: String, serving_source_stream_name: String, side_inputs_store_name: String) -> GetVertexPodSpecReq {
        GetVertexPodSpecReq {
            default_resources,
            env,
            isb_svc_type,
            image,
            pipeline_spec: Box::new(pipeline_spec),
            pull_policy,
            serving_source_stream_name,
            side_inputs_store_name,
        }
    }
}


