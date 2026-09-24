import 'package:flutter/material.dart';
import 'package:gogdl_flutter/gogdl_flutter.dart';

Future<void> main() async {
  await RustLib.init();
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  @override
  Widget build(BuildContext context) {
    final loginLink = GogdlApi().getLoginLink();
    return MaterialApp(
      home: Scaffold(
        appBar: AppBar(title: const Text('gogdl_flutter example')),
        body: Center(
          child: Padding(
            padding: const EdgeInsets.all(16),
            child: SelectableText('GOG login link:\n$loginLink'),
          ),
        ),
      ),
    );
  }
}
