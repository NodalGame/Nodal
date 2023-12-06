import 'package:flame/game.dart';
import 'package:flutter/material.dart';
import 'package:nodal/flame_game.dart';

void main() {
  WidgetsFlutterBinding.ensureInitialized();
  runApp(const MyGameApp());
}

class MyGameApp extends StatelessWidget {
  const MyGameApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Simple Game',
      home: Scaffold(
        body: GameWidget(game: MyGridGame()),
      ),
    );
  }
}
