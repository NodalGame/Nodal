extends TextureButton

signal check_answer()


# Called when the node enters the scene tree for the first time.
func _ready():
	pass # Replace with function body.


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta):
	pass


# Called when pressed
func _pressed():
	print("pressed check answer")
	emit_signal("check_answer")
