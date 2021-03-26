use actix_web::http::header::{HeaderName, HeaderMap, HeaderValue, Date};
use actix_web::client::Client;
use serde::Deserialize;
use anyhow::{Result, anyhow};
use std::time::SystemTime;
use openssl::sign::{RsaPssSaltlen, Signer, Verifier};
use openssl::rsa::{Rsa, Padding};
use openssl::pkey::{PKey, Private};
use openssl::hash::MessageDigest;
use openssl::base64::{encode_block, decode_block};
use log::info;

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

    // Only case where we sign a signature with no id is when sending an error resposne
    let user;
    if req_headers.contains_key("user-id") {
        user = req_headers.get("user-id").unwrap().to_str()?;
    } else {
        user = "-1";
    }
    res_headers.insert(HeaderName::from_static("user-id"), HeaderValue::from_str(user)?);

    // sign
    // TODO: check if we want to not just create our own host header, instead of using the
    // others host
    let string_to_sign = format!("*request-target: {} {}\ndate: {}\nuser-id: {}",
    req_method.to_lowercase(),
    req_path,
    date.to_string(),
    user);

    let mut signer = Signer::new(MessageDigest::sha512(), &private_key)?;
    signer.set_rsa_padding(Padding::PKCS1_PSS)?;
    signer.set_rsa_mgf1_md(MessageDigest::sha512())?;
    signer.set_rsa_pss_saltlen(RsaPssSaltlen::custom(20))?;
    signer.update(string_to_sign.as_ref())?;
    let signature = signer.sign_to_vec()?;
    let enc_signature = encode_block(&signature);
    let final_signature = format!("sig1=:{}:", enc_signature);

    res_headers.insert(HeaderName::from_static("signature"), HeaderValue::from_str(&final_signature)?);
    Ok(())
}

#[derive(Debug)]
struct SignatureInput {
    alg: String,
    created: u64,
    expires: u64,
    key_id: String,
    covered_content: String,
}

pub async fn check_signature(req_headers: &HeaderMap, req_path: &str, req_method: &str) -> Result<String> {

    let mut sig_input_struct = SignatureInput {
        alg: String::from(""),
        created: 0,
        expires: 0,
        key_id: String::from(""),
        covered_content: String::from(""),
    };

    // get header value for sig input
    let signature_input = req_headers.get("Signature-Input").unwrap().to_str().unwrap();

    // TODO: check for 0 splits, could be caught later potentially. - Darren
    let iter_signature_input = signature_input.split(';');

    // build struct and do soft validation on signature-input header contents
    // TODO: can make this better by using an enum but im lazy x - Darren
    // TODO: realistically only the created and expires will fail to parse, but these
    // aren't in the protocl so we can kinda of ignore them for now - Darren
    for entry in iter_signature_input {
        let trim_entry = entry.trim();
        if trim_entry.starts_with("sig1=") {
            sig_input_struct.covered_content = trim_entry.strip_prefix("sig1=").unwrap().to_string();
        }
        else if trim_entry.starts_with("alg=") {
            sig_input_struct.alg = trim_entry.strip_prefix("alg=").unwrap().to_string();
        }
        else if trim_entry.starts_with("created=") {
            sig_input_struct.created = trim_entry.strip_prefix("created=").unwrap().parse::<u64>().unwrap();
        }
        else if trim_entry.starts_with("expires=") {
            sig_input_struct.expires = trim_entry.strip_prefix("expires=").unwrap().parse::<u64>().unwrap();
        }
        else if trim_entry.starts_with("keyId=") {
            sig_input_struct.key_id = trim_entry.strip_prefix("keyId=").unwrap().to_string();
        }
        else {
            // invalid attribute used in request
            return Err(anyhow!("Error: Invalid signature-input attribute: {}", trim_entry));
        }
    }
    // check covered_content / sig1= is valid with headers

    // check starts and ends with parenthesis
    if !sig_input_struct.covered_content.starts_with('(') || !sig_input_struct.covered_content.ends_with(')') {
        // if not, then error
        return Err(anyhow!("Error: Invalid sig1=, not surrounded by parenthesis: {}", sig_input_struct.covered_content));
    }
    sig_input_struct.covered_content = sig_input_struct.covered_content.strip_prefix("(").unwrap().to_string();
    sig_input_struct.covered_content = sig_input_struct.covered_content.strip_suffix(")").unwrap().to_string();
    let iter = sig_input_struct.covered_content.split(',');

    // check each one against headers, dealing with speical * cases
    // meanwhile building signature input

    /* * cases
     * *request-target
     * *created - not impl
     * *expires - not impl
     */

    let mut signature_strings = Vec::new();
    for field in iter {
        let field_trim = field.trim();

        let req_tar_string = "*request-target";
        if field_trim.starts_with(req_tar_string) {
            signature_strings.push(format!("{}: {} {}", req_tar_string, req_method, req_path));
        }
        // TODO: if statements for *created and *expires

        else {
            // check field against headers, if exist add, else error
            if !req_headers.contains_key(field_trim) {
                return Err(anyhow!("Error: No header exists for: {}", field_trim));
            }

            // add to vector
            signature_strings.push(
                // TODO: header might not contain valid ascii, worker will panic right
                // now, should return error if fail
                format!("{}: {}", field_trim, req_headers.get(field_trim).unwrap().to_str().unwrap())
            );
        }
    }

    // used so serde can deal with the key: <key> from the api response
    #[derive(Deserialize)]
    struct Key {
        key: String,
    }

    // TODO: potentially cache key in db per domain
    // TODO: extract domain from keyId

    // makes request for key, returning an error if this fails, or key isn't a valid
    // string
    let client = Client::default();
    let unparsed_key = match client.get(&sig_input_struct.key_id).send().await {
       Ok(mut response) => match response.json::<Key>().await {
           Ok(key) => key,
           Err(e) => return Err(anyhow!("Error: parsing body for key: {}", e)),
       },
       Err(e) => return Err(anyhow!("Error: while making key req to keyId: {}", e)),
    };

    // build string to sign
    let size = signature_strings.len();
    let mut index: usize = 1;
    let mut string_to_sign = String::with_capacity(300); // arbritary alloc, should be enough for most signature inputs without any reallocs
    for field in signature_strings {
       if size == index {
           string_to_sign.push_str(&field);
       }
       else {
           string_to_sign.push_str(&format!("{}\n", field));
           index += 1;
       }
    }
    // have checked signature exists, value should be a valid string (hopefully)
    let mut enc_signature = req_headers.get("signature").unwrap().to_str().unwrap();
    // format: sig1=:<enc_signature>:
    if !enc_signature.starts_with("sig1=:") ||
       !enc_signature.ends_with(':') {
        return Err(anyhow!("Error: invalid signature format, must be 'sig1=:<enc_signature>:'"))
    }
    enc_signature = enc_signature.strip_prefix("sig1=:").unwrap()
        .strip_suffix(":").unwrap();

    let public_key_parsed = match Rsa::public_key_from_pem_pkcs1(&unparsed_key.key.as_ref()) {
        Ok(key) => PKey::from_rsa(key).unwrap(),
        Err(e) => return Err(anyhow!("Error: parsing public key: {}", e)),
    };

    let mut verifier = Verifier::new(MessageDigest::sha512(), &public_key_parsed).unwrap();
    verifier.set_rsa_padding(Padding::PKCS1_PSS).unwrap();
    verifier.set_rsa_mgf1_md(MessageDigest::sha512())?;
    verifier.set_rsa_pss_saltlen(RsaPssSaltlen::custom(20))?;
    verifier.update(string_to_sign.as_ref()).unwrap();

    let denc_signature = match decode_block(&enc_signature) {
        Ok(decoded) => decoded,
        Err(e) => return Err(anyhow!("Error: decoding signature from base64: {}", e)),
    };

    if verifier.verify(&denc_signature).unwrap() {
        info!("Successful Request from: {}", &sig_input_struct.key_id);
        // get implementation id by querying database
        let full_url = sig_input_struct.key_id.clone();
        let parsed_url = full_url.split("/api/").collect::<Vec<_>>()[0];
        return Ok(parsed_url.to_string())
    } else {
        return Err(anyhow!("Error: verifying signature, may not match, signed with this string: \n{}", string_to_sign))
    }
}
