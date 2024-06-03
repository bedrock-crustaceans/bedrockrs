use std::collections::BTreeMap;
use std::io::{Cursor, Read};

use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use bedrock_core::stream::read::ByteStreamRead;
use bedrock_core::stream::write::ByteStreamWrite;
use jsonwebtoken::{DecodingKey, Validation};
use proto_core::error::ProtoCodecError;
use proto_core::ProtoCodec;
use serde_json::Value;

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

impl ProtoCodec for ConnectionRequestType {
    fn proto_serialize(&self, buf: &mut ByteStreamWrite) -> Result<(), ProtoCodecError>
    where
        Self: Sized,
    {
        todo!()
    }

    // TODO: Add microsoft auth
    fn proto_deserialize(cursor: &mut ByteStreamRead) -> Result<Self, ProtoCodecError>
    where
        Self: Sized,
    {
        let mut certificate_chain: Vec<BTreeMap<String, Value>> = vec![];

        // read the ConnectionRequests length
        // (certificate_chain len + raw_token len + 8)
        // 8 = i32 len + i32 len (length of certificate_chain's len and raw_token's len)
        // can be ignored, other lengths are provided
        match cursor.read_uvar32() {
            Ok(_) => {}
            Err(e) => return Err(ProtoCodecError::IOError(e)),
        };

        // read length of certificate_chain vec
        let certificate_chain_len = match cursor.read_i32le() {
            Ok(l) => l.0,
            Err(e) => return Err(ProtoCodecError::IOError(e)),
        };

        let mut certificate_chain_buf = vec![0; certificate_chain_len as usize];

        // read string data (certificate_chain)
        match cursor.read_exact(&mut certificate_chain_buf) {
            Ok(_) => {}
            Err(e) => return Err(ProtoCodecError::IOError(e)),
        };

        // transform into string
        let certificate_chain_string = match String::from_utf8(certificate_chain_buf) {
            Ok(v) => v,
            Err(e) => return Err(ProtoCodecError::UTF8Error(e)),
        };

        // parse certificate chain string into json
        let certificate_chain_json = match serde_json::from_str(certificate_chain_string.as_str()) {
            Ok(v) => v,
            Err(e) => return Err(ProtoCodecError::JsonError(e)),
        };

        let certificate_chain_json_jwts = match certificate_chain_json {
            Value::Object(mut v) => {
                match v.get_mut("chain") {
                    None => {
                        // the certificate chain should always be an object with just an array of
                        // JWTs called "chain"
                        return Err(ProtoCodecError::FormatMismatch(String::from(
                            "Missing element \"chain\" in JWT certificate_chain",
                        )));
                    }
                    Some(v) => {
                        match v.take() {
                            Value::Array(v) => v,
                            other => {
                                // the certificate chain should always be an object with just an
                                // array of JWTs called "chain"
                                return Err(ProtoCodecError::FormatMismatch(format!("Expected \"chain\" in JWT certificate_chain to be an Array, but got {other:?}")));
                            }
                        }
                    }
                }
            }
            other => {
                // the certificate chain should always be an object with just an array of
                // JWTs called "chain"
                return Err(ProtoCodecError::FormatMismatch(format!(
                    "Expected Object in base of JWT certificate_chain, got {other:?}"
                )));
            }
        };

        let mut key_data = vec![];

        for jwt_json in certificate_chain_json_jwts {
            let jwt_string = match jwt_json {
                Value::String(str) => str,
                other => {
                    // the certificate chain's should always be a jwt string
                    return Err(ProtoCodecError::FormatMismatch(format!("Expected chain array in certificate_chain to just contain Strings, but got {other:?}")));
                }
            };

            let jwt_header = match jsonwebtoken::decode_header(jwt_string.as_str()) {
                Ok(v) => v,
                Err(e) => return Err(ProtoCodecError::JwtError(e)),
            };

            let mut jwt_validation = Validation::new(jwt_header.alg);
            // TODO: This definitely is not right. Even Zuri-MC doesn't understand this.. I may understand it.. I do understand it
            jwt_validation.insecure_disable_signature_validation();
            jwt_validation.set_required_spec_claims::<&str>(&[]);

            // Is first jwt, use self-signed header from x5u
            if key_data.is_empty() {
                let x5u = match jwt_header.x5u {
                    None => {
                        return Err(ProtoCodecError::FormatMismatch(String::from(
                            "Expected x5u in JWT header",
                        )));
                    }
                    Some(ref v) => v.as_bytes(),
                };

                key_data = match BASE64_STANDARD.decode(x5u) {
                    Ok(v) => v,
                    Err(e) => return Err(ProtoCodecError::Base64DecodeError(e)),
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
                    return Err(ProtoCodecError::JwtError(e));
                }
            };

            key_data = match jwt.claims.get("identityPublicKey") {
                None => return Err(ProtoCodecError::FormatMismatch(String::from("Expected identityPublicKey field in JWT for validation"))),
                Some(v) => match v {
                    Value::String(str) => match BASE64_STANDARD.decode(str.as_bytes()) {
                        Ok(v) => v.clone(),
                        Err(e) => return Err(ProtoCodecError::Base64DecodeError(e)),
                    },
                    other => return Err(ProtoCodecError::FormatMismatch(format!("Expected identityPublicKey field in JWT to be of type String, got {other:?}"))),
                },
            };

            certificate_chain.push(jwt.claims);
        }

        // read length of certificate_chain vec
        let raw_token_len = match cursor.read_i32le() {
            Ok(l) => l.0,
            Err(e) => return Err(ProtoCodecError::IOError(e)),
        };

        let mut raw_token_buf = vec![0; raw_token_len as usize];

        // read string data (certificate_chain)
        match cursor.read_exact(&mut raw_token_buf) {
            Ok(_) => {}
            Err(e) => return Err(ProtoCodecError::IOError(e)),
        };

        // transform into string
        let raw_token_string = match String::from_utf8(raw_token_buf) {
            Ok(v) => v,
            Err(e) => return Err(ProtoCodecError::UTF8Error(e)),
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
