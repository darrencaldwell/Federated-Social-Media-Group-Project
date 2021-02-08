use actix_web::http::header::{HeaderName, HeaderMap, HeaderValue, Date};
use anyhow::Result;
use std::time::SystemTime;
use openssl::sign::{Signer};
use openssl::rsa::Padding;
use openssl::pkey::{PKey, Private};
use openssl::hash::MessageDigest;
use openssl::base64::{encode_block};

/// Signs
pub fn sign_signature<'a>(res_headers: &'a mut HeaderMap,
                          req_headers: &'a HeaderMap,
                          req_method: &'a str,
                          req_path: &'a str,
                          private_key: &'a PKey<Private>) -> Result<()> {

            // create signature-input header
            // need sig1=(x, y, z); keyId=x
            let header_string = "sig1=(*request-target, date, user-id); keyId=https://cs3099user-b5.host.cs.st-andrews.ac.uk/api/key; alg=RSASSA-PSS-SHA512";
            res_headers.insert(HeaderName::from_static("signature-input"), HeaderValue::from_static(header_string));
            let date = Date(SystemTime::now().into());
            res_headers.insert(HeaderName::from_static("date"), HeaderValue::from_str(&date.to_string())?);

            //TODO: what if userid doesn't exist!!
            let user = req_headers.get("user-id").unwrap().to_str()?;
            res_headers.insert(HeaderName::from_static("user-id"), HeaderValue::from_str(user)?);

            // sign
            // TODO: check if we want to not just create our own host header, instead of using the
            // others host
            let string_to_sign = format!("*request-target: {} {}\ndate: {}\nuser-id: {}",
            req_method.to_lowercase(),
            req_path,
            date.to_string(),
            user);
            println!("{:?}",string_to_sign);

            let mut signer = Signer::new(MessageDigest::sha512(), &private_key)?;
            signer.set_rsa_padding(Padding::PKCS1_PSS)?;
            signer.update(string_to_sign.as_ref())?;
            let signature = signer.sign_to_vec()?;
            let enc_signature = encode_block(&signature);

            res_headers.insert(HeaderName::from_static("signature"), HeaderValue::from_str(&enc_signature)?);
            Ok(())
}
pub fn check_signature(req_headers: &HeaderMap) -> Result<bool> {
    Ok(true)
}
