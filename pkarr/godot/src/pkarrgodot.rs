use godot::prelude::*;
use godot::classes::Engine;
use godot::classes::{Node, RandomNumberGenerator, FileAccess};
use godot::classes::file_access::ModeFlags;
use godot::builtin::{PackedByteArray, StringName, GString};
use once_cell::sync::Lazy;

use std::collections::HashMap;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::fs::OpenOptions;
use std::io::Write;

use std::time::Instant;
use tracing_subscriber;

use pkarr::{Client, PublicKey, SignedPacket, Keypair};
use crate::state::{GLOBAL_IPS, PEER_IPS, DOWNLOADED_DATA, GLOBAL_ARRAY,GLOBAL_HTTP,IP_IPFS , ID_IPFS };




use simple_dns::Name;
use simple_dns::rdata::TXT;

use std::convert::TryInto;




#[derive(GodotClass)]
#[class(base=Node)]
pub struct PkarrGodot {
    base: Base<Node>,
}

#[godot_api]
impl INode for PkarrGodot {
    fn init(base: Base<Node>) -> Self {
         godot_print!("PkarrGodot");
        Self { 
            base,
        }
    }
    
    fn process(&mut self, _delta: f64) {
 }
 
}



#[godot_api]
impl PkarrGodot {

//probando las malditas señales
 #[godot_api(signals)]
    #[signal]
    fn speed_increased();
    #[signal]
    fn ips_actualizadas(data: GString);
    #[signal]
    fn http_actualizado(data: GString);
    #[signal]
    fn string_format(data: GString);
    #[signal]
    fn ips_ipfs(data: GString);
    #[signal]
    fn IDs_ipfs(data: GString);
    #[signal]
    fn resolv(data: GString);
    #[signal]
    fn published(success: bool, message: GString);


//SIN USO 1 
    #[func]
    pub fn generate_random_8byte_number(&self) -> PackedByteArray {
        let mut number = PackedByteArray::new();
        let mut rng = RandomNumberGenerator::new_gd();
        let my_seed = StringName::from("tsmdomtext").hash_u32() as u64;
        rng.set_seed(my_seed);
        for _ in 0..8 {
            number.push(rng.randi_range(0, 255) as u8);
        }
        number
    }
//
// Helper para logging desde threads
    #[func]
    fn log_to_file(&self, msg: GString, path: GString) {
        let path_string = path.to_string();
        if let Ok(mut file) = OpenOptions::new()
            .create(true)
            .append(true)
            .open(path_string)
        {
            let _ = writeln!(file, "[{}] {}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs(), msg);
        }
    }


// NUMBER ALEATORIO 
    #[func]
    fn generate_numbers(&self, count: i64) -> PackedByteArray {
    let mut numbers = PackedByteArray::new();
    let mut rng = RandomNumberGenerator::new_gd();
    let my_seed = StringName::from("tsmdomtext").hash_u32() as u64;
    rng.set_seed(my_seed);

    let count = count.clamp(1, 1024) as usize;  //1024 números

    for _ in 0..count {
        numbers.push(rng.randi_range(0, 255) as u8);
    }

    numbers
    }
//


//SIN USO 
    #[func]
    fn obtener_tamano_archivo(&self, path: GString) -> u64 {
        // Intenta abrir el archivo en modo lectura (nota el 'mut' aquí)
        let mut file = match FileAccess::open(&path, ModeFlags::READ) {
            Some(f) => f,
            None => {
                godot_error!("No se pudo abrir el archivo: {}", path);
                return 0;
            }
        };

        // Obtiene el tamaño del archivo
        let length = file.get_length();
        file.close();
        length
    }

/*no se usa  */
   #[func]
    pub fn run_ipfs(&self) -> PackedByteArray {
        godot_print!("run ipfs se ejecuta ");
       // kad::run_libp2p_kad_flow();

        let mut number = PackedByteArray::new();
        let mut rng = RandomNumberGenerator::new_gd();
        let my_seed = StringName::from("tsmdomtext").hash_u32() as u64;
        rng.set_seed(my_seed);
        for _ in 0..8 {
            number.push(rng.randi_range(0, 255) as u8);
        }
        number
}


/*prueba  */
    #[func]
    pub fn get_secret_bytes(&self) -> PackedByteArray {
        let secret_bytes: [u8; 32] = [
            199, 133, 251, 69, 66, 206, 61, 213, 151, 163, 166, 14, 142, 46, 94, 231,
            66, 126, 8, 67, 114, 56, 186, 37, 12, 18, 111, 207, 0, 223, 229, 145,
        ];

        let mut packed = PackedByteArray::new();
        for byte in secret_bytes {
            packed.push(byte);
        }

        packed
    }






   #[func]
    pub fn key_rand(&self) -> PackedByteArray {

        let keypair = Keypair::random();
        let secret_bytes: [u8; 32] = keypair.secret_key();

        let mut packed = PackedByteArray::new();
        for byte in secret_bytes {
            packed.push(byte);

        }

        packed
    }





#[func]
pub fn prepare_packet(&self, key: GString, value: GString, mode: GString, relays: PackedStringArray, keypass: PackedByteArray, ttl: u32) -> bool {
    let bytes = keypass.to_vec();
    if bytes.len() != 32 {
        godot_error!("La clave debe tener exactamente 32 bytes, pero tiene {}", bytes.len());
        return false;
    }

    let mut secret = [0u8; 32];
    secret.copy_from_slice(&bytes);
    let keypair = Keypair::from_secret_key(&secret);

  
    let name_string = key.to_string();
    let value_string = value.to_string();

    let converted: Name = match name_string.as_str().try_into() {
        Ok(n) => n,
        Err(e) => {
            godot_error!("❌ Nombre inválido: {:?}", e);
            return false;
        }
    };

let txt_converted: TXT = match value_string.as_str().try_into() {
    Ok(t) => t,
    Err(e) => {
        godot_error!("❌ Valor TXT inválido: {:?}", e);
        return false;
    }
};
    





    enum Mode {
            Dht,
            Relays,
            Both,
        }

        let mode_enum = match mode.to_string().to_lowercase().as_str() {
            "dht" => Mode::Dht,
            "relays" => Mode::Relays,
            _ => Mode::Both,
        };

        let mut builder = pkarr::Client::builder();

        match mode_enum {
        Mode::Dht => {
            builder.no_relays();
        }
        Mode::Relays => {
            builder.no_dht();
            let relay_vec = relays
                .as_slice()
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>();

            if let Err(e) = builder.relays(&relay_vec) {
                godot_error!("❌ Error al configurar relays: {:?}", e);
                return false;
            }
        }
        Mode::Both => {
        godot_print!("modo both en #publicar , no configurado o por defecto");

        }
    }





    let client = match pkarr::Client::builder().build() {
        Ok(c) => c,
        Err(e) => {
            godot_error!("Error al construir el cliente: {:?}", e);
            return false;
        }
    };

        let signed_packet = match SignedPacket::builder()
        .txt(converted.try_into().unwrap(), txt_converted.try_into().unwrap(), ttl)
        .sign(&keypair)
    {

        Ok(packet) => packet,
        Err(e) => {
            godot_error!("❌ Error al firmar el paquete: {:?}", e);
            return false;
        }
    };



        let instant = Instant::now();
        godot_print!("✅ Paquete firmado con clave pública: {}", keypair.public_key());
        let result = futures::executor::block_on(client.publish(&signed_packet, None));




            match result {
                Ok(()) => {
                    godot_print!(
                        "✅ Publicación exitosa: {} en {:?}",
                        keypair.public_key(),
                        instant.elapsed()
                    );
                }
                Err(err) => {
                    godot_error!(
                        "❌ Falló la publicación de {}\nError: {}",
                        keypair.public_key(),
                        err
                    );
                }
            }

                
                true
}







  
    #[func]
    pub fn resolve_key(&mut self, key: GString, mode: GString, relays: PackedStringArray)-> GString {

    enum Mode {
        Dht,
        Relays,
        Both,
    }

    let key_str = key.to_string();
    let public_key = match key_str.as_str().try_into() {
        Ok(pk) => pk,
        Err(_) => {
            godot_error!("❌ Clave zbase32 inválida");
            return "".into();
        }
    };
    // arreglar modos
    let mode_enum = match mode.to_string().to_lowercase().as_str() {
        "dht" => Mode::Dht,
        "relays" => Mode::Relays,
        _ => Mode::Both,
    };

    let mut builder = pkarr::Client::builder();

    match mode_enum {
    Mode::Dht => {
        builder.no_relays();
    }
    Mode::Relays => {
        builder.no_dht();
        let relay_vec = relays
            .as_slice()
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        if let Err(e) = builder.relays(&relay_vec) {
            godot_error!("❌ Error al configurar relays: {:?}", e);
            return "".into();
        }
    }
    
    Mode::Both => {
    godot_print!("modo both o sin modo configurado correctamente en #resolver ");

    }
    
 }



    let client = match builder.build() {
        Ok(c) => c,
        Err(e) => {
            godot_error!("❌ Error al construir cliente: {:?}", e);
            return "".into();
        }
    };
    godot_print!("🔍 Resolviendo clave: {}", key_str);

    let start = std::time::Instant::now();
    let result = futures::executor::block_on(async {
        client.resolve(&public_key).await
    });

    match result {
        Some(packet) => {
            let packet_str = packet.to_string();
            godot_print!(
                "✅ Resuelto en {:?} ms: {}",
                start.elapsed().as_millis(),
                packet
            );
            return GString::from(&packet_str);
        }
        None => {
            godot_warn!("❌ Falló la resolución de {}", key_str);
            return "".into();
        }
    }
}








#[func]
pub fn search(&self, _key: GString, _value: GString, _keypass: PackedByteArray) -> bool {


true

 } 


#[func]
pub fn public_key(&self, key: PackedByteArray) -> GString {
    let bytes = key.to_vec();

    if bytes.len() != 32 {
        godot_error!("La clave debe tener exactamente 32 bytes, pero tiene {}", bytes.len());
        return "".into();
    }

    let mut secret = [0u8; 32];
    secret.copy_from_slice(&bytes);

    let keypair = Keypair::from_secret_key(&secret);
    let pk_str = keypair.public_key().to_string();
    GString::from(&pk_str)
}




//# infoips
//test solo
  #[func]
  pub fn info_ips(&mut self) -> bool {
  {
    godot_print!("AGREGANDO IPS FICTICIAS");
    let _ruta = r"C:\Users\Emabe\Downloads\sample.torrent";
    PEER_IPS.lock().unwrap().insert("peer1".to_string(), vec!["127.0.0.1".to_string()]);
    PEER_IPS.lock().unwrap().insert("peer2".to_string(), vec![
        "127.0.0.1".to_string(),
        "128.34.56.3".to_string()
    ]);
    godot_print!("agrego una ip a peer2 ");
    if let Some(ips) = PEER_IPS.lock().unwrap().get_mut("peer2") {
        ips.push("192.168.0.1".to_string());
    }
}

{
     godot_print!("alisto los ips ");
    if let Some(ips) = PEER_IPS.lock().unwrap().get("peer2") {
            for ip in ips {
                self.base_mut().emit_signal("string_format", &[GString::from(ip).to_variant()]);
                println!("IP: {}", ip);
            }
        }
    
}

{
    godot_print!("agrego una entrada si no existe en key o si no crea la key ");
    let peer_id = "peer1".to_string();
    let ip = "127.0.0.1".to_string();

    let mut map = PEER_IPS.lock().unwrap();
    let entry = map.entry(peer_id).or_insert_with(Vec::new);

    if !entry.contains(&ip) {
        entry.push(ip);
    }
}

   
{
    godot_print!("retiro un dato de la lista");
    let mut map = PEER_IPS.lock().unwrap();
    if let Some(ips) = map.get_mut("peer1") {
        ips.retain(|x| x != "127.0.0.1");
    }
}

{
    godot_print!("solo info si existe el key ");
    let exists = PEER_IPS.lock().unwrap().contains_key("peer1".to_string().as_str());
        if exists {
            println!("ya está registrado");
            
        } else {
        godot_print!("no existe key ");
            
        }
}

    godot_print!("elimino el key ");
    PEER_IPS.lock().unwrap().remove("peer1");


    return true;

    }
//


//# _key
  #[func]
    pub fn _key(&mut self, keys: GString) -> bool {

        let exists = PEER_IPS.lock().unwrap().contains_key(keys.to_string().as_str());
        if exists {
            
            return true;
        } else {
        
            return false;
        }



    }
//



//GET IPS viejo quitar
 #[func]
 fn get_ips(&mut self) -> GString {
    let ips = match GLOBAL_IPS.lock() {
        Ok(i) => i,
        Err(_) => return GString::from("[ERROR DE MUTEX IPs]"),
    };

    let http = match GLOBAL_HTTP.lock() {
        Ok(h) => h,
        Err(_) => return GString::from("[ERROR DE MUTEX HTTP]"),
    };

    let ip_str = if ips.is_empty() {
        "[SIN IPs]".to_string()
    } else {
        format!("IPs actuales: {}", ips.join(", "))
    };

    let http_str = if http.is_empty() {
        "[SIN HTTP]".to_string()
    } else {
        format!("HTTP parseado: {}", http.join(", "))
    };

    let resultado_str = format!("{}\n{}", ip_str, http_str);
    let resultado = GString::from(&resultado_str);


    self.base_mut().emit_signal("ips_actualizadas", &[GString::from(&ip_str).to_variant()]);
    self.base_mut().emit_signal("http_actualizado", &[GString::from(&http_str).to_variant()]);

    resultado
    }

//


//GET_IPFS viejo quitar
    #[func]
    fn get_ipfs(&mut self) -> GString {
        let ips = match IP_IPFS.lock() {
            Ok(i) => i,
            Err(_) => return GString::from("[ERROR DE MUTEX IPs]"),
        };

        let ids = match ID_IPFS.lock() {
            Ok(h) => h,
            Err(_) => return GString::from("[ERROR DE MUTEX IPFS]"),
        };

        let ip_str = if ips.is_empty() {
            "[SIN IPs]".to_string()
        } else {
            format!("IPs actuales: {}", ips.join(", "))
        };

        let idpf_str = if ids.is_empty() {
            "[SIN IDs]".to_string()
        } else {
            format!("IDs parseado: {}", ids.join(", "))
        };

        let resultado_str = format!("{}\n{}", ip_str, idpf_str);
        let resultado = GString::from(&resultado_str);


        self.base_mut().emit_signal("ips_ipfs", &[GString::from(&ip_str).to_variant()]);
        self.base_mut().emit_signal("IDs_ipfs", &[GString::from(&idpf_str).to_variant()]);

        resultado
        }




    }
