# \CustomBaseImagesApi

All URIs are relative to *https://api.snyk.io/rest*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_custom_base_image**](CustomBaseImagesApi.md#create_custom_base_image) | **POST** /custom_base_images | Create a Custom Base Image from an existing container project
[**delete_custom_base_image**](CustomBaseImagesApi.md#delete_custom_base_image) | **DELETE** /custom_base_images/{custombaseimage_id} | Delete a custom base image
[**get_custom_base_image**](CustomBaseImagesApi.md#get_custom_base_image) | **GET** /custom_base_images/{custombaseimage_id} | Get a custom base image
[**get_custom_base_images**](CustomBaseImagesApi.md#get_custom_base_images) | **GET** /custom_base_images | Get a custom base image collection
[**update_custom_base_image**](CustomBaseImagesApi.md#update_custom_base_image) | **PATCH** /custom_base_images/{custombaseimage_id} | Update a custom base image



## create_custom_base_image

> crate::models::CustomBaseImageResponse create_custom_base_image(version, custom_base_image_post_request)
Create a Custom Base Image from an existing container project

In order to create a custom base image, you first need to import your base images into Snyk. You can do this through the CLI, UI, or API.  This endpoint marks an image as a custom base image. This means that the image will get added to the pool of images from which Snyk can recommend base image upgrades.  Note, after the first image in a repository gets added, a versioning schema cannot be passed in this endpoint. To update the versioning schema, the PATCH endpoint must be used. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**version** | **String** | The requested version of the endpoint to process the request | [required] |
**custom_base_image_post_request** | Option<[**CustomBaseImagePostRequest**](CustomBaseImagePostRequest.md)> | The properties used in the creation of a custom base image |  |

### Return type

[**crate::models::CustomBaseImageResponse**](CustomBaseImageResponse.md)

### Authorization

[APIToken](../README.md#APIToken), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/vnd.api+json
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_custom_base_image

> delete_custom_base_image(version, custombaseimage_id)
Delete a custom base image

Delete a custom base image resource. (the related container project is unaffected)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**version** | **String** | The requested version of the endpoint to process the request | [required] |
**custombaseimage_id** | **uuid::Uuid** | Unique identifier for custom base image | [required] |

### Return type

 (empty response body)

### Authorization

[APIToken](../README.md#APIToken), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_custom_base_image

> crate::models::CustomBaseImageResponse get_custom_base_image(version, custombaseimage_id)
Get a custom base image

Get a custom base image

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**version** | **String** | The requested version of the endpoint to process the request | [required] |
**custombaseimage_id** | **uuid::Uuid** | Unique identifier for custom base image | [required] |

### Return type

[**crate::models::CustomBaseImageResponse**](CustomBaseImageResponse.md)

### Authorization

[APIToken](../README.md#APIToken), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_custom_base_images

> crate::models::CustomBaseImageCollectionResponse get_custom_base_images(version, starting_after, ending_before, limit, project_id, org_id, group_id, repository, tag, include_in_recommendations, sort_by, sort_direction)
Get a custom base image collection

Get a list of custom base images with support for ordering and filtering. Either the org_id or group_id parameters must be set to authorize successfully. If sorting by version, the repository filter is required. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**version** | **String** | The requested version of the endpoint to process the request | [required] |
**starting_after** | Option<**String**> | Return the page of results immediately after this cursor |  |
**ending_before** | Option<**String**> | Return the page of results immediately before this cursor |  |
**limit** | Option<**i32**> | Number of results to return per page |  |[default to 10]
**project_id** | Option<**uuid::Uuid**> | The ID of the container project that the custom base image is based off of. |  |
**org_id** | Option<**uuid::Uuid**> | The organization ID of the custom base image |  |
**group_id** | Option<**uuid::Uuid**> | The group ID of the custom base image |  |
**repository** | Option<**String**> | The image repository |  |
**tag** | Option<**String**> | The image tag |  |
**include_in_recommendations** | Option<**bool**> | Whether this image should be recommended as a base image upgrade |  |
**sort_by** | Option<**String**> | Which column to sort by.  If sorting by version, the versioning schema is used.  |  |
**sort_direction** | Option<**String**> | Which direction to sort |  |[default to ASC]

### Return type

[**crate::models::CustomBaseImageCollectionResponse**](CustomBaseImageCollectionResponse.md)

### Authorization

[APIToken](../README.md#APIToken), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_custom_base_image

> crate::models::CustomBaseImageResponse update_custom_base_image(version, custombaseimage_id, custom_base_image_patch_request)
Update a custom base image

Updates a custom base image's attributes

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**version** | **String** | The requested version of the endpoint to process the request | [required] |
**custombaseimage_id** | **uuid::Uuid** | Unique identifier for custom base image | [required] |
**custom_base_image_patch_request** | Option<[**CustomBaseImagePatchRequest**](CustomBaseImagePatchRequest.md)> | custom base image to be updated |  |

### Return type

[**crate::models::CustomBaseImageResponse**](CustomBaseImageResponse.md)

### Authorization

[APIToken](../README.md#APIToken), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/vnd.api+json
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

