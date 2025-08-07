use anyhow::{Context, Result, anyhow};
use base64::{Engine, prelude::BASE64_URL_SAFE_NO_PAD};
use jsonwebtoken::{
    DecodingKey, EncodingKey,
    jwk::{
        AlgorithmParameters, CommonParameters, EllipticCurve, Jwk, JwkSet, OctetKeyPairParameters,
        PublicKeyUse,
    },
};
use ring::signature::KeyPair;
use std::{collections::HashMap, path::PathBuf};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use uuid::Uuid;

use crate::{api::types::jwt_claim::Claims, config::types::Config};

pub struct JwtIssuer {
    header: jsonwebtoken::Header,
    key_pairs: HashMap<Uuid, JwtKeyPair>,
    iss: String,
    aud: String,
}

pub struct JwtKeyPair {
    private_key: EncodingKey,
    public_key: DecodingKey,
    pub x: String,
}

impl JwtIssuer {
    pub async fn new(config: &Config) -> Result<Self> {
        let header = jsonwebtoken::Header::new(jsonwebtoken::Algorithm::EdDSA);
        let iss = config.jwks.iss.clone();
        let aud = config.jwks.aud.clone();

        let mut key_pairs = HashMap::new();
        for jwk_config in &config.jwks.keys {
            let key_path_string = format!("{}/{}.pem", config.jwks.keys_path, jwk_config.kid);
            let key_pair = read_or_generate_jwks_pkcs8(key_path_string.into()).await?;
            key_pairs.insert(jwk_config.kid, key_pair);
        }

        Ok(JwtIssuer {
            header,
            iss,
            aud,
            key_pairs,
        })
    }

    pub fn get_kid(&self) -> Uuid {
        self.key_pairs.keys().next().cloned().unwrap()
    }

    pub fn issue_jwt(&self, kid: Uuid, sub: Uuid) -> Result<String> {
        let mut header = self.header.clone();
        header.kid = Some(kid.to_string());

        let private_key = self
            .key_pairs
            .get(&kid)
            .or_else(|| self.key_pairs.values().next())
            .ok_or_else(|| anyhow!("fail to get jwt key pair"))?
            .private_key
            .clone();

        let now = chrono::Utc::now();
        let claim = Claims {
            aud: self.aud.clone(),
            iss: self.iss.clone(),
            sub: sub,
            exp: (now + chrono::Duration::hours(1)).timestamp(),
            jti: Uuid::now_v7(),
            iat: now.timestamp(),
            nbf: now.timestamp(),
        };

        let jwt = jsonwebtoken::encode(&header, &claim, &private_key)
            .map_err(|e| anyhow!("fail to encode jwt: {}", e))
            .context("fail to issue jwt")?;

        Ok(jwt)
    }

    pub fn jwks(&self) -> Result<JwkSet> {
        let mut keys = Vec::new();
        for (kid, key_pair) in self.key_pairs.iter() {
            let jwk = Jwk {
                common: CommonParameters {
                    key_id: Some(kid.to_string()),
                    key_algorithm: Some(jsonwebtoken::jwk::KeyAlgorithm::EdDSA),
                    public_key_use: Some(PublicKeyUse::Signature),
                    ..Default::default()
                },
                algorithm: AlgorithmParameters::OctetKeyPair(OctetKeyPairParameters {
                    key_type: jsonwebtoken::jwk::OctetKeyPairType::OctetKeyPair,
                    curve: EllipticCurve::Ed25519,
                    x: key_pair.x.clone(),
                }),
            };
            keys.push(jwk);
        }
        Ok(JwkSet { keys })
    }
}

async fn read_or_generate_jwks_pkcs8(path: PathBuf) -> Result<JwtKeyPair> {
    if let Ok(mut fs) = tokio::fs::File::options()
        .read(true)
        .open(path.clone())
        .await
    {
        let mut buf = Vec::with_capacity(100);
        fs.read_to_end(&mut buf)
            .await
            .context("fail to read jwks file")?;
        let key_pair = ring::signature::Ed25519KeyPair::from_pkcs8(&buf)
            .context("fail to read pkcs8 jwk key")?;

        let private_key = jsonwebtoken::EncodingKey::from_ed_der(&buf);
        let public_key = jsonwebtoken::DecodingKey::from_ed_der(key_pair.public_key().as_ref());

        return Ok(JwtKeyPair {
            private_key,
            public_key,
            x: BASE64_URL_SAFE_NO_PAD.encode(key_pair.public_key().as_ref()),
        });
    }

    if let Ok(mut fs) = tokio::fs::File::options()
        .write(true)
        .create_new(true)
        .open(path.clone())
        .await
    {
        let rng = ring::rand::SystemRandom::new();
        let gen_key = ring::signature::Ed25519KeyPair::generate_pkcs8(&rng)
            .context("fail to generate pkcs8 jwk key")?;
        fs.write_all(gen_key.as_ref()).await?;
        let key_pair = ring::signature::Ed25519KeyPair::from_pkcs8(gen_key.as_ref())
            .context("fail to read pkcs8 jwk key")?;

        let private_key = jsonwebtoken::EncodingKey::from_ed_der(gen_key.as_ref());
        let public_key = jsonwebtoken::DecodingKey::from_ed_der(key_pair.public_key().as_ref());

        return Ok(JwtKeyPair {
            private_key,
            public_key,
            x: BASE64_URL_SAFE_NO_PAD.encode(key_pair.public_key().as_ref()),
        });
    }

    Err(anyhow::anyhow!(
        "fail to read or generate jwks pkce8 at {}",
        path.display()
    ))
}
