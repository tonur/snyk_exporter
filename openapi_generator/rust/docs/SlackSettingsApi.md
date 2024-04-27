# \SlackSettingsApi

All URIs are relative to *https://api.snyk.io/rest*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_slack_default_notification_settings**](SlackSettingsApi.md#create_slack_default_notification_settings) | **POST** /orgs/{org_id}/slack_app/{bot_id} | Create new Slack notification default settings.
[**create_slack_project_notification_settings**](SlackSettingsApi.md#create_slack_project_notification_settings) | **POST** /orgs/{org_id}/slack_app/{bot_id}/projects/{project_id} | Create a new Slack settings override for a given project.
[**delete_slack_default_notification_settings**](SlackSettingsApi.md#delete_slack_default_notification_settings) | **DELETE** /orgs/{org_id}/slack_app/{bot_id} | Remove the given Slack App integration
[**delete_slack_project_notification_settings**](SlackSettingsApi.md#delete_slack_project_notification_settings) | **DELETE** /orgs/{org_id}/slack_app/{bot_id}/projects/{project_id} | Remove Slack settings override for a project.
[**get_slack_default_notification_settings**](SlackSettingsApi.md#get_slack_default_notification_settings) | **GET** /orgs/{org_id}/slack_app/{bot_id} | Get Slack integration default notification settings.
[**get_slack_project_notification_settings_collection**](SlackSettingsApi.md#get_slack_project_notification_settings_collection) | **GET** /orgs/{org_id}/slack_app/{bot_id}/projects | Slack notification settings overrides for projects
[**update_slack_project_notification_settings**](SlackSettingsApi.md#update_slack_project_notification_settings) | **PATCH** /orgs/{org_id}/slack_app/{bot_id}/projects/{project_id} | Update Slack notification settings for a project.



## create_slack_default_notification_settings

> crate::models::GetSlackDefaultNotificationSettings200Response create_slack_default_notification_settings(version, org_id, bot_id, settings_request)
Create new Slack notification default settings.

Create new Slack notification default settings for a given tenant.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**version** | **String** | The requested version of the endpoint to process the request | [required] |
**org_id** | **uuid::Uuid** | Org ID | [required] |
**bot_id** | **uuid::Uuid** | Bot ID | [required] |
**settings_request** | Option<[**SettingsRequest**](SettingsRequest.md)> | Create new Slack notification default settings for a tenant. |  |

### Return type

[**crate::models::GetSlackDefaultNotificationSettings200Response**](getSlackDefaultNotificationSettings_200_response.md)

### Authorization

[APIToken](../README.md#APIToken), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/vnd.api+json
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_slack_project_notification_settings

> crate::models::CreateSlackProjectNotificationSettings201Response create_slack_project_notification_settings(version, org_id, project_id, bot_id, settings_request)
Create a new Slack settings override for a given project.

Create Slack settings override for a project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**version** | **String** | The requested version of the endpoint to process the request | [required] |
**org_id** | **uuid::Uuid** | Org ID | [required] |
**project_id** | **uuid::Uuid** | Project ID | [required] |
**bot_id** | **uuid::Uuid** | Bot ID | [required] |
**settings_request** | Option<[**SettingsRequest**](SettingsRequest.md)> | Create new Slack notification default settings for a tenant. |  |

### Return type

[**crate::models::CreateSlackProjectNotificationSettings201Response**](createSlackProjectNotificationSettings_201_response.md)

### Authorization

[APIToken](../README.md#APIToken), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/vnd.api+json
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_slack_default_notification_settings

> delete_slack_default_notification_settings(version, org_id, bot_id)
Remove the given Slack App integration

Remove the given Slack App integration

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**version** | **String** | The requested version of the endpoint to process the request | [required] |
**org_id** | **uuid::Uuid** | Org ID | [required] |
**bot_id** | **uuid::Uuid** | Bot ID | [required] |

### Return type

 (empty response body)

### Authorization

[APIToken](../README.md#APIToken), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_slack_project_notification_settings

> delete_slack_project_notification_settings(version, org_id, project_id, bot_id)
Remove Slack settings override for a project.

Remove Slack settings override for a project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**version** | **String** | The requested version of the endpoint to process the request | [required] |
**org_id** | **uuid::Uuid** | Org ID | [required] |
**project_id** | **uuid::Uuid** | Project ID | [required] |
**bot_id** | **uuid::Uuid** | Bot ID | [required] |

### Return type

 (empty response body)

### Authorization

[APIToken](../README.md#APIToken), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_slack_default_notification_settings

> crate::models::GetSlackDefaultNotificationSettings200Response get_slack_default_notification_settings(version, org_id, bot_id)
Get Slack integration default notification settings.

Get Slack integration default notification settings for the provided tenant ID and bot ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**version** | **String** | The requested version of the endpoint to process the request | [required] |
**org_id** | **uuid::Uuid** | Org ID | [required] |
**bot_id** | **uuid::Uuid** | Bot ID | [required] |

### Return type

[**crate::models::GetSlackDefaultNotificationSettings200Response**](getSlackDefaultNotificationSettings_200_response.md)

### Authorization

[APIToken](../README.md#APIToken), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_slack_project_notification_settings_collection

> crate::models::GetProjectSettingsCollection get_slack_project_notification_settings_collection(version, org_id, bot_id, starting_after, ending_before, limit)
Slack notification settings overrides for projects

Slack notification settings overrides for projects. These settings overrides the default settings configured for the tenant.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**version** | **String** | The requested version of the endpoint to process the request | [required] |
**org_id** | **uuid::Uuid** | Org ID | [required] |
**bot_id** | **uuid::Uuid** | Bot ID | [required] |
**starting_after** | Option<**String**> | Return the page of results immediately after this cursor |  |
**ending_before** | Option<**String**> | Return the page of results immediately before this cursor |  |
**limit** | Option<**i32**> | Number of results to return per page |  |[default to 10]

### Return type

[**crate::models::GetProjectSettingsCollection**](GetProjectSettingsCollection.md)

### Authorization

[APIToken](../README.md#APIToken), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_slack_project_notification_settings

> crate::models::CreateSlackProjectNotificationSettings201Response update_slack_project_notification_settings(version, org_id, bot_id, project_id, project_settings_patch_request)
Update Slack notification settings for a project.

Update Slack notification settings for a project.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**version** | **String** | The requested version of the endpoint to process the request | [required] |
**org_id** | **uuid::Uuid** | Org ID | [required] |
**bot_id** | **uuid::Uuid** | Bot ID | [required] |
**project_id** | **uuid::Uuid** | Project ID | [required] |
**project_settings_patch_request** | Option<[**ProjectSettingsPatchRequest**](ProjectSettingsPatchRequest.md)> | Update existing project specific settings for a project. |  |

### Return type

[**crate::models::CreateSlackProjectNotificationSettings201Response**](createSlackProjectNotificationSettings_201_response.md)

### Authorization

[APIToken](../README.md#APIToken), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/vnd.api+json
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

