# \IacSettingsApi

All URIs are relative to *https://api.snyk.io/rest*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_iac_settings_for_group**](IacSettingsApi.md#get_iac_settings_for_group) | **GET** /groups/{group_id}/settings/iac | Get the Infrastructure as Code Settings for a group
[**get_iac_settings_for_org**](IacSettingsApi.md#get_iac_settings_for_org) | **GET** /orgs/{org_id}/settings/iac | Get the Infrastructure as Code Settings for an org.
[**update_iac_settings_for_group**](IacSettingsApi.md#update_iac_settings_for_group) | **PATCH** /groups/{group_id}/settings/iac | Update the Infrastructure as Code Settings for a group
[**update_iac_settings_for_org**](IacSettingsApi.md#update_iac_settings_for_org) | **PATCH** /orgs/{org_id}/settings/iac | Update the Infrastructure as Code Settings for an org



## get_iac_settings_for_group

> crate::models::GetIacSettingsForGroup200Response get_iac_settings_for_group(version, group_id)
Get the Infrastructure as Code Settings for a group

Get the Infrastructure as Code Settings for a group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**version** | **String** | The requested version of the endpoint to process the request | [required] |
**group_id** | **uuid::Uuid** | The id of the group whose Infrastructure as Code settings are requested | [required] |

### Return type

[**crate::models::GetIacSettingsForGroup200Response**](getIacSettingsForGroup_200_response.md)

### Authorization

[APIToken](../README.md#APIToken), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_iac_settings_for_org

> crate::models::GetIacSettingsForOrg200Response get_iac_settings_for_org(version, org_id)
Get the Infrastructure as Code Settings for an org.

Get the Infrastructure as Code Settings for an org.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**version** | **String** | The requested version of the endpoint to process the request | [required] |
**org_id** | **uuid::Uuid** | The id of the org whose Infrastructure as Code settings are requested. | [required] |

### Return type

[**crate::models::GetIacSettingsForOrg200Response**](getIacSettingsForOrg_200_response.md)

### Authorization

[APIToken](../README.md#APIToken), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_iac_settings_for_group

> crate::models::GetIacSettingsForGroup200Response update_iac_settings_for_group(version, group_id, update_iac_settings_for_group_request)
Update the Infrastructure as Code Settings for a group

Update the Infrastructure as Code Settings for a group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**version** | **String** | The requested version of the endpoint to process the request | [required] |
**group_id** | **uuid::Uuid** | The id of the group whose Infrastructure as Code settings are getting updated | [required] |
**update_iac_settings_for_group_request** | Option<[**UpdateIacSettingsForGroupRequest**](UpdateIacSettingsForGroupRequest.md)> |  |  |

### Return type

[**crate::models::GetIacSettingsForGroup200Response**](getIacSettingsForGroup_200_response.md)

### Authorization

[APIToken](../README.md#APIToken), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/vnd.api+json
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_iac_settings_for_org

> crate::models::GetIacSettingsForOrg200Response update_iac_settings_for_org(version, org_id, update_iac_settings_for_org_request)
Update the Infrastructure as Code Settings for an org

Update the Infrastructure as Code Settings for an org.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**version** | **String** | The requested version of the endpoint to process the request | [required] |
**org_id** | **uuid::Uuid** | The id of the org whose Infrastructure as Code settings are getting updated | [required] |
**update_iac_settings_for_org_request** | Option<[**UpdateIacSettingsForOrgRequest**](UpdateIacSettingsForOrgRequest.md)> |  |  |

### Return type

[**crate::models::GetIacSettingsForOrg200Response**](getIacSettingsForOrg_200_response.md)

### Authorization

[APIToken](../README.md#APIToken), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/vnd.api+json
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

