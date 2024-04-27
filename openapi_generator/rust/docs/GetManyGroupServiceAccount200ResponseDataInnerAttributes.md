# GetManyGroupServiceAccount200ResponseDataInnerAttributes

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**access_token_ttl_seconds** | Option<**f32**> | The time, in seconds, that a generated access token will be valid for. Defaults to 1hr if unset. Only provided when auth_type is oauth_private_key_jwt. | [optional]
**api_key** | Option<**String**> | The Snyk API Key for this service account. Only returned on creation, and only when auth_type is api_key. | [optional]
**auth_type** | **String** | The authentication strategy for the service account:   * api_key - Regular Snyk API Key.   * oauth_client_secret - OAuth2 client_credentials grant, which returns a client secret that can be used to retrieve an access token.   * oauth_private_key_jwt - OAuth2 client_credentials grant, using private_key_jwt client_assertion as laid out OIDC Connect Core 1.0, section 9. | 
**client_id** | Option<**String**> | The service account's attached client-id. Used to request an access-token. Only provided when auth_type is oauth_private_key_jwt. | [optional]
**client_secret** | Option<**String**> | The client secret used for obtaining access tokens. Only sent on creation of new service accounts and cannot be retrieved thereafter. Only provided when auth_type is oauth_client_secret. | [optional]
**jwks_url** | Option<**String**> | A JWKs URL used to verify signed JWT requests against. Must be https. Only provided when auth_type is oauth_private_key_jwt. | [optional]
**level** | Option<**String**> | The level of access for the service account:   * Group - the service account was created at the Group level.   * Org - the service account was created at the Org level. | [optional]
**name** | **String** | A human-friendly name of the service account. | 
**role_id** | [**uuid::Uuid**](uuid::Uuid.md) | The ID of the role which the Service Account is associated with. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


