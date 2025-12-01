//use godot::classes::Sprite2D;
use godot::prelude::*;
use godot::classes::{Node, RandomNumberGenerator, FileAccess};
use godot::classes::file_access::ModeFlags;
use godot::builtin::{PackedByteArray, StringName, GString};
use bech32::{ToBase32, Variant};
use secp256k1::{KeyPair, Secp256k1, XOnlyPublicKey,SecretKey};// determinar esto 

/*es prueba mada mas */
use nostr::{
    prelude::{hex::ToHex, FromBech32, ToBech32},
  //  secp256k1::{SecretKey, XOnlyPublicKey},

    EventId,
};
use std::str::FromStr;
use nostr::key::XOnlyPublicKey as NostrXOnlyPublicKey;
use nostr::key::SecretKey as NostrSecretKey;

#[derive(GodotClass)]
#[class(base=Node)]
struct Keyl {
    speed: f64,
    angular_speed: f64,
    base: Base<Node>

}


#[godot_api]
impl INode for Keyl {
    fn init(base: Base<Node>) -> Self {
        godot_print!("keyl!"); // Prints to the Godot console

        Self {
            speed: 400.0,
            angular_speed: std::f64::consts::PI,
            base,
        }
    }

    fn physics_process(&mut self, _delta: f64) {
        // GDScript code:
        //
        // rotation += angular_speed * delta
        // var velocity = Vector2.UP.rotated(rotation) * speed
        // position += velocity * delta

        self.angular_speed += 1.0;


        // or verbose:
        // let this = self.base_mut();
        // this.set_position(
        //     this.position() + velocity * delta as f32
        // );
    }
}

#[godot_api]
impl Keyl {
    #[func]
    fn increase_speed(&mut self, amount: f64) {
        self.speed += amount;
        self.base_mut().emit_signal("speed_increased", &[]);
    }

    #[signal]
    fn speed_increased();



#[func]
pub fn public_key(&self, key: PackedByteArray) -> GString {
    let bytes = key.to_vec();

    if bytes.len() != 32 {
        godot_error!("La clave debe tener exactamente 32 bytes, pero tiene {}", bytes.len());
        return "".into();
    }
    
    let mut secret = [0u8; 32];
    secret.copy_from_slice(&bytes);
    
    "".into()

}

#[func]
pub fn generate_key(&self) -> PackedByteArray {
    let secp = Secp256k1::new();
    let mut rng = rand::rngs::OsRng::default();
    let keypair = KeyPair::new(&secp, &mut rng);
    let secret = keypair.secret_key().secret_bytes();
    PackedByteArray::from(secret.to_vec())
}


    /* 
 #[func]
 pub fn to_npub(pubkey: &XOnlyPublicKey) -> String {
        bech32::encode("npub", pubkey.serialize().to_base32(), Variant::Bech32).unwrap()
    }
    #[func]
    pub fn to_nsec(keypair: &KeyPair) -> String {
        bech32::encode(
            "nsec",
            keypair.secret_key().secret_bytes().to_base32(),
            Variant::Bech32,
        )
        .unwrap()
    }


#[func]
pub fn to_npub(&self, pubkey: PackedByteArray) -> GString {
    let bytes = pubkey.to_vec();

    if bytes.len() != 32 {
        godot_error!("La clave pública debe tener 32 bytes, pero tiene {}", bytes.len());
        return "".into();
    }

    let mut key_bytes = [0u8; 32];
    key_bytes.copy_from_slice(&bytes);

    match bech32::encode("npub", key_bytes.to_base32(), Variant::Bech32) {
        Ok(encoded) => encoded.into(),
        Err(e) => {
            godot_error!("Error al codificar npub: {}", e);
            "".into()
        }
    }
}

*/

#[func]
pub fn to_nsec(&self, secret: PackedByteArray) -> GString {
    let bytes = secret.to_vec();

    if bytes.len() != 32 {
        godot_error!("La clave privada debe tener 32 bytes, pero tiene {}", bytes.len());
        return "".into();
    }

    let mut key_bytes = [0u8; 32];
    key_bytes.copy_from_slice(&bytes);

    match bech32::encode("nsec", key_bytes.to_base32(), Variant::Bech32) {
        Ok(encoded) => {
        godot_print!("✅ nsec codificado: {}", encoded); 
        GString::from(&encoded)
        }
        Err(e) => {
            godot_error!("Error al codificar nsec: {}", e);
            "".into()
        }
    }
}

#[func]
pub fn to_npub(&self, secret: PackedByteArray) -> GString {
    let bytes = secret.to_vec();

    if bytes.len() != 32 {
        godot_error!("❌ La clave privada debe tener 32 bytes, pero tiene {}", bytes.len());
        return "".into();
    }

    let mut key_bytes = [0u8; 32];
    key_bytes.copy_from_slice(&bytes);

    let secp = Secp256k1::new();
    let sk = match secp256k1::SecretKey::from_slice(&key_bytes) {
        Ok(sk) => sk,
        Err(e) => {
            godot_error!("❌ Clave privada inválida: {}", e);
            return "".into();
        }
    };

    let keypair = KeyPair::from_secret_key(&secp, &sk);
    let pubkey = keypair.x_only_public_key().0;

    match bech32::encode("npub", pubkey.serialize().to_base32(), Variant::Bech32) {
        Ok(encoded) => {
        godot_print!("✅ npub codificado: {}", encoded); 
        GString::from(&encoded)
        }
        Err(e) => {
            godot_error!("❌ Error al codificar npub: {}", e);
            "".into()
        }
    }
}



    #[func]
    fn hex_npub(&self, npub: GString) -> GString {
        let s = npub.to_string();

        match NostrXOnlyPublicKey::from_bech32(&s) {
            Ok(public_key) => {
                let hex = public_key.to_hex();
                godot_print!("✅ Clave pública hex: {}", hex);
                GString::from(&hex)
            }
            Err(e) => {
                godot_error!("❌ Error al decodificar npub hex: {}", e);
                "".into()
            }
        }
    }


    #[func]
    fn hex_nsec(&self, nsec: GString) -> GString {
        let s = nsec.to_string();

        match NostrSecretKey::from_bech32(&s) {
            Ok(secret_key) => {
                let hex = secret_key.secret_bytes().to_vec().to_hex();
                godot_print!("✅ Clave secreta hex: {}", hex);
                GString::from(&hex)

            }
            Err(e) => {
                godot_error!("❌ Error al decodificar nsec hex: {}", e);
                "".into()
            }
        }
    }


#[func]
fn npub(&self, npub: GString) -> GString {
    let s = npub.to_string();

    match NostrXOnlyPublicKey::from_str(&s) {
        Ok(pubkey) => match pubkey.to_bech32() {
            Ok(bech32) => GString::from(&bech32),
            Err(e) => {
                godot_error!("❌ Error al convertir a bech32: {}", e);
                "".into()
            }
        },
        Err(e) => {
            godot_error!("❌ Error al parsear clave pública: {}", e);
            "".into()
        }
    }
}


#[func]
fn nsec(&self, nsec: GString) -> GString {
    let s = nsec.to_string();

    match NostrSecretKey::from_str(&s) {
        Ok(pubkey) => match pubkey.to_bech32() {
            Ok(bech32) => GString::from(&bech32),
            Err(e) => {
                godot_error!("❌ Error al convertir a bech32: {}", e);
                "".into()
            }
        },
        Err(e) => {
            godot_error!("❌ Error al parsear clave secreta: {}", e);
            "".into()
        }
    }
}


}