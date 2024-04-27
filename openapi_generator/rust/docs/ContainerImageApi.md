# \ContainerImageApi

All URIs are relative to *https://api.snyk.io/rest*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_container_image**](ContainerImageApi.md#get_container_image) | **GET** /orgs/{org_id}/container_images/{image_id} | Get instance of container image
[**list_container_image**](ContainerImageApi.md#list_container_image) | **GET** /orgs/{org_id}/container_images | List instances of container image
[**list_image_target_refs**](ContainerImageApi.md#list_image_target_refs) | **GET** /orgs/{org_id}/container_images/{image_id}/relationships/image_target_refs | List instances of image target references for a container image



## get_container_image

> crate::models::GetContainerImage200Response get_container_image(version, org_id, image_id)
Get instance of container image

Get instance of container image

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**version** | **String** | The requested version of the endpoint to process the request | [required] |
**org_id** | **uuid::Uuid** | Org ID | [required] |
**image_id** | **String** | Image ID | [required] |

### Return type

[**crate::models::GetContainerImage200Response**](getContainerImage_200_response.md)

### Authorization

[APIToken](../README.md#APIToken), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_container_image

> crate::models::ListContainerImage200Response list_container_image(org_id, version, image_ids, platform, names, limit, starting_after, ending_before)
List instances of container image

List instances of container image

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_id** | **uuid::Uuid** | Org ID | [required] |
**version** | **String** | The requested version of the endpoint to process the request | [required] |
**image_ids** | Option<[**Vec<String>**](String.md)> | A comma-separated list of Image IDs |  |
**platform** | Option<[**Platform**](.md)> | The image Operating System and processor architecture |  |
**names** | Option<[**Vec<String>**](String.md)> | The container registry names |  |
**limit** | Option<**i32**> | Number of results to return per page |  |[default to 10]
**starting_after** | Option<**String**> | Return the page of results immediately after this cursor |  |
**ending_before** | Option<**String**> | Return the page of results immediately before this cursor |  |

### Return type

[**crate::models::ListContainerImage200Response**](listContainerImage_200_response.md)

### Authorization

[APIToken](../README.md#APIToken), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_image_target_refs

> crate::models::ListImageTargetRefs200Response list_image_target_refs(org_id, image_id, version, limit, starting_after, ending_before)
List instances of image target references for a container image

List instances of image target references for a container image

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_id** | **uuid::Uuid** | Org ID | [required] |
**image_id** | **String** | Image ID | [required] |
**version** | **String** | The requested version of the endpoint to process the request | [required] |
**limit** | Option<**i32**> | Number of results to return per page |  |[default to 10]
**starting_after** | Option<**String**> | Return the page of results immediately after this cursor |  |
**ending_before** | Option<**String**> | Return the page of results immediately before this cursor |  |

### Return type

[**crate::models::ListImageTargetRefs200Response**](listImageTargetRefs_200_response.md)

### Authorization

[APIToken](../README.md#APIToken), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

