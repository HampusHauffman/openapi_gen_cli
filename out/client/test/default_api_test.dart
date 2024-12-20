import 'package:test/test.dart';
import 'package:openapi/openapi.dart';


/// tests for DefaultApi
void main() {
  final instance = Openapi().getDefaultApi();

  group(DefaultApi, () {
    // Get a pet
    //
    //Future<PetGet200Response> petGet() async
    test('test petGet', () async {
      // TODO
    });

  });
}
