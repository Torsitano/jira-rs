# UiModificationDetails

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | The ID of the UI modification. | [readonly]
**name** | **String** | The name of the UI modification. The maximum length is 255 characters. | [readonly]
**description** | Option<**String**> | The description of the UI modification. The maximum length is 255 characters. | [optional][readonly]
**param_self** | **String** | The URL of the UI modification. | [readonly]
**data** | Option<**String**> | The data of the UI modification. The maximum size of the data is 50000 characters. | [optional][readonly]
**contexts** | Option<[**Vec<crate::models::UiModificationContextDetails>**](UiModificationContextDetails.md)> | List of contexts of the UI modification. The maximum number of contexts is 1000. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


