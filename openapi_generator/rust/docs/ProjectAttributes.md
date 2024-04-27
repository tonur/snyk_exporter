# ProjectAttributes

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**build_args** | Option<[**crate::models::ProjectAttributesBuildArgs**](ProjectAttributes_build_args.md)> |  | [optional]
**business_criticality** | Option<**Vec<String>**> |  | [optional]
**created** | **String** | The date that the project was created on | 
**environment** | Option<**Vec<String>**> |  | [optional]
**lifecycle** | Option<**Vec<String>**> |  | [optional]
**name** | **String** | Project name. | 
**origin** | **String** | The origin the project was added from. | 
**read_only** | **bool** | Whether the project is read-only | 
**settings** | [**crate::models::ProjectSettings**](ProjectSettings.md) |  | 
**status** | **String** | Describes if a project is currently monitored or it is de-activated. | 
**tags** | Option<[**Vec<crate::models::PatchProjectRequestDataAttributesTagsInner>**](PatchProjectRequest_data_attributes_tags_inner.md)> |  | [optional]
**target_file** | **String** | Path within the target to identify a specific file/directory/image etc. when scanning just part  of the target, and not the entity. | 
**target_reference** | **String** | The additional information required to resolve which revision of the resource should be scanned. | 
**target_runtime** | Option<**String**> | Dotnet Target, for relevant projects | [optional]
**r#type** | **String** | The package manager of the project. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


