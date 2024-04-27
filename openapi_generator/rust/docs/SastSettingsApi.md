# \SastSettingsApi

All URIs are relative to *https://api.snyk.io/rest*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_sast_settings**](SastSettingsApi.md#get_sast_settings) | **GET** /orgs/{org_id}/settings/sast | Retrieves the SAST settings for an org
[**update_org_sast_settings**](SastSettingsApi.md#update_org_sast_settings) | **PATCH** /orgs/{org_id}/settings/sast | Enable/Disable the Snyk Code settings for an org



## get_sast_settings

> crate::models::GetSastSettings200Response get_sast_settings(version, org_id)
Retrieves the SAST settings for an org

Retrieves the SAST settings for an org

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**version** | **String** | The requested version of the endpoint to process the request | [required] |
**org_id** | **uuid::Uuid** | The id of the org for which we want to retrieve the SAST settings | [required] |

### Return type

[**crate::models::GetSastSettings200Response**](getSastSettings_200_response.md)

### Authorization

[APIToken](../README.md#APIToken), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_org_sast_settings

> crate::models::GetSastSettings200Response update_org_sast_settings(version, org_id, update_org_sast_settings_request)
Enable/Disable the Snyk Code settings for an org

Enable/Disable the Snyk Code settings for an org

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**version** | **String** | The requested version of the endpoint to process the request | [required] |
**org_id** | **uuid::Uuid** | The id of the org for which we want to update the Snyk Code setting | [required] |
**update_org_sast_settings_request** | [**UpdateOrgSastSettingsRequest**](UpdateOrgSastSettingsRequest.md) |  | [required] |

### Return type

[**crate::models::GetSastSettings200Response**](getSastSettings_200_response.md)

### Authorization

[APIToken](../README.md#APIToken), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/vnd.api+json
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

