# JiraStatus

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The ID of the status. | [optional]
**name** | Option<**String**> | The name of the status. | [optional]
**status_category** | Option<**String**> | The category of the status. | [optional]
**scope** | Option<[**crate::models::StatusScope**](StatusScope.md)> |  | [optional]
**description** | Option<**String**> | The description of the status. | [optional]
**usages** | Option<[**Vec<crate::models::ProjectIssueTypes>**](ProjectIssueTypes.md)> | Projects and issue types where the status is used. Only available if the `usages` expand is requested. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


