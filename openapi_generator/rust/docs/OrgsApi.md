# \OrgsApi

All URIs are relative to *https://api.snyk.io/rest*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_org**](OrgsApi.md#get_org) | **GET** /orgs/{org_id} | Get organization
[**list_orgs**](OrgsApi.md#list_orgs) | **GET** /orgs | List accessible organizations
[**update_org**](OrgsApi.md#update_org) | **PATCH** /orgs/{org_id} | Update organization



## get_org

> crate::models::GetOrg200Response get_org(version, org_id)
Get organization

Get the full details of an organization.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**version** | **String** | The requested version of the endpoint to process the request | [required] |
**org_id** | **uuid::Uuid** | Unique identifier for org | [required] |

### Return type

[**crate::models::GetOrg200Response**](getOrg_200_response.md)

### Authorization

[APIToken](../README.md#APIToken), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_orgs

> crate::models::ListOrgs200Response list_orgs(version, starting_after, ending_before, limit, group_id, is_personal, slug, name, expand)
List accessible organizations

Get a paginated list of organizations you have access to.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**version** | **String** | The requested version of the endpoint to process the request | [required] |
**starting_after** | Option<**String**> | Return the page of results immediately after this cursor |  |
**ending_before** | Option<**String**> | Return the page of results immediately before this cursor |  |
**limit** | Option<**i32**> | Number of results to return per page |  |[default to 10]
**group_id** | Option<**uuid::Uuid**> | If set, only return organizations within the specified group |  |
**is_personal** | Option<**bool**> | If true, only return organizations that are not part of a group. |  |
**slug** | Option<**String**> | Only return orgs whose slug exactly matches this value. |  |
**name** | Option<**String**> | Only return orgs whose name contains this value. |  |
**expand** | Option<[**Vec<String>**](String.md)> | Expand the specified related resources in the response to include their attributes. |  |

### Return type

[**crate::models::ListOrgs200Response**](listOrgs_200_response.md)

### Authorization

[APIToken](../README.md#APIToken), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_org

> crate::models::UpdateOrg200Response update_org(version, org_id, update_org_request)
Update organization

Update the details of an organization

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**version** | **String** | The requested version of the endpoint to process the request | [required] |
**org_id** | **uuid::Uuid** | Unique identifier for org | [required] |
**update_org_request** | Option<[**UpdateOrgRequest**](UpdateOrgRequest.md)> |  |  |

### Return type

[**crate::models::UpdateOrg200Response**](updateOrg_200_response.md)

### Authorization

[APIToken](../README.md#APIToken), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/vnd.api+json
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

