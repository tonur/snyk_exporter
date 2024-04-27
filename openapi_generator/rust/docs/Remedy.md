# Remedy

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**correlation_id** | Option<**String**> | An optional identifier for correlating remedies between coordinates or across issues. They are scoped to a single Project and test run. Remedies with the same correlation_id must have the same contents.  | [optional]
**description** | Option<**String**> | A markdown-formatted optional description of this remedy. Links are not permitted. | [optional]
**meta** | Option<[**crate::models::RemedyMetadata**](RemedyMetadata.md)> |  | [optional]
**r#type** | **String** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


