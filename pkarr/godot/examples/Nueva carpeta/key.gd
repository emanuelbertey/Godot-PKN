extends Control
var global_key
var npub
var nsec
var hex_npub
var hex_nsec
var peer
func _ready() -> void:
	peer = Keyl.new()

	##prints(peer.to_nsec(peer.generate_key()))
	#var key = peer.generate_key()
	#prints(peer.to_nsec(key))
	#prints(peer.to_npub(key))#npub1y6l9mfpe3n6v5ws232yyvfhdpjy6hzc9qhvf7afe44rk70qejges4j34a6
	#peer.hex_npub("npub10elfcs4fr0l0r8af98jlmgdh9c8tcxjvz9qkw038js35mp4dma8qzvjptg")
	#peer.hex_nsec("nsec1vl029mgpspedva04g90vltkh6fvh240zqtv9k0t9af8935ke9laqsnlfe5")
	#prints(peer.npub("7e7e9c42a91bfef19fa929e5fda1b72e0ebc1a4c1141673e2794234d86addf4e"))
	#prints(peer.nsec("67dea2ed018072d675f5415ecfaed7d2597555e202d85b3d65ea4e58d2d92ffa"))


func _on_create_key_pressed() -> void:
	createkey()
	p_npub()
	p_nsec()
	pass # Replace with function body.

func createkey():
	global_key = peer.generate_key()


func _on_npub_hex_pressed() -> void:
	if global_key == null:
		createkey()
	p_npub()



func _on_nsec_hex_pressed() -> void:
	if global_key == null:
		createkey()
	p_nsec()





func p_npub():
	npub = peer.to_npub(global_key)
	$GridContainer2/npub.text = npub




func p_nsec():
	nsec = peer.to_nsec(global_key)
	$GridContainer2/nsec.text = nsec




func _on_hex_npub_pressed() -> void:
	p_hsec()
	pass # Replace with function body.

func p_hsec():
	prints("nsec")
	if global_key == null:
		createkey()
	if nsec == null:
		p_nsec()
	hex_nsec = peer.hex_nsec(nsec)
	$GridContainer2/hex_sec.text = hex_nsec





func phpub():
	if global_key == null:
		createkey()
	if npub == null:
		p_npub()
	hex_npub = peer.hex_npub(npub)
	$GridContainer2/hex_pub.text = hex_npub




func _on_hex_nsec_pressed() -> void:
	phpub()
	
	pass # Replace with function body.















func _on_hex_to_nsec_pressed() -> void:
	if $enter.text == "":
		return
	$out.text = peer.nsec($enter.text)
	
	pass # Replace with function body.


func _on_hex_to_npub_pressed() -> void:
	if $enter.text == "":
		return
	$out.text = peer.npub($enter.text)
	pass # Replace with function body.


func _on_npub_to_hex_pressed() -> void:
	if $enter.text == "":
		return
	$out.text = peer.hex_npub($enter.text)
	pass # Replace with function body.


func _on_nsec_to_hex_pressed() -> void:
	if $enter.text == "":
		return
	$out.text = peer.hex_nsec($enter.text)
	pass # Replace with function body.


func _on_quit_pressed() -> void:
	self.queue_free()
	pass # Replace with function body.
