# AppPostRequestDataAttributes

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**access_token_ttl_seconds** | Option<**f32**> | The access token time to live for your app, in seconds. It only affects the newly generated access tokens, existing access token will  continue to have their previous time to live as expiration. | [optional]
**context** | [**crate::models::Context**](Context.md) |  | 
**name** | **String** | New name of the app to display to users during authorization flow. | 
**redirect_uris** | **Vec<String>** | List of allowed redirect URIs to call back after authentication. | 
**scopes** | **Vec<String>** | The scopes this app is allowed to request during authorization. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


