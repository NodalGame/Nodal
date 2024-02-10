extends Node2D


# Preload the custom node 
var CustomNode2D = preload("res://logic/objects/node.gd")

func _ready():
	pass

func _load_puzzle():
	print("loading puzzle")
	# Naively loads a 2x2 puzzle
	var node_0 = CustomNode2D.new(100, 100)
	var node_1 = CustomNode2D.new(100, 200)
	var node_2 = CustomNode2D.new(200, 100)
	var node_3 = CustomNode2D.new(200, 200)
	
	add_child(node_0)
	add_child(node_1)
	add_child(node_2)
	add_child(node_3)
