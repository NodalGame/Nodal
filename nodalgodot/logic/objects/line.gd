extends Node2D

var is_dragging = false
var line = Line2D.new()
var start_node = null


func _ready():
	add_child(line)
	line.width = 4
	line.default_color = Color(1, 0, 0)  # Red color

func _input(event):
	if event is InputEventMouseButton:
		if event.button_index == MOUSE_BUTTON_LEFT and event.pressed:
			# Check if a draggable node was clicked
			var clicked_node = get_node_at_mouse_position(event.position)
			if clicked_node:
				start_drag(clicked_node)
		elif event.button_index == MOUSE_BUTTON_LEFT and not event.pressed:
			if is_dragging:
				# Check if released over a draggable node
				var released_node = get_node_at_mouse_position(event.position)
				if released_node and released_node != start_node:
					end_drag(released_node)
				else:
					cancel_drag()

	elif event is InputEventMouseMotion and is_dragging:
		# Update the line to follow the mouse cursor
		line.points[1] = event.position

func start_drag(node):
	print("start drag")
	start_node = node
	is_dragging = true
	line.points = [start_node.global_position, start_node.global_position]  # Start and end at the node
	line.z_index = 1

func end_drag(node):
	print("end drag")
	is_dragging = false
	line.points[1] = node.global_position  # End the line at the second node's position

func cancel_drag():
	is_dragging = false
	line.points = []  # Clear the line points

func get_node_at_mouse_position(mouse_position):
	for node in get_parent().custom_nodes:
		if node.intercepts(mouse_position):
			print("intercepts")
			return node
	return null
