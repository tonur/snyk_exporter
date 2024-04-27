# \ServiceAccountsApi

All URIs are relative to *https://api.snyk.io/rest*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_group_service_account**](ServiceAccountsApi.md#create_group_service_account) | **POST** /groups/{group_id}/service_accounts | Create a service account for a group.
[**create_org_service_account**](ServiceAccountsApi.md#create_org_service_account) | **POST** /orgs/{org_id}/service_accounts | Create a service account for an organization.
[**delete_one_group_service_account**](ServiceAccountsApi.md#delete_one_group_service_account) | **DELETE** /groups/{group_id}/service_accounts/{serviceaccount_id} | Delete a group service account.
[**delete_service_account**](ServiceAccountsApi.md#delete_service_account) | **DELETE** /orgs/{org_id}/service_accounts/{serviceaccount_id} | Delete a service account in an organization.
[**get_many_group_service_account**](ServiceAccountsApi.md#get_many_group_service_account) | **GET** /groups/{group_id}/service_accounts | Get a list of group service accounts.
[**get_many_org_service_accounts**](ServiceAccountsApi.md#get_many_org_service_accounts) | **GET** /orgs/{org_id}/service_accounts | Get a list of organization service accounts.
[**get_one_group_service_account**](ServiceAccountsApi.md#get_one_group_service_account) | **GET** /groups/{group_id}/service_accounts/{serviceaccount_id} | Get a group service account.
[**get_one_org_service_account**](ServiceAccountsApi.md#get_one_org_service_account) | **GET** /orgs/{org_id}/service_accounts/{serviceaccount_id} | Get an organization service account.
[**update_group_service_account**](ServiceAccountsApi.md#update_group_service_account) | **PATCH** /groups/{group_id}/service_accounts/{serviceaccount_id} | Update a group service account.
[**update_org_service_account**](ServiceAccountsApi.md#update_org_service_account) | **PATCH** /orgs/{org_id}/service_accounts/{serviceaccount_id} | Update an organization service account.
[**update_org_service_account_secret**](ServiceAccountsApi.md#update_org_service_account_secret) | **POST** /orgs/{org_id}/service_accounts/{serviceaccount_id}/secrets | Manage an organization service account's client secret.
[**update_service_account_secret**](ServiceAccountsApi.md#update_service_account_secret) | **POST** /groups/{group_id}/service_accounts/{serviceaccount_id}/secrets | Manage a group service account's client secret.



## create_group_service_account

> crate::models::CreateGroupServiceAccount201Response create_group_service_account(group_id, version, create_group_service_account_request)
Create a service account for a group.

Create a service account for a group. The service account can be used to generate access tokens. Currently, we only allow group service accounts using default group roles.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **uuid::Uuid** | The ID of the Snyk Group that is creating and owns the service account | [required] |
**version** | **String** | The requested version of the endpoint to process the request | [required] |
**create_group_service_account_request** | [**CreateGroupServiceAccountRequest**](CreateGroupServiceAccountRequest.md) |  | [required] |

### Return type

[**crate::models::CreateGroupServiceAccount201Response**](createGroupServiceAccount_201_response.md)

### Authorization

[APIToken](../README.md#APIToken), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/vnd.api+json
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_org_service_account

> crate::models::GetOneGroupServiceAccount200Response create_org_service_account(org_id, version, create_org_service_account_request)
Create a service account for an organization.

Create a service account for an organization. The service account can be used to generate access tokens.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_id** | **uuid::Uuid** | The ID of the Snyk Organization that is creating and will own the service account. | [required] |
**version** | **String** | The requested version of the endpoint to process the request | [required] |
**create_org_service_account_request** | [**CreateOrgServiceAccountRequest**](CreateOrgServiceAccountRequest.md) |  | [required] |

### Return type

[**crate::models::GetOneGroupServiceAccount200Response**](getOneGroupServiceAccount_200_response.md)

### Authorization

[APIToken](../README.md#APIToken), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/vnd.api+json
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_one_group_service_account

> delete_one_group_service_account(group_id, serviceaccount_id, version)
Delete a group service account.

Permanently delete a group-level service account by its ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **uuid::Uuid** | The ID of the Snyk Group that owns the service account. | [required] |
**serviceaccount_id** | **uuid::Uuid** | The ID of the service account. | [required] |
**version** | **String** | The requested version of the endpoint to process the request | [required] |

### Return type

 (empty response body)

### Authorization

[APIToken](../README.md#APIToken), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_service_account

> delete_service_account(org_id, serviceaccount_id, version)
Delete a service account in an organization.

Delete a service account in an organization.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_id** | **uuid::Uuid** | The ID of org to which the service account belongs. | [required] |
**serviceaccount_id** | **uuid::Uuid** | The ID of the service account. | [required] |
**version** | **String** | The requested version of the endpoint to process the request | [required] |

### Return type

 (empty response body)

### Authorization

[APIToken](../README.md#APIToken), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_many_group_service_account

> crate::models::GetManyGroupServiceAccount200Response get_many_group_service_account(group_id, version, starting_after, ending_before, limit)
Get a list of group service accounts.

Get all service accounts for a group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **uuid::Uuid** | The ID of the Snyk Group that owns the service accounts. | [required] |
**version** | **String** | The requested version of the endpoint to process the request | [required] |
**starting_after** | Option<**String**> | Return the page of results immediately after this cursor |  |
**ending_before** | Option<**String**> | Return the page of results immediately before this cursor |  |
**limit** | Option<**i32**> | Number of results to return per page |  |[default to 10]

### Return type

[**crate::models::GetManyGroupServiceAccount200Response**](getManyGroupServiceAccount_200_response.md)

### Authorization

[APIToken](../README.md#APIToken), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_many_org_service_accounts

> crate::models::GetManyGroupServiceAccount200Response get_many_org_service_accounts(org_id, version, starting_after, ending_before, limit)
Get a list of organization service accounts.

Get all service accounts for an organization.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_id** | **uuid::Uuid** | The ID of the Snyk Organization that owns the service accounts. | [required] |
**version** | **String** | The requested version of the endpoint to process the request | [required] |
**starting_after** | Option<**String**> | Return the page of results immediately after this cursor |  |
**ending_before** | Option<**String**> | Return the page of results immediately before this cursor |  |
**limit** | Option<**i32**> | Number of results to return per page |  |[default to 10]

### Return type

[**crate::models::GetManyGroupServiceAccount200Response**](getManyGroupServiceAccount_200_response.md)

### Authorization

[APIToken](../README.md#APIToken), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_one_group_service_account

> crate::models::GetOneGroupServiceAccount200Response get_one_group_service_account(group_id, serviceaccount_id, version)
Get a group service account.

Get a group-level service account by its ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **uuid::Uuid** | The ID of the Snyk Group that owns the service account. | [required] |
**serviceaccount_id** | **uuid::Uuid** | The ID of the service account. | [required] |
**version** | **String** | The requested version of the endpoint to process the request | [required] |

### Return type

[**crate::models::GetOneGroupServiceAccount200Response**](getOneGroupServiceAccount_200_response.md)

### Authorization

[APIToken](../README.md#APIToken), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_one_org_service_account

> crate::models::GetOneGroupServiceAccount200Response get_one_org_service_account(org_id, serviceaccount_id, version)
Get an organization service account.

Get an organization-level service account by its ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_id** | **uuid::Uuid** | The ID of the Snyk Organization that owns the service account. | [required] |
**serviceaccount_id** | **uuid::Uuid** | The ID of the service account. | [required] |
**version** | **String** | The requested version of the endpoint to process the request | [required] |

### Return type

[**crate::models::GetOneGroupServiceAccount200Response**](getOneGroupServiceAccount_200_response.md)

### Authorization

[APIToken](../README.md#APIToken), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_group_service_account

> crate::models::GetOneGroupServiceAccount200Response update_group_service_account(group_id, serviceaccount_id, version, update_group_service_account_request)
Update a group service account.

Update the name of a group's service account by its ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **uuid::Uuid** | The ID of the Snyk Group that owns the service account. | [required] |
**serviceaccount_id** | **uuid::Uuid** | The ID of the service account. | [required] |
**version** | **String** | The requested version of the endpoint to process the request | [required] |
**update_group_service_account_request** | [**UpdateGroupServiceAccountRequest**](UpdateGroupServiceAccountRequest.md) |  | [required] |

### Return type

[**crate::models::GetOneGroupServiceAccount200Response**](getOneGroupServiceAccount_200_response.md)

### Authorization

[APIToken](../README.md#APIToken), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/vnd.api+json
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_org_service_account

> crate::models::GetOneGroupServiceAccount200Response update_org_service_account(org_id, serviceaccount_id, version, update_org_service_account_request)
Update an organization service account.

Update the name of an organization-level service account by its ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_id** | **uuid::Uuid** | The ID of the Snyk Organization that owns the service account. | [required] |
**serviceaccount_id** | **uuid::Uuid** | The ID of the service account. | [required] |
**version** | **String** | The requested version of the endpoint to process the request | [required] |
**update_org_service_account_request** | [**UpdateOrgServiceAccountRequest**](UpdateOrgServiceAccountRequest.md) |  | [required] |

### Return type

[**crate::models::GetOneGroupServiceAccount200Response**](getOneGroupServiceAccount_200_response.md)

### Authorization

[APIToken](../README.md#APIToken), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/vnd.api+json
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_org_service_account_secret

> crate::models::CreateGroupServiceAccount201Response update_org_service_account_secret(org_id, serviceaccount_id, version, update_service_account_secret_request)
Manage an organization service account's client secret.

Manage the client secret of an organization-level service account by the service account ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_id** | **uuid::Uuid** | The ID of the Snyk Organization that owns the service account. | [required] |
**serviceaccount_id** | **uuid::Uuid** | The ID of the service account. | [required] |
**version** | **String** | The requested version of the endpoint to process the request | [required] |
**update_service_account_secret_request** | [**UpdateServiceAccountSecretRequest**](UpdateServiceAccountSecretRequest.md) |  | [required] |

### Return type

[**crate::models::CreateGroupServiceAccount201Response**](createGroupServiceAccount_201_response.md)

### Authorization

[APIToken](../README.md#APIToken), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/vnd.api+json
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_service_account_secret

> crate::models::CreateGroupServiceAccount201Response update_service_account_secret(group_id, serviceaccount_id, version, update_service_account_secret_request)
Manage a group service account's client secret.

Manage the client secret of a group service account by the service account ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **uuid::Uuid** | The ID of the Snyk Group that owns the service account. | [required] |
**serviceaccount_id** | **uuid::Uuid** | The ID of the service account. | [required] |
**version** | **String** | The requested version of the endpoint to process the request | [required] |
**update_service_account_secret_request** | [**UpdateServiceAccountSecretRequest**](UpdateServiceAccountSecretRequest.md) |  | [required] |

### Return type

[**crate::models::CreateGroupServiceAccount201Response**](createGroupServiceAccount_201_response.md)

### Authorization

[APIToken](../README.md#APIToken), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/vnd.api+json
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

