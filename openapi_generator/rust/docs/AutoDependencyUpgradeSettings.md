# AutoDependencyUpgradeSettings

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**ignored_dependencies** | Option<**Vec<String>**> | Dependencies which should NOT be included in an automatic upgrade operation. | [optional]
**is_enabled** | Option<**bool**> | Automatically raise pull requests to update out-of-date dependencies. | [optional]
**is_inherited** | Option<**bool**> | Apply the auto dependency integration settings of the Organization to this project. | [optional]
**is_major_upgrade_enabled** | Option<**bool**> | Include major version in dependency upgrade recommendation. | [optional]
**limit** | Option<**f32**> | Limit of dependency upgrade PRs which can be opened simultaneously. When the limit is reached, no new upgrade PRs are created. If specified, must be between 1 and 10. | [optional]
**minimum_age** | Option<**f32**> | Minimum dependency maturity period in days. If specified, must be between 1 and 365. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


