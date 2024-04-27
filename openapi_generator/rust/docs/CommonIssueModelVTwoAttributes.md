# CommonIssueModelVTwoAttributes

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**coordinates** | Option<[**Vec<crate::models::CoordinateVTwo>**](CoordinateVTwo.md)> |  | [optional]
**created_at** | Option<**String**> |  | [optional]
**description** | Option<**String**> | A description of the issue in Markdown format | [optional]
**effective_severity_level** | Option<**String**> | The type from enumeration of the issue’s severity level. This is usually set from the issue’s producer, but can be overridden by policies. | [optional]
**problems** | Option<[**Vec<crate::models::CommonIssueModelAttributesProblemsInner>**](CommonIssueModel_attributes_problems_inner.md)> |  | [optional]
**severities** | Option<[**Vec<crate::models::Severity>**](Severity.md)> | The severity level of the vulnerability: ‘low’, ‘medium’, ‘high’ or ‘critical’. | [optional]
**slots** | Option<[**crate::models::Slots**](Slots.md)> |  | [optional]
**title** | Option<**String**> | A human-readable title for this issue. | [optional]
**r#type** | Option<**String**> | The issue type | [optional]
**updated_at** | Option<**String**> | When the vulnerability information was last modified. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


