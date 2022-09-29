# Dashboard

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**description** | Option<**String**> |  | [optional]
**id** | Option<**String**> | The ID of the dashboard. | [optional][readonly]
**is_favourite** | Option<**bool**> | Whether the dashboard is selected as a favorite by the user. | [optional][readonly]
**name** | Option<**String**> | The name of the dashboard. | [optional][readonly]
**owner** | Option<[**crate::models::DashboardOwner**](Dashboard_owner.md)> |  | [optional]
**popularity** | Option<**i64**> | The number of users who have this dashboard as a favorite. | [optional][readonly]
**rank** | Option<**i32**> | The rank of this dashboard. | [optional][readonly]
**param_self** | Option<**String**> | The URL of these dashboard details. | [optional][readonly]
**share_permissions** | Option<[**Vec<crate::models::SharePermission>**](SharePermission.md)> | The details of any view share permissions for the dashboard. | [optional][readonly]
**edit_permissions** | Option<[**Vec<crate::models::SharePermission>**](SharePermission.md)> | The details of any edit share permissions for the dashboard. | [optional][readonly]
**automatic_refresh_ms** | Option<**i32**> | The automatic refresh interval for the dashboard in milliseconds. | [optional][readonly]
**view** | Option<**String**> | The URL of the dashboard. | [optional][readonly]
**is_writable** | Option<**bool**> | Whether the current user has permission to edit the dashboard. | [optional][readonly]
**system_dashboard** | Option<**bool**> | Whether the current dashboard is system dashboard. | [optional][readonly]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


