use std::collections::BTreeMap;
use std::io::{Cursor, Read};

use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use byteorder::LittleEndian;
use byteorder::ReadBytesExt;
use jsonwebtoken::{DecodingKey, Validation};
use serde_json::Value;
use varint_rs::VarintReader;

use serialize::error::{DeserilizationError, SerilizationError};
use serialize::proto::de::MCProtoDeserialize;
use serialize::proto::ser::MCProtoSerialize;

#[derive(Debug)]
pub struct ConnectionRequestType {
    /// Array of Base64 encoded JSON Web Token certificates to authenticate the player.
    ///
    /// The last certificate in the chain will have a property 'extraData' that contains player identity information including the XBL XUID (if the player was signed into XBL at the time of the connection).
    pub certificate_chain: Vec<BTreeMap<String, Value>>,
    /// Base64 encoded JSON Web Token that contains other relevant client properties.
    ///
    /// Properties Include:
    /// - SelfSignedId
    /// - ServerAddress = (unresolved url if applicable)
    /// - ClientRandomId
    /// - SkinId
    /// - SkinData
    /// - SkinImageWidth
    /// - SkinImageHeight
    /// - CapeData
    /// - CapeImageWidth
    /// - CapeImageHeight
    /// - SkinResourcePatch
    /// - SkinGeometryData
    /// - SkinGeometryDataEngineVersion
    /// - SkinAnimationData
    /// - PlayFabId
    /// - AnimatedImageData = Array of:
    ///   - Type
    ///   - Image
    ///   - ImageWidth
    ///   - ImageHeight
    ///   - Frames
    ///   - AnimationExpression
    /// - ArmSize
    /// - SkinColor
    /// - PersonaPieces = Array of:
    ///   - PackId
    ///   - PieceId
    ///   - IsDefault
    ///   - PieceType
    ///   - ProuctId
    /// - PieceTintColors = Array of:
    ///   - PieceType
    ///   - Colors = Array of color hexstrings
    /// - IsEduMode (if edu mode)
    /// - TenantId (if edu mode)
    /// - ADRole (if edu mode)
    /// - IsEditorMode
    /// - GameVersion
    /// - DeviceModel
    /// - DeviceOS = (see enumeration: BuildPlatform)
    /// - DefaultInputMode = (see enumeration: InputMode)
    /// - CurrentInputMode = (see enumeration: InputMode)
    /// - UIProfile = (see enumeration: UIProfile)
    /// - GuiScale
    /// - LanguageCode
    /// - PlatformUserId
    /// - ThirdPartyName
    /// - ThirdPartyNameOnly
    /// - PlatformOnlineId
    /// - PlatformOfflineId
    /// - DeviceId
    /// - TrustedSkin
    /// - PremiumSkin
    /// - PersonaSkin
    /// - OverrideSkin
    /// - CapeOnClassicSkin
    /// - CapeId
    /// - CompatibleWithClientSideChunkGen
    pub raw_token: BTreeMap<String, Value>,
}

// TODO: Add MCSerialize
impl MCProtoSerialize for ConnectionRequestType {
    fn proto_serialize(&self, buf: &mut Vec<u8>) -> Result<(), SerilizationError>
    where
        Self: Sized,
    {
        // Write entire length
        // 8 = i32 + i32 for length of both
        //match buf.write_u64_varint((self.certificate_chain.len() + self.certificate_chain.len() + 8) as u64) {
        //    Ok(_) => {}
        //    Err(_) => { return Err(SerilizationError::WriteVarintError) }
        //};

        // write length of certificate_chain vec
        // buf.put_i32_le(self.certificate_chain.len() as i32);

        // write strings (certificate_chain)
        //buf.put_slice(&self.certificate_chain.as_bytes());

        // write length of raw_token vec
        // buf.put_i32_le(self.raw_token.len() as i32);

        // write strings (raw_token)
        //buf.put_slice(&self.raw_token.as_bytes());

        Ok(())
    }
}

// TODO: Add microsoft auth
impl MCProtoDeserialize for ConnectionRequestType {
    fn proto_deserialize(cursor: &mut Cursor<Vec<u8>>) -> Result<Self, DeserilizationError>
    where
        Self: Sized,
    {
        let mut certificate_chain: Vec<BTreeMap<String, Value>> = vec![];

        // read the ConnectionRequest's length
        // (certificate_chain len + raw_token len + 8)
        // 8 = i32 len + i32 len (length of certificate_chain's len and raw_token's len)
        // can be ignored, other lengths are provided
        match cursor.read_u64_varint() {
            Ok(_) => {}
            Err(_) => return Err(DeserilizationError::ReadIOError),
        };

        // read length of certificate_chain vec
        let certificate_chain_len = match cursor.read_i32::<LittleEndian>() {
            Ok(l) => l,
            Err(_) => return Err(DeserilizationError::ReadIOError),
        };

        let mut certificate_chain_buf = vec![0; certificate_chain_len as usize];

        // read string data (certificate_chain)
        match cursor.read_exact(&mut certificate_chain_buf) {
            Ok(_) => {}
            Err(_) => return Err(DeserilizationError::NotEnoughRemainingError),
        };

        // transform into string
        let certificate_chain_string = match String::from_utf8(certificate_chain_buf) {
            Ok(v) => v,
            Err(_) => return Err(DeserilizationError::ReadUtf8StringError),
        };

        // parse certificate chain string into json
        let certificate_chain_json = match serde_json::from_str(certificate_chain_string.as_str()) {
            Ok(v) => v,
            Err(_) => return Err(DeserilizationError::ReadJsonError),
        };

        let certificate_chain_json_jwts = match certificate_chain_json {
            Value::Object(mut v) => {
                match v.get_mut("chain") {
                    None => {
                        // the certificate chain should always be a object with just an array of jwts called "chain"
                        return Err(DeserilizationError::ReadJsonError);
                    }
                    Some(v) => {
                        match v.take() {
                            Value::Array(v) => v,
                            _ => {
                                // the certificate chain should always be a object with just an array of jwts called "chain"
                                return Err(DeserilizationError::ReadJsonError);
                            }
                        }
                    }
                }
            }
            _ => {
                // the certificate chain should always be a object with just an array of jwts called "chain"
                return Err(DeserilizationError::ReadJsonError);
            }
        };

        let mut key_data = vec![];

        for jwt_json in certificate_chain_json_jwts {
            let jwt_string = match jwt_json {
                Value::String(str) => str,
                _ => {
                    // the certificate chain's should always be a jwt string
                    return Err(DeserilizationError::ReadJsonError);
                }
            };

            let jwt_header = match jsonwebtoken::decode_header(jwt_string.as_str()) {
                Ok(v) => v,
                Err(_) => return Err(DeserilizationError::ReadJwtError),
            };

            let mut jwt_validation = Validation::new(jwt_header.alg);
            // TODO: This definitely is not right. Even Zuri-MC doesn't understand this
            jwt_validation.insecure_disable_signature_validation();
            jwt_validation.set_required_spec_claims::<&str>(&[]);
            jwt_validation.set_issuer::<&str>(&["Mojang"]);

            // Is first jwt, use self-signed header from x5u
            if key_data.is_empty() {
                let x5u = match jwt_header.x5u {
                    None => return Err(DeserilizationError::MissingField),
                    Some(ref v) => v.as_bytes(),
                };

                key_data = match BASE64_STANDARD.decode(x5u) {
                    Ok(v) => v,
                    Err(_) => return Err(DeserilizationError::Base64Error),
                };
            }

            // Decode the jwt string into a jwt
            let jwt = match jsonwebtoken::decode::<BTreeMap<String, Value>>(
                jwt_string.as_str(),
                &DecodingKey::from_ec_der(&key_data),
                &jwt_validation,
            ) {
                Ok(v) => v,
                Err(e) => {
                    println!("ERROR: {e}");
                    return Err(DeserilizationError::ReadJwtError);
                }
            };

            key_data = match jwt.claims.get("identityPublicKey") {
                None => return Err(DeserilizationError::MissingField),
                Some(v) => match v {
                    Value::String(str) => match BASE64_STANDARD.decode(str.as_bytes()) {
                        Ok(v) => v.clone(),
                        Err(_) => return Err(DeserilizationError::Base64Error),
                    },
                    _ => return Err(DeserilizationError::ReadJsonError),
                },
            };

            certificate_chain.push(jwt.claims);
        }

        // read length of certificate_chain vec
        let raw_token_len = match cursor.read_i32::<LittleEndian>() {
            Ok(l) => l,
            Err(_) => return Err(DeserilizationError::ReadIOError),
        };

        let mut raw_token_buf = vec![0; raw_token_len as usize];

        // read string data (certificate_chain)
        match cursor.read_exact(&mut raw_token_buf) {
            Ok(_) => {}
            Err(_) => return Err(DeserilizationError::NotEnoughRemainingError),
        };

        // transform into string
        let raw_token_string = match String::from_utf8(raw_token_buf) {
            Ok(v) => v,
            Err(_) => return Err(DeserilizationError::ReadUtf8StringError),
        };

        // let raw_token =
        //     match jwtk::decode_without_verify::<BTreeMap<String, Value>>(raw_token_string.as_str())
        //     {
        //         Ok(v) => v.claims().extra.clone(),
        //         Err(_) => return Err(DeserilizationError::ReadJwtError),
        //     };

        return Ok(Self {
            certificate_chain,
            raw_token: BTreeMap::new(),
        });
    }
}
