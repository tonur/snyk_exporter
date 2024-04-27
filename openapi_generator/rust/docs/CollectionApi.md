# \CollectionApi

All URIs are relative to *https://api.snyk.io/rest*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_collection**](CollectionApi.md#create_collection) | **POST** /orgs/{org_id}/collections | Create a collection
[**delete_collection**](CollectionApi.md#delete_collection) | **DELETE** /orgs/{org_id}/collections/{collection_id} | Delete a collection
[**delete_projects_collection**](CollectionApi.md#delete_projects_collection) | **DELETE** /orgs/{org_id}/collections/{collection_id}/relationships/projects | Remove projects from a collection
[**get_collection**](CollectionApi.md#get_collection) | **GET** /orgs/{org_id}/collections/{collection_id} | Get a collection
[**get_collections**](CollectionApi.md#get_collections) | **GET** /orgs/{org_id}/collections | Get collections
[**get_projects_of_collection**](CollectionApi.md#get_projects_of_collection) | **GET** /orgs/{org_id}/collections/{collection_id}/relationships/projects | Get projects from the specified collection
[**update_collection**](CollectionApi.md#update_collection) | **PATCH** /orgs/{org_id}/collections/{collection_id} | Edit a collection
[**update_collection_with_projects**](CollectionApi.md#update_collection_with_projects) | **POST** /orgs/{org_id}/collections/{collection_id}/relationships/projects | Add projects to a collection



## create_collection

> crate::models::CreateCollection201Response create_collection(version, org_id, create_collection_request)
Create a collection

Create a collection

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**version** | **String** | The requested version of the endpoint to process the request | [required] |
**org_id** | **uuid::Uuid** | Org ID | [required] |
**create_collection_request** | Option<[**CreateCollectionRequest**](CreateCollectionRequest.md)> |  |  |

### Return type

[**crate::models::CreateCollection201Response**](createCollection_201_response.md)

### Authorization

[APIToken](../README.md#APIToken), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/vnd.api+json
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_collection

> delete_collection(version, org_id, collection_id)
Delete a collection

Delete a collection

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**version** | **String** | The requested version of the endpoint to process the request | [required] |
**org_id** | **uuid::Uuid** | Org ID | [required] |
**collection_id** | **uuid::Uuid** | Unique identifier for a collection | [required] |

### Return type

 (empty response body)

### Authorization

[APIToken](../README.md#APIToken), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_projects_collection

> delete_projects_collection(version, org_id, collection_id, delete_projects_from_collection_request)
Remove projects from a collection

Remove projects from a collection by specifying an array of project ids

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**version** | **String** | The requested version of the endpoint to process the request | [required] |
**org_id** | **uuid::Uuid** | Org ID | [required] |
**collection_id** | **uuid::Uuid** | Unique identifier for a collection | [required] |
**delete_projects_from_collection_request** | Option<[**DeleteProjectsFromCollectionRequest**](DeleteProjectsFromCollectionRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[APIToken](../README.md#APIToken), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/vnd.api+json
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_collection

> crate::models::CreateCollection201Response get_collection(version, org_id, collection_id)
Get a collection

Get a collection

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**version** | **String** | The requested version of the endpoint to process the request | [required] |
**org_id** | **uuid::Uuid** | Org ID | [required] |
**collection_id** | **uuid::Uuid** | Unique identifier for a collection | [required] |

### Return type

[**crate::models::CreateCollection201Response**](createCollection_201_response.md)

### Authorization

[APIToken](../README.md#APIToken), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_collections

> crate::models::GetCollections200Response get_collections(version, org_id, starting_after, ending_before, limit, sort, direction, name, is_generated)
Get collections

Return a list of organization's collections with issues counts  and projects count.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**version** | **String** | The requested version of the endpoint to process the request | [required] |
**org_id** | **uuid::Uuid** | Org ID | [required] |
**starting_after** | Option<**String**> | Return the page of results immediately after this cursor |  |
**ending_before** | Option<**String**> | Return the page of results immediately before this cursor |  |
**limit** | Option<**i32**> | Number of results to return per page |  |[default to 10]
**sort** | Option<**String**> | Return collections sorted by the specified attributes |  |
**direction** | Option<**String**> | Return collections sorted in the specified direction |  |[default to DESC]
**name** | Option<**String**> | Return collections which names include the provided string |  |
**is_generated** | Option<**bool**> | Return collections where is_generated matches the provided boolean |  |

### Return type

[**crate::models::GetCollections200Response**](getCollections_200_response.md)

### Authorization

[APIToken](../README.md#APIToken), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_projects_of_collection

> crate::models::GetProjectsOfCollectionResponse get_projects_of_collection(version, org_id, collection_id, starting_after, ending_before, limit, sort, direction, target_id, show, integration)
Get projects from the specified collection

Return a list of organization's projects that are from the specified collection.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**version** | **String** | The requested version of the endpoint to process the request | [required] |
**org_id** | **uuid::Uuid** | Org ID | [required] |
**collection_id** | **uuid::Uuid** | Unique identifier for a collection | [required] |
**starting_after** | Option<**String**> | Return the page of results immediately after this cursor |  |
**ending_before** | Option<**String**> | Return the page of results immediately before this cursor |  |
**limit** | Option<**i32**> | Number of results to return per page |  |[default to 10]
**sort** | Option<**String**> | Return projects sorted by the specified attributes |  |
**direction** | Option<**String**> | Return projects sorted in the specified direction |  |[default to DESC]
**target_id** | Option<[**Vec<uuid::Uuid>**](uuid::Uuid.md)> | Return projects that belong to the provided targets |  |
**show** | Option<[**Vec<String>**](String.md)> | Return projects that are with or without issues |  |
**integration** | Option<[**Vec<String>**](String.md)> | Return projects that match the provided integration types |  |

### Return type

[**crate::models::GetProjectsOfCollectionResponse**](GetProjectsOfCollectionResponse.md)

### Authorization

[APIToken](../README.md#APIToken), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_collection

> crate::models::CreateCollection201Response update_collection(version, org_id, collection_id, update_collection_request)
Edit a collection

Edit a collection

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**version** | **String** | The requested version of the endpoint to process the request | [required] |
**org_id** | **uuid::Uuid** | Org ID | [required] |
**collection_id** | **uuid::Uuid** | Unique identifier for a collection | [required] |
**update_collection_request** | Option<[**UpdateCollectionRequest**](UpdateCollectionRequest.md)> |  |  |

### Return type

[**crate::models::CreateCollection201Response**](createCollection_201_response.md)

### Authorization

[APIToken](../README.md#APIToken), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/vnd.api+json
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_collection_with_projects

> update_collection_with_projects(version, org_id, collection_id, update_collection_with_projects_request)
Add projects to a collection

Add projects to a collection by specifying an array of project ids

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**version** | **String** | The requested version of the endpoint to process the request | [required] |
**org_id** | **uuid::Uuid** | Org ID | [required] |
**collection_id** | **uuid::Uuid** | Unique identifier for a collection | [required] |
**update_collection_with_projects_request** | Option<[**UpdateCollectionWithProjectsRequest**](UpdateCollectionWithProjectsRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[APIToken](../README.md#APIToken), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/vnd.api+json
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

