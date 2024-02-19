extends Node2D
class_name GameNode

# Class member variables for x and y coordinates
var x: float
var y: float
var sprite: Sprite2D
var sprite_position: Vector2
var connections: Array[GameNode]


# Constructor
func _init(init_x: float, init_y: float):
	x = init_x
	y = init_y
	# Optionally, set the position of the Node2D to the x/y coordinates
	position = Vector2(x, y)

	# Create a Sprite node and add it as a child of this node
	sprite = Sprite2D.new()
	add_child(sprite)
	# Optionally, set the texture of the sprite
	sprite.texture = preload("res://sprite_imgs/EmptyNode.png")

	# Additional setup for the sprite can go here
	# For example, you might want to center the sprite, adjust its scale, etc.
	sprite.centered = true
	sprite.scale = Vector2(0.5, 0.5)
	sprite.visible = true
	
	sprite_position = sprite.global_position

# Example function to demonstrate using the x and y coordinates
func print_coordinates():
	print("X: ", x, ", Y: ", y)

# Called when the node enters the scene tree for the first time.
func _ready():
	pass # Replace with function body.


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta):
	pass


# Boolean returning if mouse intercepts the texture of the node 
func intercepts(mouse_position):
	return mouse_position.distance_to(sprite_position) <= 50.0
	
	
func add_connection(node: GameNode):
	connections.append(node)
	
	
func remove_connection(node: GameNode):
	var idx = connections.find(node)
	if idx != -1:
		connections.remove_at(idx)


func get_connections() -> Array[GameNode]:
	return connections
