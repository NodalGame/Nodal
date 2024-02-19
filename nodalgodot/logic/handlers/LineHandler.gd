extends Node2D
class_name LineHandler

var is_dragging = false
var start_node = null
var game_nodes: Array[GameNode]
var active_line: Line2D
var active_node: GameNode
var lines: Array[Line2D]


func _init(nodes):
	game_nodes = nodes


func _ready():
	pass

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

	elif event is InputEventMouseMotion and is_dragging and active_line:
		# Update the line to follow the mouse cursor
		active_line.points[1] = event.position

func start_drag(node):
	start_node = node
	is_dragging = true
	
	# init new line
	active_line = Line2D.new()
	add_child(active_line)
	active_line.width = 4
	active_line.default_color = Color(1, 0, 0)  # Red color
	
	active_line.points = [start_node.global_position, start_node.global_position]  # Start and end at the node
	active_line.z_index = 1
	
	# Replace active node
	active_node = start_node

func end_drag(node):
	is_dragging = false
	
	# give each connected node new connection
	if active_node:
		active_node.add_connection(node)
		node.add_connection(active_node)
	active_node = null
	
	# kill line if exists
	if active_line:
		active_line.points[1] = node.global_position  # End the line at the second node's position
		lines.append(active_line)
		active_line = null

func cancel_drag():
	is_dragging = false
	active_line.points = []
	active_line = null 

func get_node_at_mouse_position(mouse_position):
	for node in game_nodes:
		if node.intercepts(mouse_position):
			return node
	return null
