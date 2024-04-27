# PullRequestsSettings

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**fail_only_for_issues_with_fix** | Option<**bool**> | Only fail when the issues found have a fix available. | [optional]
**policy** | Option<**String**> | Fail if the project has any issues (\"all\"), or fail if a PR is introducing a new dependency with issues (\"only_new\"). If this value is unset, the setting is inherited from the org default. | [optional]
**severity_threshold** | Option<**String**> | Only fail for issues greater than or equal to the specified severity. If this value is unset, the setting is inherited from the org default. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


