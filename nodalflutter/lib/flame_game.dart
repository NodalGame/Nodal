import 'package:flame/events.dart';
import 'package:flame/extensions.dart';
import 'package:flame/game.dart';
import 'package:flutter/material.dart';

import 'package:collection/collection.dart';

class MyGridGame extends FlameGame with DragCallbacks {
  final List<Circle> circles = [];
  final List<Line> lines = [];
  Line? currentLine;
  Vector2? currentTouchPoint;

  @override
  Future<void> onLoad() async {
    super.onLoad();
    _createGrid();
  }

  void _createGrid() {
    var gridSize = 3;
    var circleRadius = 30.0; // Adjust as needed
    var spacing = 100.0; // Adjust spacing as needed

    for (var i = 0; i < gridSize; i++) {
      for (var j = 0; j < gridSize; j++) {
        var x = spacing * (j + 1);
        var y = spacing * (i + 1);
        circles.add(Circle(Offset(x, y), circleRadius));
      }
    }
  }

  @override
  void render(Canvas canvas) {
    super.render(canvas);
    circles.forEach((circle) => circle.render(canvas));
    lines.forEach((line) => line.render(canvas));
    currentLine?.render(canvas);
  }

  @override
  void onDragStart(DragStartEvent event) {
    super.onDragStart(event);

    final touchPoint = event.canvasPosition.toOffset();
    final circle = _circleAtPoint(touchPoint);
    if (circle != null) {
      currentLine = Line(circle.center, circle.center);
      currentTouchPoint = event.canvasPosition;
    }
  }

  @override
  void onDragUpdate(DragUpdateEvent event) {
    super.onDragUpdate(event);

    if (currentLine == null) return;
    currentLine = Line(currentLine!.start, event.canvasStartPosition.toOffset());
    currentTouchPoint = event.canvasStartPosition;
  }

  @override
  void onDragEnd(DragEndEvent event) {
    super.onDragEnd(event);

    if (currentLine == null) return;

    final touchPoint = currentTouchPoint?.toOffset();
    final circle = _circleAtPoint(touchPoint!);
    if (circle != null) {
      lines.add(Line(currentLine!.start, circle.center));
    }
    currentLine = null;
  }

  Circle? _circleAtPoint(Offset point) {
    return circles.firstWhereOrNull((circle) => circle.contains(point));
  }
}


class Circle {
  final Offset center;
  final double radius;

  Circle(this.center, this.radius);

  void render(Canvas canvas) {
    final paint = Paint()..color = Colors.white;
    canvas.drawCircle(center, radius, paint);
  }

  bool contains(Offset point) {
    return (center - point).distance <= radius;
  }
}

class Line {
  Offset start;
  Offset end;

  Line(this.start, this.end);

  void render(Canvas canvas) {
    final paint = Paint()
      ..color = Colors.blue
      ..strokeWidth = 2.0;
    canvas.drawLine(start, end, paint);
  }
}