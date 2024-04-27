# \OpenApiApi

All URIs are relative to *https://api.snyk.io/rest*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_api_version**](OpenApiApi.md#get_api_version) | **GET** /openapi/{version} | 
[**list_api_versions**](OpenApiApi.md#list_api_versions) | **GET** /openapi | 



## get_api_version

> serde_json::Value get_api_version(version)


Get OpenAPI specification effective at version.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**version** | **String** | The requested version of the API | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[APIToken](../README.md#APIToken), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_api_versions

> Vec<String> list_api_versions()


List available versions of OpenAPI specification

### Parameters

This endpoint does not need any parameter.

### Return type

**Vec<String>**

### Authorization

[APIToken](../README.md#APIToken), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

