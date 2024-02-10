extends Node2D

var load_puzzle = preload("res://logic/load_puzzle.gd")

# This will get a specific puzzle ID to load on each instantiation 
func _init():
	pass


# Called when the node enters the scene tree for the first time.
func _ready():
	print("readying puzzlescreen")
	load_puzzle.new()._load_puzzle()


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta):
	pass
