# PageOfStatuses

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**start_at** | Option<**i64**> | The index of the first item returned on the page. | [optional]
**total** | Option<**i64**> | Number of items that satisfy the search. | [optional]
**is_last** | Option<**bool**> | Whether this is the last page. | [optional]
**max_results** | Option<**i32**> | The maximum number of items that could be returned. | [optional]
**values** | Option<[**Vec<crate::models::JiraStatus>**](JiraStatus.md)> | The list of items. | [optional]
**param_self** | Option<**String**> | The URL of this page. | [optional]
**next_page** | Option<**String**> | The URL of the next page of results, if any. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


