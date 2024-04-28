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
pub struct CollectionRelationshipsCreatedByUserData {
    /// ID of the user that created the collection. Null for auto-collections.
    #[serde(rename = "id", deserialize_with = "Option::deserialize")]
    pub id: Option<uuid::Uuid>,
    #[serde(rename = "type")]
    pub r#type: Type,
}

impl CollectionRelationshipsCreatedByUserData {
    pub fn new(id: Option<uuid::Uuid>, r#type: Type) -> CollectionRelationshipsCreatedByUserData {
        CollectionRelationshipsCreatedByUserData {
            id,
            r#type,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "user")]
    User,
}

impl Default for Type {
    fn default() -> Type {
        Self::User
    }
}
