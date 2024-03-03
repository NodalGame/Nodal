extends Node

# Assume you have a list of UUIDs for levels
var level_uuids = ["uuid1", "uuid2", "uuid3"]  # Replace these with actual UUIDs

func _ready():
	# Assuming you have a VBoxContainer or similar to arrange buttons
	var container = $VBoxContainer  # Adjust the path according to your scene structure

	for uuid in level_uuids:
		var button = preload("res://puzzlescreen.tscn").instance()  # Load the Button scene
		button.text = "Level " + uuid  # Set button text (customize as needed)
		button.connect("pressed", self, "_on_Button_pressed", [uuid])  # Connect the button's pressed signal
		container.add_child(button)  # Add the button to the container

func _on_button_pressed(uuid):
	print("Load level with UUID: " + uuid)  # Replace with actual level loading logic
	# For example, you might change the scene or load a level based on the UUID
