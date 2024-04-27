# \SbomApi

All URIs are relative to *https://api.snyk.io/rest*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_sbom**](SbomApi.md#get_sbom) | **GET** /orgs/{org_id}/projects/{project_id}/sbom | Get a project’s SBOM document



## get_sbom

> ::std::collections::HashMap<String, serde_json::Value> get_sbom(version, org_id, project_id, format)
Get a project’s SBOM document

This endpoint lets you retrieve the SBOM document of a software project. It supports the following formats: * CycloneDX version 1.4 in JSON (set `format` to `cyclonedx1.4+json`). * CycloneDX version 1.4 in XML (set `format` to `cyclonedx1.4+xml`). * SPDX version 2.3 in JSON (set `format` to `spdx2.3+json`).  By default it will respond with an empty JSON:API response.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**version** | **String** | The requested version of the endpoint to process the request | [required] |
**org_id** | **uuid::Uuid** | Unique identifier for an organization | [required] |
**project_id** | **uuid::Uuid** | Unique identifier for a project | [required] |
**format** | Option<**String**> | The desired SBOM format of the response. |  |

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[APIToken](../README.md#APIToken), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/vnd.api+json, application/vnd.cyclonedx+json, application/vnd.cyclonedx+xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

