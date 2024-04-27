# VersioningSchema

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **String** |  | 
**expression** | **String** | The regular expression used to describe the format of tags. Keep in mind that backslashes in the expression need to be escaped due to being encompassed in a JSON string.  | 
**label** | Option<**String**> | A customizable string that can be set for a custom versioning schema to describe its meaning. This label has no function.  | [optional]
**is_selected** | **bool** | Whether this image should be the recommendation. Only one image can be selected at a given time. Setting this as true will remove previous selection.  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


