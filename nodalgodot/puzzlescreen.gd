extends Node2D

var LineHandler = preload("res://logic/handlers/LineHandler.gd")
var GameNode = preload("res://logic/objects/GameNode.gd")
var game_nodes: Array[GameNode]

# This will get a specific puzzle ID to load on each instantiation 
func _init():
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
	var check_answer_btn = get_node("CheckAnswerBtn")
	check_answer_btn.connect("check_answer", _on_check_answer)
	print("done readying")


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta):
	pass
	

# Called when check answer button pressed
func _on_check_answer():
	print("checking answer")
	# Verify all nodes connected via BFS (logic will change with multi-color)
	var visited = {}
	var queue: Array[GameNode] = []
	
	if game_nodes.is_empty():
		return
		
	queue.push_back(game_nodes[0])
	visited[game_nodes[0]] = true
	
	while queue.size() != 0:
		var curr_node = queue.pop_front()
		for neighbor in curr_node.get_connections():
			if not visited.has(neighbor):
				queue.push_back(neighbor)
				visited[neighbor] = true
				
	if len(visited) == len(game_nodes):
		print("solved")
	else:
		print("unsolved")
