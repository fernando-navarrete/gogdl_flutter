import 'dart:typed_data';

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

  test('clearCache completes on a fresh, unauthenticated GogdlApi', () async {
    await GogdlApi().clearCache();
  });

  test('removeProducts with nothing to remove returns zeros, offline', () async {
    final removal = await GogdlApi().removeProducts(
      gameId: 1,
      path: '/nonexistent',
      buildName: 'build',
      keep: Int32List.fromList([1]),
      remove: Int32List(0),
    );
    expect(removal.filesRemoved, BigInt.zero);
    expect(removal.bytesFreed, BigInt.zero);
  });

  testWidgets('example app shows the login link', (tester) async {
    await tester.pumpWidget(const MyApp());
    expect(find.textContaining('auth.gog.com'), findsOneWidget);
    expect(find.byType(SelectableText), findsOneWidget);
  });
}
