/*
 * Snyk API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: REST
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PatchProjectRequestDataRelationshipsOwner {
    #[serde(rename = "data")]
    pub data: Box<crate::models::PatchProjectRequestDataRelationshipsOwnerData>,
}

impl PatchProjectRequestDataRelationshipsOwner {
    pub fn new(data: crate::models::PatchProjectRequestDataRelationshipsOwnerData) -> PatchProjectRequestDataRelationshipsOwner {
        PatchProjectRequestDataRelationshipsOwner {
            data: Box::new(data),
        }
    }
}

