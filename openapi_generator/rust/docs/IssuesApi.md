# \IssuesApi

All URIs are relative to *https://api.snyk.io/rest*

Method | HTTP request | Description
------------- | ------------- | -------------
[**fetch_issues_per_purl**](IssuesApi.md#fetch_issues_per_purl) | **GET** /orgs/{org_id}/packages/{purl}/issues | List issues for a package
[**get_group_issue_by_issue_id**](IssuesApi.md#get_group_issue_by_issue_id) | **GET** /groups/{group_id}/issues/{issue_id} | Get an issue
[**get_org_issue_by_issue_id**](IssuesApi.md#get_org_issue_by_issue_id) | **GET** /orgs/{org_id}/issues/{issue_id} | Get an issue
[**list_group_issues**](IssuesApi.md#list_group_issues) | **GET** /groups/{group_id}/issues | Get issues by group ID
[**list_issues_for_many_purls**](IssuesApi.md#list_issues_for_many_purls) | **POST** /orgs/{org_id}/packages/issues | List issues for a given set of packages  (Currently not available to all customers)
[**list_org_issues**](IssuesApi.md#list_org_issues) | **GET** /orgs/{org_id}/issues | Get issues by org ID



## fetch_issues_per_purl

> crate::models::IssuesResponse fetch_issues_per_purl(version, purl, org_id, offset, limit)
List issues for a package

Query issues for a specific package version identified by Package URL (purl). Snyk returns only direct vulnerabilities. Transitive vulnerabilities (from dependencies) are not returned because they can vary depending on context.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**version** | **String** | The requested version of the endpoint to process the request | [required] |
**purl** | **String** | A URI-encoded Package URL (purl). Supported purl types are apk, cargo, cocoapods, composer, deb, gem, generic, golang, hex, maven, npm, nuget, pub, pypi, rpm, and swift. A version for the package is also required. | [required] |
**org_id** | **uuid::Uuid** | Unique identifier for an organization | [required] |
**offset** | Option<**f32**> | Specify the number of results to skip before returning results. Must be greater than or equal to 0. Default is 0. |  |
**limit** | Option<**f32**> | Specify the number of results to return. Must be greater than 0 and less than 1000. Default is 1000. |  |

### Return type

[**crate::models::IssuesResponse**](IssuesResponse.md)

### Authorization

[APIToken](../README.md#APIToken), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_group_issue_by_issue_id

> crate::models::GetGroupIssueByIssueId200Response get_group_issue_by_issue_id(version, group_id, issue_id)
Get an issue

Get an issue

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**version** | **String** | The requested version of the endpoint to process the request | [required] |
**group_id** | **uuid::Uuid** | Group ID | [required] |
**issue_id** | **uuid::Uuid** | Issue ID | [required] |

### Return type

[**crate::models::GetGroupIssueByIssueId200Response**](getGroupIssueByIssueID_200_response.md)

### Authorization

[APIToken](../README.md#APIToken), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_org_issue_by_issue_id

> crate::models::GetGroupIssueByIssueId200Response get_org_issue_by_issue_id(version, org_id, issue_id)
Get an issue

Get an issue

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**version** | **String** | The requested version of the endpoint to process the request | [required] |
**org_id** | **uuid::Uuid** | Org ID | [required] |
**issue_id** | **uuid::Uuid** | Issue ID | [required] |

### Return type

[**crate::models::GetGroupIssueByIssueId200Response**](getGroupIssueByIssueID_200_response.md)

### Authorization

[APIToken](../README.md#APIToken), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_group_issues

> crate::models::ListGroupIssues200Response list_group_issues(version, group_id, starting_after, ending_before, limit, scan_item_period_id, scan_item_period_type, r#type, updated_before, updated_after, created_before, created_after, effective_severity_level, status, ignored)
Get issues by group ID

Get a list of a group's issues.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**version** | **String** | The requested version of the endpoint to process the request | [required] |
**group_id** | **uuid::Uuid** | Group ID | [required] |
**starting_after** | Option<**String**> | Return the page of results immediately after this cursor |  |
**ending_before** | Option<**String**> | Return the page of results immediately before this cursor |  |
**limit** | Option<**i32**> | Number of results to return per page |  |[default to 10]
**scan_item_period_id** | Option<**uuid::Uuid**> | A scan item id to filter issues through their scan item relationship. |  |
**scan_item_period_type** | Option<[**ScanItemType**](.md)> | A scan item types to filter issues through their scan item relationship. |  |
**r#type** | Option<[**TypeDef**](.md)> | An issue type to filter issues. |  |
**updated_before** | Option<**String**> | A filter to select issues updated before this date. |  |
**updated_after** | Option<**String**> | A filter to select issues updated after this date. |  |
**created_before** | Option<**String**> | A filter to select issues created before this date. |  |
**created_after** | Option<**String**> | A filter to select issues created after this date. |  |
**effective_severity_level** | Option<[**Vec<String>**](String.md)> | One or more effective severity levels to filter issues. |  |
**status** | Option<[**Vec<String>**](String.md)> | An issue's status |  |
**ignored** | Option<**bool**> | Whether an issue is ignored or not. |  |

### Return type

[**crate::models::ListGroupIssues200Response**](listGroupIssues_200_response.md)

### Authorization

[APIToken](../README.md#APIToken), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_issues_for_many_purls

> crate::models::IssuesWithPurlsResponse list_issues_for_many_purls(version, org_id, bulk_package_urls_request_body)
List issues for a given set of packages  (Currently not available to all customers)

This endpoint is not available to all customers. If you are interested please contact support. Query issues for a batch of packages identified by Package URL (purl). Only direct vulnerabilities are returned, transitive vulnerabilities (from dependencies) are not returned because they can vary depending on context.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**version** | **String** | The requested version of the endpoint to process the request | [required] |
**org_id** | **uuid::Uuid** | Unique identifier for an organization | [required] |
**bulk_package_urls_request_body** | [**BulkPackageUrlsRequestBody**](BulkPackageUrlsRequestBody.md) |  | [required] |

### Return type

[**crate::models::IssuesWithPurlsResponse**](IssuesWithPurlsResponse.md)

### Authorization

[APIToken](../README.md#APIToken), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/vnd.api+json
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_org_issues

> crate::models::ListGroupIssues200Response list_org_issues(version, org_id, starting_after, ending_before, limit, scan_item_period_id, scan_item_period_type, r#type, updated_before, updated_after, created_before, created_after, effective_severity_level, status, ignored)
Get issues by org ID

Get a list of an organization's issues.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**version** | **String** | The requested version of the endpoint to process the request | [required] |
**org_id** | **uuid::Uuid** | Org ID | [required] |
**starting_after** | Option<**String**> | Return the page of results immediately after this cursor |  |
**ending_before** | Option<**String**> | Return the page of results immediately before this cursor |  |
**limit** | Option<**i32**> | Number of results to return per page |  |[default to 10]
**scan_item_period_id** | Option<**uuid::Uuid**> | A scan item id to filter issues through their scan item relationship. |  |
**scan_item_period_type** | Option<[**ScanItemType**](.md)> | A scan item types to filter issues through their scan item relationship. |  |
**r#type** | Option<[**TypeDef**](.md)> | An issue type to filter issues. |  |
**updated_before** | Option<**String**> | A filter to select issues updated before this date. |  |
**updated_after** | Option<**String**> | A filter to select issues updated after this date. |  |
**created_before** | Option<**String**> | A filter to select issues created before this date. |  |
**created_after** | Option<**String**> | A filter to select issues created after this date. |  |
**effective_severity_level** | Option<[**Vec<String>**](String.md)> | One or more effective severity levels to filter issues. |  |
**status** | Option<[**Vec<String>**](String.md)> | An issue's status |  |
**ignored** | Option<**bool**> | Whether an issue is ignored or not. |  |

### Return type

[**crate::models::ListGroupIssues200Response**](listGroupIssues_200_response.md)

### Authorization

[APIToken](../README.md#APIToken), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

