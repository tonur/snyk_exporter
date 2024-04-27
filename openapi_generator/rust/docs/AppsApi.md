# \AppsApi

All URIs are relative to *https://api.snyk.io/rest*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_app**](AppsApi.md#create_app) | **POST** /orgs/{org_id}/apps | Create a new app for an organization.
[**create_group_app_install**](AppsApi.md#create_group_app_install) | **POST** /groups/{group_id}/apps/installs | Install a Snyk Apps to this group.
[**create_org_app**](AppsApi.md#create_org_app) | **POST** /orgs/{org_id}/apps/creations | Create a new Snyk App for an organization.
[**create_org_app_install**](AppsApi.md#create_org_app_install) | **POST** /orgs/{org_id}/apps/installs | Install a Snyk Apps to this organization.
[**delete_app**](AppsApi.md#delete_app) | **DELETE** /orgs/{org_id}/apps/{client_id} | Delete an app
[**delete_app_bot**](AppsApi.md#delete_app_bot) | **DELETE** /orgs/{org_id}/app_bots/{bot_id} | Revoke app bot authorization
[**delete_app_by_id**](AppsApi.md#delete_app_by_id) | **DELETE** /orgs/{org_id}/apps/creations/{app_id} | Delete an app by its App ID.
[**delete_app_org_install_by_id**](AppsApi.md#delete_app_org_install_by_id) | **DELETE** /orgs/{org_id}/apps/installs/{install_id} | Revoke app authorization for an Snyk Organization with install ID.
[**delete_group_app_install_by_id**](AppsApi.md#delete_group_app_install_by_id) | **DELETE** /groups/{group_id}/apps/installs/{install_id} | Revoke app authorization for an Snyk Group with install ID.
[**delete_user_app_install_by_id**](AppsApi.md#delete_user_app_install_by_id) | **DELETE** /self/apps/installs/{install_id} | Revoke access for an app by install ID.
[**get_app**](AppsApi.md#get_app) | **GET** /orgs/{org_id}/apps/{client_id} | Get an app by client id
[**get_app_bots**](AppsApi.md#get_app_bots) | **GET** /orgs/{org_id}/app_bots | Get a list of app bots authorized to an organization.
[**get_app_by_id**](AppsApi.md#get_app_by_id) | **GET** /orgs/{org_id}/apps/creations/{app_id} | Get a Snyk App by its App ID.
[**get_app_installs_for_group**](AppsApi.md#get_app_installs_for_group) | **GET** /groups/{group_id}/apps/installs | Get a list of apps installed for a group.
[**get_app_installs_for_org**](AppsApi.md#get_app_installs_for_org) | **GET** /orgs/{org_id}/apps/installs | Get a list of apps installed for an organization.
[**get_app_installs_for_user**](AppsApi.md#get_app_installs_for_user) | **GET** /self/apps/installs | Get a list of apps installed for an user.
[**get_apps**](AppsApi.md#get_apps) | **GET** /orgs/{org_id}/apps | Get a list of apps created by an organization.
[**get_org_apps**](AppsApi.md#get_org_apps) | **GET** /orgs/{org_id}/apps/creations | Get a list of apps created by an organization.
[**get_user_app_sessions**](AppsApi.md#get_user_app_sessions) | **GET** /self/apps/{app_id}/sessions | Get a list of active OAuth sessions for the app.
[**get_user_installed_apps**](AppsApi.md#get_user_installed_apps) | **GET** /self/apps | Get a list of apps that can act on your behalf.
[**manage_app_creation_secret**](AppsApi.md#manage_app_creation_secret) | **POST** /orgs/{org_id}/apps/creations/{app_id}/secrets | Manage client secret for the Snyk App.
[**manage_secrets**](AppsApi.md#manage_secrets) | **POST** /orgs/{org_id}/apps/{client_id}/secrets | Manage client secrets for an app.
[**revoke_user_app_session**](AppsApi.md#revoke_user_app_session) | **DELETE** /self/apps/{app_id}/sessions/{session_id} | Revoke an active user app session.
[**revoke_user_installed_app**](AppsApi.md#revoke_user_installed_app) | **DELETE** /self/apps/{app_id} | Revoke an app
[**update_app**](AppsApi.md#update_app) | **PATCH** /orgs/{org_id}/apps/{client_id} | Update app attributes that are name, redirect URIs, and access token time to live
[**update_app_creation_by_id**](AppsApi.md#update_app_creation_by_id) | **PATCH** /orgs/{org_id}/apps/creations/{app_id} | Update app creation attributes such as name, redirect URIs, and access token time to live using the App ID.
[**update_group_app_install_secret**](AppsApi.md#update_group_app_install_secret) | **POST** /groups/{group_id}/apps/installs/{install_id}/secrets | Manage client secret for non-interactive Snyk App installations.
[**update_org_app_install_secret**](AppsApi.md#update_org_app_install_secret) | **POST** /orgs/{org_id}/apps/installs/{install_id}/secrets | Manage client secret for non-interactive Snyk App installations.



## create_app

> crate::models::AppPostResponse create_app(version, org_id, app_post_request)
Create a new app for an organization.

Create a new app for an organization. Deprecated, use /orgs/{org_id}/apps/creations instead.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**version** | **String** | The requested version of the endpoint to process the request | [required] |
**org_id** | **uuid::Uuid** | Org ID | [required] |
**app_post_request** | Option<[**AppPostRequest**](AppPostRequest.md)> | app to be created |  |

### Return type

[**crate::models::AppPostResponse**](AppPostResponse.md)

### Authorization

[APIToken](../README.md#APIToken), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/vnd.api+json
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_group_app_install

> crate::models::CreateGroupAppInstall201Response create_group_app_install(version, group_id, create_group_app_install_request)
Install a Snyk Apps to this group.

Install a Snyk Apps to this group, the Snyk App must use unattended authentication eg client credentials.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**version** | **String** | The requested version of the endpoint to process the request | [required] |
**group_id** | **uuid::Uuid** | Group ID | [required] |
**create_group_app_install_request** | Option<[**CreateGroupAppInstallRequest**](CreateGroupAppInstallRequest.md)> | App Install to be created |  |

### Return type

[**crate::models::CreateGroupAppInstall201Response**](createGroupAppInstall_201_response.md)

### Authorization

[APIToken](../README.md#APIToken), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/vnd.api+json
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_org_app

> crate::models::AppPostResponse create_org_app(version, org_id, app_post_request)
Create a new Snyk App for an organization.

Create a new Snyk App for an organization.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**version** | **String** | The requested version of the endpoint to process the request | [required] |
**org_id** | **uuid::Uuid** | Org ID | [required] |
**app_post_request** | Option<[**AppPostRequest**](AppPostRequest.md)> | Snyk App details for app to be created. |  |

### Return type

[**crate::models::AppPostResponse**](AppPostResponse.md)

### Authorization

[APIToken](../README.md#APIToken), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/vnd.api+json
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_org_app_install

> crate::models::CreateGroupAppInstall201Response create_org_app_install(version, org_id, create_group_app_install_request)
Install a Snyk Apps to this organization.

Install a Snyk Apps to this organization, the Snyk App must use unattended authentication eg client credentials.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**version** | **String** | The requested version of the endpoint to process the request | [required] |
**org_id** | **uuid::Uuid** | Org ID | [required] |
**create_group_app_install_request** | Option<[**CreateGroupAppInstallRequest**](CreateGroupAppInstallRequest.md)> | App Install to be created |  |

### Return type

[**crate::models::CreateGroupAppInstall201Response**](createGroupAppInstall_201_response.md)

### Authorization

[APIToken](../README.md#APIToken), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/vnd.api+json
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_app

> delete_app(version, org_id, client_id)
Delete an app

Delete an app by app id. Deprecated, use /orgs/{org_id}/apps/creations/{app_id} instead.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**version** | **String** | The requested version of the endpoint to process the request | [required] |
**org_id** | **uuid::Uuid** | Org ID | [required] |
**client_id** | **uuid::Uuid** | Client ID | [required] |

### Return type

 (empty response body)

### Authorization

[APIToken](../README.md#APIToken), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_app_bot

> delete_app_bot(bot_id, version, org_id)
Revoke app bot authorization

Revoke app bot authorization. Deprecated, use /orgs/{org_id}/apps/installs/{install_id} instead.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**bot_id** | **uuid::Uuid** | The ID of the app bot | [required] |
**version** | **String** | The requested version of the endpoint to process the request | [required] |
**org_id** | **uuid::Uuid** | Org ID | [required] |

### Return type

 (empty response body)

### Authorization

[APIToken](../README.md#APIToken), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_app_by_id

> delete_app_by_id(org_id, app_id, version)
Delete an app by its App ID.

Delete an app by its App ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_id** | **uuid::Uuid** | Org ID | [required] |
**app_id** | **uuid::Uuid** | App ID | [required] |
**version** | **String** | The requested version of the endpoint to process the request | [required] |

### Return type

 (empty response body)

### Authorization

[APIToken](../README.md#APIToken), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_app_org_install_by_id

> delete_app_org_install_by_id(version, org_id, install_id)
Revoke app authorization for an Snyk Organization with install ID.

Revoke app authorization for an Snyk Organization with install ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**version** | **String** | The requested version of the endpoint to process the request | [required] |
**org_id** | **uuid::Uuid** | Org ID | [required] |
**install_id** | **uuid::Uuid** | Install ID | [required] |

### Return type

 (empty response body)

### Authorization

[APIToken](../README.md#APIToken), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_group_app_install_by_id

> delete_group_app_install_by_id(version, group_id, install_id)
Revoke app authorization for an Snyk Group with install ID.

Revoke app authorization for an Snyk Group with install ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**version** | **String** | The requested version of the endpoint to process the request | [required] |
**group_id** | **uuid::Uuid** | Group ID | [required] |
**install_id** | **uuid::Uuid** | Install ID | [required] |

### Return type

 (empty response body)

### Authorization

[APIToken](../README.md#APIToken), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_user_app_install_by_id

> delete_user_app_install_by_id(version, install_id)
Revoke access for an app by install ID.

Revoke access for an app by install ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**version** | **String** | The requested version of the endpoint to process the request | [required] |
**install_id** | **uuid::Uuid** | Install ID | [required] |

### Return type

 (empty response body)

### Authorization

[APIToken](../README.md#APIToken), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_app

> crate::models::GetAppById200Response get_app(org_id, client_id, version)
Get an app by client id

Get an App by client id. Deprecated, use /orgs/{org_id}/apps/creations/{app_id} instead.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_id** | **uuid::Uuid** | Org ID | [required] |
**client_id** | **uuid::Uuid** | Client ID | [required] |
**version** | **String** | The requested version of the endpoint to process the request | [required] |

### Return type

[**crate::models::GetAppById200Response**](getAppByID_200_response.md)

### Authorization

[APIToken](../README.md#APIToken), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_app_bots

> crate::models::GetAppBots200Response get_app_bots(org_id, version, expand, starting_after, ending_before, limit)
Get a list of app bots authorized to an organization.

Get a list of app bots authorized to an organization. Deprecated, use /orgs/{org_id}/apps/installs instead.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_id** | **uuid::Uuid** | Org ID | [required] |
**version** | **String** | The requested version of the endpoint to process the request | [required] |
**expand** | Option<[**Vec<String>**](String.md)> | Expand relationships. |  |
**starting_after** | Option<**String**> | Return the page of results immediately after this cursor |  |
**ending_before** | Option<**String**> | Return the page of results immediately before this cursor |  |
**limit** | Option<**i32**> | Number of results to return per page |  |[default to 10]

### Return type

[**crate::models::GetAppBots200Response**](getAppBots_200_response.md)

### Authorization

[APIToken](../README.md#APIToken), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_app_by_id

> crate::models::GetAppById200Response get_app_by_id(org_id, app_id, version)
Get a Snyk App by its App ID.

Get a Snyk App by its App ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_id** | **uuid::Uuid** | Org ID | [required] |
**app_id** | **uuid::Uuid** | App ID | [required] |
**version** | **String** | The requested version of the endpoint to process the request | [required] |

### Return type

[**crate::models::GetAppById200Response**](getAppByID_200_response.md)

### Authorization

[APIToken](../README.md#APIToken), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_app_installs_for_group

> crate::models::GetAppInstallsForGroup200Response get_app_installs_for_group(group_id, version, expand, starting_after, ending_before, limit)
Get a list of apps installed for a group.

Get a list of apps installed for a group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **uuid::Uuid** | Group ID | [required] |
**version** | **String** | The requested version of the endpoint to process the request | [required] |
**expand** | Option<[**Vec<String>**](String.md)> | Expand relationships. |  |
**starting_after** | Option<**String**> | Return the page of results immediately after this cursor |  |
**ending_before** | Option<**String**> | Return the page of results immediately before this cursor |  |
**limit** | Option<**i32**> | Number of results to return per page |  |[default to 10]

### Return type

[**crate::models::GetAppInstallsForGroup200Response**](getAppInstallsForGroup_200_response.md)

### Authorization

[APIToken](../README.md#APIToken), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_app_installs_for_org

> crate::models::GetAppInstallsForGroup200Response get_app_installs_for_org(org_id, version, expand, starting_after, ending_before, limit)
Get a list of apps installed for an organization.

Get a list of apps installed for an organization.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_id** | **uuid::Uuid** | Org ID | [required] |
**version** | **String** | The requested version of the endpoint to process the request | [required] |
**expand** | Option<[**Vec<String>**](String.md)> | Expand relationships. |  |
**starting_after** | Option<**String**> | Return the page of results immediately after this cursor |  |
**ending_before** | Option<**String**> | Return the page of results immediately before this cursor |  |
**limit** | Option<**i32**> | Number of results to return per page |  |[default to 10]

### Return type

[**crate::models::GetAppInstallsForGroup200Response**](getAppInstallsForGroup_200_response.md)

### Authorization

[APIToken](../README.md#APIToken), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_app_installs_for_user

> crate::models::GetAppInstallsForGroup200Response get_app_installs_for_user(version, expand, starting_after, ending_before, limit)
Get a list of apps installed for an user.

Get a list of apps installed for an user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**version** | **String** | The requested version of the endpoint to process the request | [required] |
**expand** | Option<[**Vec<String>**](String.md)> | Expand relationships. |  |
**starting_after** | Option<**String**> | Return the page of results immediately after this cursor |  |
**ending_before** | Option<**String**> | Return the page of results immediately before this cursor |  |
**limit** | Option<**i32**> | Number of results to return per page |  |[default to 10]

### Return type

[**crate::models::GetAppInstallsForGroup200Response**](getAppInstallsForGroup_200_response.md)

### Authorization

[APIToken](../README.md#APIToken), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_apps

> crate::models::GetApps200Response get_apps(org_id, version, starting_after, ending_before, limit)
Get a list of apps created by an organization.

Get a list of apps created by an organization. Deprecated, use /orgs/{org_id}/apps/creations instead.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_id** | **uuid::Uuid** | Org ID | [required] |
**version** | **String** | The requested version of the endpoint to process the request | [required] |
**starting_after** | Option<**String**> | Return the page of results immediately after this cursor |  |
**ending_before** | Option<**String**> | Return the page of results immediately before this cursor |  |
**limit** | Option<**i32**> | Number of results to return per page |  |[default to 10]

### Return type

[**crate::models::GetApps200Response**](getApps_200_response.md)

### Authorization

[APIToken](../README.md#APIToken), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_org_apps

> crate::models::GetApps200Response get_org_apps(org_id, version, starting_after, ending_before, limit)
Get a list of apps created by an organization.

Get a list of apps created by an organization.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_id** | **uuid::Uuid** | Org ID | [required] |
**version** | **String** | The requested version of the endpoint to process the request | [required] |
**starting_after** | Option<**String**> | Return the page of results immediately after this cursor |  |
**ending_before** | Option<**String**> | Return the page of results immediately before this cursor |  |
**limit** | Option<**i32**> | Number of results to return per page |  |[default to 10]

### Return type

[**crate::models::GetApps200Response**](getApps_200_response.md)

### Authorization

[APIToken](../README.md#APIToken), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_app_sessions

> crate::models::GetUserAppSessions200Response get_user_app_sessions(version, app_id, starting_after, ending_before, limit)
Get a list of active OAuth sessions for the app.

Get a list of active OAuth sessions for the app.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**version** | **String** | The requested version of the endpoint to process the request | [required] |
**app_id** | **uuid::Uuid** | App ID | [required] |
**starting_after** | Option<**String**> | Return the page of results immediately after this cursor |  |
**ending_before** | Option<**String**> | Return the page of results immediately before this cursor |  |
**limit** | Option<**i32**> | Number of results to return per page |  |[default to 10]

### Return type

[**crate::models::GetUserAppSessions200Response**](getUserAppSessions_200_response.md)

### Authorization

[APIToken](../README.md#APIToken), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user_installed_apps

> crate::models::GetUserInstalledApps200Response get_user_installed_apps(version, starting_after, ending_before, limit)
Get a list of apps that can act on your behalf.

Get a list of apps that can act on your behalf.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**version** | **String** | The requested version of the endpoint to process the request | [required] |
**starting_after** | Option<**String**> | Return the page of results immediately after this cursor |  |
**ending_before** | Option<**String**> | Return the page of results immediately before this cursor |  |
**limit** | Option<**i32**> | Number of results to return per page |  |[default to 10]

### Return type

[**crate::models::GetUserInstalledApps200Response**](getUserInstalledApps_200_response.md)

### Authorization

[APIToken](../README.md#APIToken), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## manage_app_creation_secret

> crate::models::ManageAppCreationSecret200Response manage_app_creation_secret(version, org_id, app_id, update_group_app_install_secret_request)
Manage client secret for the Snyk App.

Manage client secret for the Snyk App.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**version** | **String** | The requested version of the endpoint to process the request | [required] |
**org_id** | **uuid::Uuid** | Org ID | [required] |
**app_id** | **uuid::Uuid** | App ID | [required] |
**update_group_app_install_secret_request** | Option<[**UpdateGroupAppInstallSecretRequest**](UpdateGroupAppInstallSecretRequest.md)> |  |  |

### Return type

[**crate::models::ManageAppCreationSecret200Response**](manageAppCreationSecret_200_response.md)

### Authorization

[APIToken](../README.md#APIToken), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/vnd.api+json
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## manage_secrets

> crate::models::ManageAppCreationSecret200Response manage_secrets(version, org_id, client_id, update_group_app_install_secret_request_data_attributes)
Manage client secrets for an app.

Manage client secrets for an app. Deprecated, use /orgs/{org_id}/apps/creations/{app_id}/secrets instead.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**version** | **String** | The requested version of the endpoint to process the request | [required] |
**org_id** | **uuid::Uuid** | Org ID | [required] |
**client_id** | **uuid::Uuid** | Client ID | [required] |
**update_group_app_install_secret_request_data_attributes** | Option<[**UpdateGroupAppInstallSecretRequestDataAttributes**](UpdateGroupAppInstallSecretRequestDataAttributes.md)> |  |  |

### Return type

[**crate::models::ManageAppCreationSecret200Response**](manageAppCreationSecret_200_response.md)

### Authorization

[APIToken](../README.md#APIToken), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/vnd.api+json
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## revoke_user_app_session

> revoke_user_app_session(version, app_id, session_id)
Revoke an active user app session.

Revoke an active user app session.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**version** | **String** | The requested version of the endpoint to process the request | [required] |
**app_id** | **uuid::Uuid** | App ID | [required] |
**session_id** | **uuid::Uuid** | Session ID | [required] |

### Return type

 (empty response body)

### Authorization

[APIToken](../README.md#APIToken), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## revoke_user_installed_app

> revoke_user_installed_app(version, app_id)
Revoke an app

Revoke access for an app by app id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**version** | **String** | The requested version of the endpoint to process the request | [required] |
**app_id** | **uuid::Uuid** | App ID | [required] |

### Return type

 (empty response body)

### Authorization

[APIToken](../README.md#APIToken), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_app

> crate::models::GetAppById200Response update_app(version, org_id, client_id, app_patch_request)
Update app attributes that are name, redirect URIs, and access token time to live

Update app attributes. Deprecated, use /orgs/{org_id}/apps/creations/{app_id} instead.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**version** | **String** | The requested version of the endpoint to process the request | [required] |
**org_id** | **uuid::Uuid** | Org ID | [required] |
**client_id** | **uuid::Uuid** | Client ID | [required] |
**app_patch_request** | Option<[**AppPatchRequest**](AppPatchRequest.md)> |  |  |

### Return type

[**crate::models::GetAppById200Response**](getAppByID_200_response.md)

### Authorization

[APIToken](../README.md#APIToken), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/vnd.api+json
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_app_creation_by_id

> crate::models::GetAppById200Response update_app_creation_by_id(version, org_id, app_id, app_patch_request)
Update app creation attributes such as name, redirect URIs, and access token time to live using the App ID.

Update app creation attributes with App ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**version** | **String** | The requested version of the endpoint to process the request | [required] |
**org_id** | **uuid::Uuid** | Org ID | [required] |
**app_id** | **uuid::Uuid** | App ID | [required] |
**app_patch_request** | Option<[**AppPatchRequest**](AppPatchRequest.md)> |  |  |

### Return type

[**crate::models::GetAppById200Response**](getAppByID_200_response.md)

### Authorization

[APIToken](../README.md#APIToken), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/vnd.api+json
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_group_app_install_secret

> crate::models::UpdateGroupAppInstallSecret200Response update_group_app_install_secret(version, group_id, install_id, update_group_app_install_secret_request)
Manage client secret for non-interactive Snyk App installations.

Manage client secret for non-interactive Snyk App installations.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**version** | **String** | The requested version of the endpoint to process the request | [required] |
**group_id** | **uuid::Uuid** | Group ID | [required] |
**install_id** | **uuid::Uuid** | Install ID | [required] |
**update_group_app_install_secret_request** | Option<[**UpdateGroupAppInstallSecretRequest**](UpdateGroupAppInstallSecretRequest.md)> |  |  |

### Return type

[**crate::models::UpdateGroupAppInstallSecret200Response**](updateGroupAppInstallSecret_200_response.md)

### Authorization

[APIToken](../README.md#APIToken), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/vnd.api+json
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_org_app_install_secret

> crate::models::UpdateGroupAppInstallSecret200Response update_org_app_install_secret(version, org_id, install_id, update_group_app_install_secret_request)
Manage client secret for non-interactive Snyk App installations.

Manage client secret for non-interactive Snyk App installations.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**version** | **String** | The requested version of the endpoint to process the request | [required] |
**org_id** | **uuid::Uuid** | Org ID | [required] |
**install_id** | **uuid::Uuid** | Install ID | [required] |
**update_group_app_install_secret_request** | Option<[**UpdateGroupAppInstallSecretRequest**](UpdateGroupAppInstallSecretRequest.md)> |  |  |

### Return type

[**crate::models::UpdateGroupAppInstallSecret200Response**](updateGroupAppInstallSecret_200_response.md)

### Authorization

[APIToken](../README.md#APIToken), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/vnd.api+json
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

