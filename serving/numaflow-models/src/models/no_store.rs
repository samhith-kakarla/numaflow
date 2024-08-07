/*
 * Numaflow
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: latest
 *
 * Generated by: https://openapi-generator.tech
 */

/// NoStore : NoStore means there will be no persistence storage and there will be data loss during pod restarts. Use this option only if you do not care about correctness (e.g., approx statistics pipeline like sampling rate, etc.).

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NoStore {}

impl NoStore {
    /// NoStore means there will be no persistence storage and there will be data loss during pod restarts. Use this option only if you do not care about correctness (e.g., approx statistics pipeline like sampling rate, etc.).
    pub fn new() -> NoStore {
        NoStore {}
    }
}
