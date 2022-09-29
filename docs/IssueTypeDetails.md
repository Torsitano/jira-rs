# IssueTypeDetails

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**param_self** | Option<**String**> | The URL of these issue type details. | [optional][readonly]
**id** | Option<**String**> | The ID of the issue type. | [optional][readonly]
**description** | Option<**String**> | The description of the issue type. | [optional][readonly]
**icon_url** | Option<**String**> | The URL of the issue type's avatar. | [optional][readonly]
**name** | Option<**String**> | The name of the issue type. | [optional][readonly]
**subtask** | Option<**bool**> | Whether this issue type is used to create subtasks. | [optional][readonly]
**avatar_id** | Option<**i64**> | The ID of the issue type's avatar. | [optional][readonly]
**entity_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Unique ID for next-gen projects. | [optional][readonly]
**hierarchy_level** | Option<**i32**> | Hierarchy level of the issue type. | [optional][readonly]
**scope** | Option<[**crate::models::IssueTypeDetailsScope**](IssueTypeDetails_scope.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


