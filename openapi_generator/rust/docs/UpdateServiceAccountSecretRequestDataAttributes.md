# UpdateServiceAccountSecretRequestDataAttributes

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**mode** | **String** | Operation to perform:   * `replace` - Replace existing secrets with a new generated secret.   * `create` - Add a new secret, preserving existing secrets. A maximum of to two secrets can exist at a time.   * `delete` - Remove an existing secret by value. At least one secret must remain per service account.  | 
**secret** | Option<**String**> | Secret to delete when using `delete` mode | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


