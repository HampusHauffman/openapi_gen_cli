# openapi.api.DefaultApi

## Load the API package
```dart
import 'package:openapi/api.dart';
```

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**petGet**](DefaultApi.md#petget) | **GET** /pet | Get a pet


# **petGet**
> PetGet200Response petGet()

Get a pet

### Example
```dart
import 'package:openapi/api.dart';

final api = Openapi().getDefaultApi();

try {
    final response = api.petGet();
    print(response);
} catch on DioException (e) {
    print('Exception when calling DefaultApi->petGet: $e\n');
}
```

### Parameters
This endpoint does not need any parameter.

### Return type

[**PetGet200Response**](PetGet200Response.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

