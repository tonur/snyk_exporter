# CustomBaseImageAttributes

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**include_in_recommendations** | **bool** | Whether this image should be recommended as a base image upgrade.  If set to true, this image could be shown as a base image upgrade to other projects. If set to false this image will never be recommended as an upgrade.  | 
**project_id** | [**uuid::Uuid**](uuid::Uuid.md) | The ID of the container project that the custom base image is based off of. The attributes of this custom base image are taken from the latest snapshot at the time of creation. This means that no changes should be made to the original project after the creation of the custom base image, as new snapshots created from any changes will NOT be picked up.  | 
**versioning_schema** | Option<[**crate::models::VersioningSchema**](VersioningSchema.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


