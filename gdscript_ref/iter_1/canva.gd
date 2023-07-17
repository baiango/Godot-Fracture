@tool
extends Sprite2D
var update_count = 0

# (Left corner -> Scene -> Reload) Saved Scene to reload the script.
func _process(delta):
	var time_sec = 3.0 # It depends on the viewport
	# Way stable than "await get_tree().create_timer(5.0).timeout" But it's inaccurate.
	if Engine.get_frames_drawn() % int(60 * time_sec): return
	print("Updated! Count: ", update_count)
	update_count += 1
	# Generating image
	var img = Image.create(512, 512, false, Image.FORMAT_RGB8)

	for __ in [0]:
		var rng = RandomNumberGenerator.new()
		rng.set_seed(1023)
		for x in 512:
			for y in 512:
				img.set_pixel(x, y, Color8(50, int(x * 0.3), rng.randi_range(0, 255)))
#	rng # Godot can't read "rng" if you warp it inside the "for __ in [0]:".

	# Setting up Sprite2D
	set_texture(ImageTexture.create_from_image(img))
	set_centered(false) # It will make sure no one mess with it.
	set_texture_filter(CanvasItem.TEXTURE_FILTER_NEAREST)


func _on_tree_exiting():
	# To clean up for uploading.
	# Switch to another scene to run it.
	# You still need to make sure you're saving it without the image.
	# We don't need to upload the generated image. it takes a ton of space.
	set_texture(null)
