# Error

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**code** | Option<**String**> | An application-specific error code, expressed as a string value. | [optional]
**detail** | **String** | A human-readable explanation specific to this occurrence of the problem. | 
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | A unique identifier for this particular occurrence of the problem. | [optional]
**meta** | Option<[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional]
**source** | Option<[**crate::models::GetCustomBaseImages400ResponseErrorsInnerSource**](getCustomBaseImages_400_response_errors_inner_source.md)> |  | [optional]
**status** | **String** | The HTTP status code applicable to this problem, expressed as a string value. | 
**title** | Option<**String**> | A short, human-readable summary of the problem that SHOULD NOT change from occurrence to occurrence of the problem, except for purposes of localization. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


