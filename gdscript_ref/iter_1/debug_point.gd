@tool
extends MeshInstance3D


@export_range(0.0, 2.0, 0.01) var points_scale := 0.1

func _ready():
	set_mesh($"../MeshInstance3D".get_mesh())
	var points := mesh.get_faces()
	set_mesh(null)
#	print_debug(points)

	for __ in 1: # Deduplicate position
		var swap := []
		for p in points:
			if p not in swap:
				swap.append(p)
		points = swap

#	print_debug(points)
	# Load point meshes
	var pt_mesh := load("res://res/point.tscn") as PackedScene
	for p in points:
		add_child(pt_mesh.instantiate())
		get_child(-1).set_position(p)


var scale_changed := 0.0
func _process(delta: float):
	if points_scale == scale_changed: return
	scale_changed = points_scale

	# Set children scale
	for c in get_children():
		c.set_scale(Vector3.ONE * points_scale)
