# RemedyMetadata

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**data** | [**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md) | Metadata information related to apply a remedy. Limited in size to 100Kb when JSON serialized. | 
**schema_version** | **String** | A schema version identifier the metadata object validates against. Note: this information is only relevant in the domain of the API consumer: the issues system always considers metadata just as an arbitrary object.  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


