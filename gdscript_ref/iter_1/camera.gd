extends Camera3D

@onready var h := $"../.." as Node3D
@onready var v := $".." as SpringArm3D


func _ready() -> void:
	Input.set_mouse_mode(Input.MOUSE_MODE_CAPTURED)


func _input(event: InputEvent) -> void:
	var mouse_sensitivity := 0.1
	if event as InputEventMouseMotion:
		var camera_input := (event as InputEventMouseMotion).get_relative() as Vector2
		h.rotate_y(deg_to_rad(-camera_input.x * mouse_sensitivity))
		v.rotate_x(deg_to_rad(-camera_input.y * mouse_sensitivity))
		v.set_rotation(Vector3(clamp(v.rotation.x, -PI/2, PI/2), v.get_rotation().y, v.get_rotation().z))

	if Input.is_action_just_pressed("ui_cancel"):
		Input.mouse_mode = Input.MOUSE_MODE_VISIBLE if Input.mouse_mode else Input.MOUSE_MODE_CAPTURED


func _physics_process(delta: float) -> void:
	# movement
	var input := Vector3(Input.get_axis("w","s"), Input.get_axis("q", "e"), Input.get_axis("a","d"))

	h.position += (
			h.transform.basis.z * input.x +
			h.transform.basis.y * input.y +
			h.transform.basis.x * input.z
	) * delta * 5
