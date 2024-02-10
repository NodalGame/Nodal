extends Node2D

var load_puzzle = preload("res://logic/load_puzzle.gd")
var lines = preload("res://logic/objects/line.gd")
var custom_nodes

# This will get a specific puzzle ID to load on each instantiation 
func _init():
	var line = lines.new()
	add_child(line)


# Called when the node enters the scene tree for the first time.
func _ready():
	print("readying puzzlescreen")
	var puzzle = load_puzzle.new()
	
	add_child(puzzle)
	custom_nodes = puzzle._load_puzzle()


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta):
	pass
