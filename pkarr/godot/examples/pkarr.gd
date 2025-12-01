extends Node

#
## Asumiendo que has instanciado la clase o la tienes en la escena
#@onready var pkarr = PkarrGodot.new()
#
#func _ready():
	#add_child(pkarr)
	#test_pkarr_features()
#
#func test_pkarr_features():
	## --- Configuración de Relays ---
	## 1. Relays en formato Array (como antes)
	#var relays_base = PackedStringArray([
		#"wss://relay.nostr.band", 
		#"wss://nos.lol"
	#])
	#
	## 2. NUEVO: Relays extra en un solo String separado por comas
	## Esto es lo que querías para facilitar la carga desde texto/input
	#var mis_relays_custom = "wss://relay.damus.io,wss://relay.snort.social,wss://relay.primal.net"
#
	## --- EJEMPLO 1: Publicar (prepare_packet) ---
	#print("--- Iniciando Publicación ---")
	#
	## Necesitamos una semilla de 32 bytes para la clave
	#var seed_str = "12345678901234567890123456789012" # 32 caracteres exactos para el ejemplo
	#var keypass = seed_str.to_utf8_buffer()
	#
	#var exito = pkarr.prepare_packet(
		#"mi-nombre-app",    # key (nombre del registro)
		#"valor-del-txt",    # value (contenido del registro TXT)
		#"relays",           # mode ("dht", "relays", o "both")
		#relays_base,        # relays (Array estándar de Godot)
		#mis_relays_custom,  # extra_relays (TU NUEVO PARAMETRO: String separado por comas)
		#keypass,            # keypass (PackedByteArray de 32 bytes)
		#300                 # ttl (Tiempo de vida en segundos)
	#)
	#
	#if exito:
		#print("Solicitud de publicación enviada correctamente.")
	#else:
		#print("Hubo un error en los parámetros de publicación.")
#
#
	## --- EJEMPLO 2: Resolver (resolve_key) ---
	#print("\n--- Iniciando Resolución ---")
	#
	## Una clave pública en formato zbase32 (ejemplo)
	#var public_key = "o4dksfbqk85ogzdb5osziw6befigbuxmuxkuxq8434q89uj56yad"
	#
	#var resultado = pkarr.resolve_key(
		#public_key,         # key (clave pública zbase32)
		#"relays",           # mode
		#relays_base,        # relays (Array estándar)
		#mis_relays_custom   # extra_relays (TU NUEVO PARAMETRO)
	#)
	#
	#if resultado != "":
		#print("Resolución exitosa! Datos recibidos:")
		#print(resultado)
	#else:
		#print("No se pudo resolver la clave o no se encontraron datos.")
