extends Node2D

var LineHandler = preload("res://logic/objects/LineHandler.gd")
var GameNode = preload("res://logic/objects/GameNode.gd")
var game_nodes: Array[GameNode]

# This will get a specific puzzle ID to load on each instantiation 
func _init():
	print("readying puzzlescreen")
	
	# Naively loads a 2x2 puzzle
	var node_0 = GameNode.new(100, 100)
	var node_1 = GameNode.new(100, 200)
	var node_2 = GameNode.new(200, 100)
	var node_3 = GameNode.new(200, 200)
	
	add_child(node_0)
	add_child(node_1)
	add_child(node_2)
	add_child(node_3)
	
	game_nodes = [node_0, node_1, node_2, node_3]
	
	var lineHandler = LineHandler.new(game_nodes)
	add_child(lineHandler)


# Called when the node enters the scene tree for the first time.
func _ready():
	pass


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta):
	pass
	
