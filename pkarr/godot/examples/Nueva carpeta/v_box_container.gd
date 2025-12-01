extends Control


@onready var window := $Window
@onready var label := $Window/Label
@onready var btn_accept := $Window/GridContainer/Button
@onready var btn_cancel := $Window/GridContainer/Button2
@onready var fond := $Window
func _ready():
	label.text = "¿Querés continuar con la acción?"
	btn_accept.text = "Aceptar"
	btn_cancel.text = "Cancelar"

	btn_accept.pressed.connect(_on_accept_pressed)
	btn_cancel.pressed.connect(_on_cancel_pressed)
	self.mouse_behavior_recursive = 1
	#window.popup_centered()

func _on_accept_pressed():
	print("Acción aceptada")
	get_tree().quit() 
	window.hide()

func _on_cancel_pressed():
	#self.MOUSE_FILTER_IGNORE
	#self.MOUSE_FILTER_PASS
	self.mouse_behavior_recursive = 1
	print("Acción cancelada")
	
	window.hide()
	
	
	
	
func _notification(what):
	if what == NOTIFICATION_WM_CLOSE_REQUEST:
		label.text = "¿quieres terminar?"
		#self.MOUSE_FILTER_STOP
		self.mouse_behavior_recursive = 0
		window.popup_centered()
		
	if what == NOTIFICATION_WM_GO_BACK_REQUEST:
		label.text = "¿quieres terminar?"
		self.mouse_behavior_recursive = 0
		#self.MOUSE_FILTER_STOP
		window.popup_centered()
