extends Node2D

# Class member variables for x and y coordinates
var x: float
var y: float

# Constructor
func _init(init_x: float, init_y: float):
	x = init_x
	y = init_y
	# Optionally, set the position of the Node2D to the x/y coordinates
	position = Vector2(x, y)

	# Create a Sprite node and add it as a child of this node
	var sprite = Sprite2D.new()
	add_child(sprite)
	# Optionally, set the texture of the sprite
	sprite.texture = preload("res://sprite_imgs/EmptyNode.png")

	# Additional setup for the sprite can go here
	# For example, you might want to center the sprite, adjust its scale, etc.
	sprite.centered = true
	sprite.scale = Vector2(0.5, 0.5)
	sprite.visible = true
	
	print_coordinates()

# Example function to demonstrate using the x and y coordinates
func print_coordinates():
	print("X: ", x, ", Y: ", y)

# Called when the node enters the scene tree for the first time.
func _ready():
	pass # Replace with function body.


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta):
	pass
