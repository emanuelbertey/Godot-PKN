# Análisis del Código Base en Rust (Extensión Pkarr Godot)

Este documento analiza la implementación actual de la extensión de Godot basada en Rust que se encuentra en `pkarr/godot`.

## 1. Arquitectura Actual

La extensión expone dos clases principales a Godot: `Keyl` (probablemente pensada como una herramienta de claves) y `Peerinfo` (que maneja las interacciones con Pkarr).

### Diagrama de Clases

```mermaid
classDiagram
    class Keyl {
        +speed: f64
        +angular_speed: f64
        +init(base: Node)
        +physics_process(delta: f64)
        +increase_speed(amount: f64)
        +public_key(key: PackedByteArray) GString
        +generate_key() PackedByteArray
        +to_nsec(secret: PackedByteArray) GString
        +to_npub(secret: PackedByteArray) GString
        +hex_npub(npub: GString) GString
        +hex_nsec(nsec: GString) GString
        +npub(npub: GString) GString
        +nsec(nsec: GString) GString
    }

    class Peerinfo {
        +init(base: Node)
        +generate_random_8byte_number() PackedByteArray
        +generate_numbers(count: i64) PackedByteArray
        +run_ipfs() PackedByteArray
        +get_secret_bytes() PackedByteArray
        +key_rand() PackedByteArray
        +prepare_packet(key, value, mode, relays, keypass) bool
        +resolve_key(key, mode, relays) bool
        +search(key, value, keypass) bool
        +public_key(key) GString
        +info_ips() bool
        +_key(keys) bool
        +get_ips() GString
        +get_ipfs() GString
    }

    class GlobalState {
        <<static>>
        +VEC_DATA: Mutex<Vec<u8>>
        +DOWNLOADED_DATA: Mutex<Vec<u8>>
        +GLOBAL_ARRAY: Mutex<Vec<u8>>
        +PEER_IPS: Mutex<HashMap<String, Vec<String>>>
        +GLOBAL_IPS: Mutex<Vec<String>>
        +GLOBAL_HTTP: Mutex<Vec<String>>
        +IP_IPFS: Mutex<Vec<String>>
        +ID_IPFS: Mutex<Vec<String>>
    }

    Peerinfo ..> GlobalState : usa
```

## 2. Análisis de Flujo (Problemas Críticos)

El problema más crítico es el uso de `futures::executor::block_on` dentro del hilo principal de Godot. Esto hace que el juego se congele mientras espera operaciones de red.

### Flujo Bloqueante (Actual)

```mermaid
sequenceDiagram
    participant Godot
    participant Peerinfo
    participant PkarrClient
    participant Network

    Godot->>Peerinfo: resolve_key(key, ...)
    Note right of Peerinfo: Hilo Principal (Main Thread)
    Peerinfo->>PkarrClient: client.resolve(key)
    activate Peerinfo
    Note right of Peerinfo: BLOQUEA el Hilo Principal
    PkarrClient->>Network: Petición DHT
    Network-->>PkarrClient: Respuesta
    PkarrClient-->>Peerinfo: Resultado
    deactivate Peerinfo
    Peerinfo->>Godot: emit_signal("resolv", data)
    Peerinfo-->>Godot: return true
```

## 3. Problemas Identificados

1.  **Bloqueo del Hilo Principal**:
    *   **Problema**: Funciones como `resolve_key` y `prepare_packet` usan `block_on`. Esto congela toda la aplicación Godot hasta que la petición de red se completa o expira (timeout).
    *   **Impacto**: Mala experiencia de usuario, la aplicación deja de responder.

2.  **Estado Global Mutable**:
    *   **Problema**: `state.rs` usa variables globales `Mutex` con `lazy_static` (`PEER_IPS`, `GLOBAL_IPS`, etc.).
    *   **Impacto**: Esto hace que el código sea difícil de testear, propenso a condiciones de carrera (race conditions) si existen múltiples nodos `Peerinfo`, y va en contra de la estructura de nodos orientada a objetos de Godot. El estado debería estar encapsulado dentro de la estructura `Peerinfo`.

3.  **Calidad y Limpieza del Código**:
    *   **Código Muerto**: Hay muchos bloques comentados y funciones marcadas como "SIN USO".
    *   **Convenciones de Nombres**: Mezcla de comentarios/logs en inglés y español. El nombre de la clase `Keyl` es poco claro (debería ser `KeyTool` o `NostrKeys`).
    *   **Valores Hardcodeados**: Semillas (seeds) y claves de prueba están escritas directamente en el código en algunas funciones.

4.  **Manejo de Errores**:
    *   **Problema**: Los errores se imprimen en la consola (`godot_error!`), pero las funciones a menudo solo devuelven `false` o cadenas vacías.
    *   **Impacto**: El código GDScript que llama a estas funciones no puede reaccionar fácilmente a errores específicos (por ejemplo, "error de red" vs "clave inválida").

## 4. Mejoras Propuestas

### A. Ejecución Asíncrona (No Bloqueante)
Mover las operaciones de red a un hilo en segundo plano o usar un runtime asíncrono compatible con Godot.

```mermaid
sequenceDiagram
    participant Godot
    participant Peerinfo
    participant BackgroundTask
    participant Network

    Godot->>Peerinfo: resolve_key(key, ...)
    Peerinfo->>BackgroundTask: Inicia tarea asíncrona
    Peerinfo-->>Godot: return (inmediato)
    
    par Tarea en Segundo Plano
        BackgroundTask->>Network: Petición DHT
        Network-->>BackgroundTask: Respuesta
        BackgroundTask->>Godot: call_deferred_thread_group("emit_signal", "resolv", data)
    end
```

### B. Refactorización de `Peerinfo`
*   Eliminar el estado global. Almacenar los datos en `self` (los campos de la estructura).
*   Usar el runtime de `tokio` (si es posible) o `std::thread` para tareas bloqueantes.

### C. Refactorización de `Keyl`
*   Renombrar a `NostrKeyTool`.
*   Eliminar `physics_process` y otra lógica de nodo no relacionada.
*   Hacer que sea un `RefCounted` o una clase de utilidad puramente estática si no necesita estar en el árbol de escena.

### D. Estandarización
*   Traducir todos los logs y comentarios a un solo idioma (preferiblemente inglés para código fuente, o español si así se requiere, pero consistente).
*   Limpiar el código no utilizado.
