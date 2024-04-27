# IssueAttributes

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**classes** | Option<[**Vec<crate::models::Class>**](Class.md)> | A list of details for weakness data, policy, etc that are the class of this issue's source. | [optional]
**coordinates** | Option<[**Vec<crate::models::Coordinate>**](Coordinate.md)> | Where the issue originated, specific to issue type. Details on what code, package, etc introduced the issue. An issue may be caused by more than one coordinate.  | [optional]
**created_at** | **String** | The creation time of this issue. | 
**description** | Option<**String**> | A markdown-formatted optional description of this issue. Links are not permitted. | [optional]
**effective_severity_level** | **String** | The computed effective severity of this issue. This is either the highest level from all included severities, or an overridden value set via group level policy.  | 
**ignored** | **bool** | A flag indicating if the issue is being ignored. Derived from the issue's ignore, which provides more details. | 
**key** | **String** | An opaque key used for uniquely identifying this issue across test runs, within a project. | 
**problems** | Option<[**Vec<crate::models::Problem>**](Problem.md)> | A list of details for vulnerability data, policy, etc that are the source of this issue. | [optional]
**resolution** | Option<[**crate::models::Resolution**](Resolution.md)> |  | [optional]
**risk** | Option<[**crate::models::Risk**](Risk.md)> |  | [optional]
**status** | **String** | The issue's status. Derived from the issue's resolution, which provides more details. | 
**title** | **String** | A human-readable title for this issue. | 
**tool** | Option<**String**> | An opaque identifier for corelating across test runs. | [optional]
**r#type** | [**crate::models::TypeDef**](TypeDef.md) |  | 
**updated_at** | **String** | The time when this issue was last modified. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


