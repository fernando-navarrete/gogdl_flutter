import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:gogdl_flutter/gogdl_flutter.dart';
import 'package:gogdl_flutter_example/main.dart';
import 'package:integration_test/integration_test.dart';

void main() {
  IntegrationTestWidgetsFlutterBinding.ensureInitialized();
  setUpAll(RustLib.init);

  test('GogdlApi constructs', () {
    expect(GogdlApi.new, returnsNormally);
  });

  test('getLoginLink returns a URL on a GOG host', () {
    final uri = Uri.parse(GogdlApi().getLoginLink());
    expect(uri.scheme, 'https');
    expect(uri.host == 'gog.com' || uri.host.endsWith('.gog.com'), isTrue);
    expect(uri.queryParameters, contains('client_id'));
  });

  testWidgets('example app shows the login link', (tester) async {
    await tester.pumpWidget(const MyApp());
    expect(find.textContaining('auth.gog.com'), findsOneWidget);
    expect(find.byType(SelectableText), findsOneWidget);
  });
}
