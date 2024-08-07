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
pub struct KafkaSource {
    #[serde(rename = "brokers", skip_serializing_if = "Option::is_none")]
    pub brokers: Option<Vec<String>>,
    #[serde(rename = "config", skip_serializing_if = "Option::is_none")]
    pub config: Option<String>,
    #[serde(rename = "consumerGroup", skip_serializing_if = "Option::is_none")]
    pub consumer_group: Option<String>,
    #[serde(rename = "sasl", skip_serializing_if = "Option::is_none")]
    pub sasl: Option<Box<crate::models::Sasl>>,
    #[serde(rename = "tls", skip_serializing_if = "Option::is_none")]
    pub tls: Option<Box<crate::models::Tls>>,
    #[serde(rename = "topic")]
    pub topic: String,
}

impl KafkaSource {
    pub fn new(topic: String) -> KafkaSource {
        KafkaSource {
            brokers: None,
            config: None,
            consumer_group: None,
            sasl: None,
            tls: None,
            topic,
        }
    }
}