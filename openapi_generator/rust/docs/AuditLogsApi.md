# \AuditLogsApi

All URIs are relative to *https://api.snyk.io/rest*

Method | HTTP request | Description
------------- | ------------- | -------------
[**list_group_audit_logs**](AuditLogsApi.md#list_group_audit_logs) | **GET** /groups/{group_id}/audit_logs/search | Search Group audit logs.
[**list_org_audit_logs**](AuditLogsApi.md#list_org_audit_logs) | **GET** /orgs/{org_id}/audit_logs/search | Search Organization audit logs.



## list_group_audit_logs

> crate::models::ListGroupAuditLogs200Response list_group_audit_logs(version, group_id, cursor, from, to, size, sort_order, user_id, project_id, event, exclude_event)
Search Group audit logs.

Search audit logs for a Group. Some Organization level events are supported as well as the following Group level events:   - api.access   - group.cloud_config.settings.edit   - group.create   - group.delete   - group.edit   - group.notification_settings.edit   - group.org.add   - group.org.remove   - group.policy.create   - group.policy.delete   - group.policy.edit   - group.request_access_settings.edit   - group.role.create   - group.role.delete   - group.role.edit   - group.service_account.create   - group.service_account.delete   - group.service_account.edit   - group.settings.edit   - group.settings.feature_flag.edit   - group.sso.add   - group.sso.auth0_connection.create   - group.sso.auth0_connection.edit   - group.sso.create   - group.sso.delete   - group.sso.edit   - group.sso.membership.sync   - group.sso.remove   - group.tag.create   - group.tag.delete   - group.user.add   - group.user.remove   - group.user.role.edit 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**version** | **String** | The requested version of the endpoint to process the request | [required] |
**group_id** | **uuid::Uuid** | The ID of the Group. | [required] |
**cursor** | Option<**String**> | The ID for the next page of results. |  |
**from** | Option<**String**> | The start date (inclusive) of the audit logs search. If this is not specified, the start of yesterday is used. Example: 2023-07-27  |  |
**to** | Option<**String**> | The end date (inclusive) of the audit logs search. Example: 2023-07-27  |  |
**size** | Option<**i32**> | Number of results to return per page. |  |
**sort_order** | Option<**String**> | Order in which results are returned. |  |[default to DESC]
**user_id** | Option<**uuid::Uuid**> | Filter logs by user ID. |  |
**project_id** | Option<**uuid::Uuid**> | Filter logs by project ID. |  |
**event** | Option<**String**> | Filter logs by event type, cannot be used in conjunction with exclude_event parameter. |  |
**exclude_event** | Option<**String**> | Exclude event type from results, cannot be used in conjunctions with event parameter. |  |

### Return type

[**crate::models::ListGroupAuditLogs200Response**](listGroupAuditLogs_200_response.md)

### Authorization

[APIToken](../README.md#APIToken), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_org_audit_logs

> crate::models::ListGroupAuditLogs200Response list_org_audit_logs(version, org_id, cursor, from, to, size, sort_order, user_id, project_id, event, exclude_event)
Search Organization audit logs.

Search audit logs for an Organization. Supported event types:   - api.access   - org.app_bot.create   - org.app.create   - org.app.delete   - org.app.edit   - org.cloud_config.settings.edit   - org.collection.create   - org.collection.delete   - org.collection.edit   - org.create   - org.delete   - org.edit   - org.ignore_policy.edit   - org.integration.create   - org.integration.delete   - org.integration.edit   - org.integration.settings.edit   - org.language_settings.edit   - org.notification_settings.edit   - org.org_source.create   - org.org_source.delete   - org.org_source.edit   - org.policy.edit   - org.project_filter.create   - org.project_filter.delete   - org.project.add   - org.project.attributes.edit   - org.project.delete   - org.project.edit   - org.project.fix_pr.auto_open   - org.project.fix_pr.manual_open   - org.project.ignore.create   - org.project.ignore.delete   - org.project.ignore.edit   - org.project.monitor   - org.project.pr_check.edit   - org.project.remove   - org.project.settings.delete   - org.project.settings.edit   - org.project.stop_monitor   - org.project.tag.add   - org.project.tag.remove   - org.project.test   - org.request_access_settings.edit   - org.sast_settings.edit   - org.service_account.create   - org.service_account.delete   - org.service_account.edit   - org.settings.feature_flag.edit   - org.target.create   - org.target.delete   - org.user.add   - org.user.invite   - org.user.invite.accept   - org.user.invite.revoke   - org.user.invite_link.accept   - org.user.invite_link.create   - org.user.invite_link.revoke   - org.user.leave   - org.user.provision.accept   - org.user.provision.create   - org.user.provision.delete   - org.user.remove   - org.user.role.create   - org.user.role.delete   - org.user.role.details.edit   - org.user.role.edit   - org.user.role.permissions.edit   - org.webhook.add   - org.webhook.delete   - user.org.notification_settings.edit 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**version** | **String** | The requested version of the endpoint to process the request | [required] |
**org_id** | **uuid::Uuid** | The ID of the organization. | [required] |
**cursor** | Option<**String**> | The ID for the next page of results. |  |
**from** | Option<**String**> | The start date (inclusive) of the audit logs search. If this is not specified, the start of yesterday is used. Example: 2023-07-27  |  |
**to** | Option<**String**> | The end date (inclusive) of the audit logs search. Example: 2023-07-27  |  |
**size** | Option<**i32**> | Number of results to return per page. |  |
**sort_order** | Option<**String**> | Order in which results are returned. |  |[default to DESC]
**user_id** | Option<**uuid::Uuid**> | Filter logs by user ID. |  |
**project_id** | Option<**uuid::Uuid**> | Filter logs by project ID. |  |
**event** | Option<**String**> | Filter logs by event type, cannot be used in conjunction with exclude_event parameter. |  |
**exclude_event** | Option<**String**> | Exclude event type from results, cannot be used in conjunctions with event parameter. |  |

### Return type

[**crate::models::ListGroupAuditLogs200Response**](listGroupAuditLogs_200_response.md)

### Authorization

[APIToken](../README.md#APIToken), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/vnd.api+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

