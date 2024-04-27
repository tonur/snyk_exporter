# \SlackApi

All URIs are relative to *https://api.snyk.io/rest*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_channel_name_by_id**](SlackApi.md#get_channel_name_by_id) | **GET** /orgs/{org_id}/slack_app/{tenant_id}/channels/{channel_id} | Get Slack Channel name by Slack Channel ID.
[**list_channels**](SlackApi.md#list_channels) | **GET** /orgs/{org_id}/slack_app/{tenant_id}/channels | Get a list of Slack channels



## get_channel_name_by_id

> crate::models::GetChannelNameById200Response get_channel_name_by_id(version, org_id, channel_id, tenant_id)
Get Slack Channel name by Slack Channel ID.

Requires the Snyk Slack App to be set up for this org. It will return the Slack channel name for the provided Slack channel ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**version** | **String** | The requested version of the endpoint to process the request | [required] |
**org_id** | **uuid::Uuid** | Org ID | [required] |
**channel_id** | **String** | Slack Channel ID | [required] |
**tenant_id** | **uuid::Uuid** | Tenant ID | [required] |

### Return type

[**crate::models::GetChannelNameById200Response**](getChannelNameById_200_response.md)

### Authorization

[APIToken](../README.md#APIToken), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_channels

> crate::models::ListChannels200Response list_channels(version, org_id, tenant_id, starting_after, ending_before, limit)
Get a list of Slack channels

Requires the Snyk Slack App to be set up for this org, will retrieve a list of channels the Snyk Slack App can access. Note that it is currently only possible to page forwards through this collection, no prev links will be generated and the ending_before parameter will not function.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**version** | **String** | The requested version of the endpoint to process the request | [required] |
**org_id** | **uuid::Uuid** | Org ID | [required] |
**tenant_id** | **uuid::Uuid** | Tenant ID | [required] |
**starting_after** | Option<**String**> | Return the page of results immediately after this cursor |  |
**ending_before** | Option<**String**> | Return the page of results immediately before this cursor |  |
**limit** | Option<**i32**> | Number of results to return per page |  |[default to 1000]

### Return type

[**crate::models::ListChannels200Response**](listChannels_200_response.md)

### Authorization

[APIToken](../README.md#APIToken), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

