# \InvitesApi

All URIs are relative to *https://api.snyk.io/rest*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_org_invitation**](InvitesApi.md#create_org_invitation) | **POST** /orgs/{org_id}/invites | Invite a user to an organization
[**delete_org_invitation**](InvitesApi.md#delete_org_invitation) | **DELETE** /orgs/{org_id}/invites/{invite_id} | Cancel a pending user invitations to an organization.
[**list_org_invitation**](InvitesApi.md#list_org_invitation) | **GET** /orgs/{org_id}/invites | List pending user invitations to an organization.



## create_org_invitation

> crate::models::CreateOrgInvitation201Response create_org_invitation(version, org_id, create_org_invitation_request)
Invite a user to an organization

Invite a user to an organization with a role.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**version** | **String** | The requested version of the endpoint to process the request | [required] |
**org_id** | **uuid::Uuid** | The id of the org the user is being invited to | [required] |
**create_org_invitation_request** | Option<[**CreateOrgInvitationRequest**](CreateOrgInvitationRequest.md)> |  |  |

### Return type

[**crate::models::CreateOrgInvitation201Response**](createOrgInvitation_201_response.md)

### Authorization

[APIToken](../README.md#APIToken), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/vnd.api+json
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_org_invitation

> delete_org_invitation(org_id, invite_id, version)
Cancel a pending user invitations to an organization.

Cancel a pending user invitations to an organization.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_id** | **uuid::Uuid** | The id of the org the user is being invited to | [required] |
**invite_id** | **uuid::Uuid** | The id of the pending invite to cancel | [required] |
**version** | **String** | The requested version of the endpoint to process the request | [required] |

### Return type

 (empty response body)

### Authorization

[APIToken](../README.md#APIToken), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_org_invitation

> crate::models::ListOrgInvitation200Response list_org_invitation(org_id, version, starting_after, ending_before, limit)
List pending user invitations to an organization.

List pending user invitations to an organization.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_id** | **uuid::Uuid** | The id of the org the user is being invited to | [required] |
**version** | **String** | The requested version of the endpoint to process the request | [required] |
**starting_after** | Option<**String**> | Return the page of results immediately after this cursor |  |
**ending_before** | Option<**String**> | Return the page of results immediately before this cursor |  |
**limit** | Option<**i32**> | Number of results to return per page |  |[default to 10]

### Return type

[**crate::models::ListOrgInvitation200Response**](listOrgInvitation_200_response.md)

### Authorization

[APIToken](../README.md#APIToken), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

