use anyhow::Ok;
use anyhow::Result;
use rand::rngs::StdRng;
use rand::SeedableRng;
use rsa::pkcs1::DecodeRsaPrivateKey;
use rsa::pkcs1::DecodeRsaPublicKey;
use rsa::pkcs1::EncodeRsaPublicKey;
use rsa::pkcs1v15::{Signature, SigningKey, VerifyingKey};
use rsa::signature::{RandomizedSigner, SignatureEncoding, Verifier};
use rsa::{pkcs1::EncodeRsaPrivateKey, pkcs8::LineEnding, RsaPrivateKey, RsaPublicKey};
use sha2::Sha256;

pub fn gen_private_key_pem() -> (String, String) {
    let bits = 2048;
    let mut rng = StdRng::from_entropy();
    let private_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
    let priv_pem = private_key
        .to_pkcs1_pem(LineEnding::LF)
        .expect("failed to encode private key");

    let public_key = private_key.to_public_key();
    let pub_pem = public_key
        .to_pkcs1_pem(LineEnding::LF)
        .expect("failed to encode public key");

    (priv_pem.to_string(), pub_pem.to_string())
}

pub fn sign_message_with_private_key_pem(private_key: &str, message: &[u8]) -> Result<Vec<u8>> {
    let private_key =
        RsaPrivateKey::from_pkcs1_pem(private_key).expect("failed to decode private key");
    let signing_key = SigningKey::<Sha256>::new(private_key);
    let mut rng = StdRng::from_entropy();

    let signature = signing_key.sign_with_rng(&mut rng, message);
    Ok(signature.to_bytes().to_vec())
}

pub fn verify_sign_with_public_key_pem(
    public_key: &str,
    message: &[u8],
    sign: &[u8],
) -> Result<()> {
    let public_key = RsaPublicKey::from_pkcs1_pem(public_key).expect("failed to decode public key");
    let verifying_key = VerifyingKey::<Sha256>::new(public_key);
    let signature = Signature::try_from(sign)?;

    verifying_key.verify(message, &signature)?;
    Ok(())
}

mod tests {
    use crate::pow;

    use super::*;

    // cargo test test_gen_private_key -- --nocapture
    // output:
    // -----BEGIN RSA PRIVATE KEY-----
    // MIIEpAIBAAKCAQEAt1mW0nKnohavlih5BGi+XXe2SB4LlmuVwVKhmmhq14SapxM3
    // /pk1N2MTHcahRlErhgC74SLBAd9ZFEbJD/zITU2H6wmAis1ZwA4H2Om6304aztaU
    // SmN7CLJC9F3hYna/Ful0pcsxhNBympBZv4xL3NG33ySY03tpro2XkEOnsjAWfEGH
    // D6zFun9NX4b+h7/iuSIHYikj5WzJXZ3wSzRIBrP9SinMeVX9/qn7n6uvypaFwCSU
    // zPmsIj/LvdhpUI3CV2m1yToAguyaqUQf54dyQv+VKzHTuM4fewFOoFv+8agpuxG6
    // n3gDILvqbviXyQUrxufsAVZDDqkYofQ2ChWwfQIDAQABAoIBAFQ9flVcxnZrk+sa
    // 11dWRLivCvohi9NlxN2Y+JT4CrbQvzmqU9zPSasUAzF4FJs5KhUcezYagLE6jDQL
    // vc9xphoWWC9+Iygi52ydRa+829ZDjX+hCWsQP/Qj6y1ZgPd7dZqyUpzmOe860pz/
    // W6ztaeGgHaoRp4HrPwgrGAr5erAcLkIaVv9vdWZkQKinDjhsk9YBxjcklHjQKqgw
    // 4rdfVzdS+dRI3gefVLt0kUi5i4LtWTFtI5prSEk4fuzWHLmmtiz/Iu8TB3vQz0Dp
    // FnVLxpC5H11ZUUmQQ+OPDGQRc19ROFMtTTbRXEcWP2YfVHt7LiMIkU+6MsBcpchp
    // d7/BJGUCgYEAx1Q8s+dnYgVxYOSwu+QRGAIJ4+p+CpQaROXIvjjbMhT3UNPfXtXM
    // m5ys1XT61gIr3ZH5zXoEJ9ydvkPDJSCbOXXYmtnColSaN2bj7MCPM2xGNXao+8gE
    // JEVFlLugDx94qd1esNL8CsfEJbtEgnt8a+5Pna8t1oB817NTYo8AmkcCgYEA63pY
    // jxfbygDKxFoOX6+sEaznYwuRmLcGXLXVFfqx/kt/iQ7cB1tphCmcQWMlDWb6UhVw
    // 2c+WypHarITHMoW8cn1aQG92XCRN2wtqsCBlUTUDOGwrVZ/0+tzhd2Y6QQHmy2KC
    // ppkNMJnDwh0/gBaJ1Rcnfdo3tNM2lQprMeRpvRsCgYAaIqhm6lN63b3U94dYy3mW
    // TUYgtTFbj7m2CO7+ShQrh+Y5md4y4BOY370lq9Xr05MO1UIMuA/tbhbcyKPArXrM
    // 2O/StfS4NiGWXA5Mj573Hh5CilFz2fD5FIAgFU3STc0TcKwmwTM88p58WAaOmURi
    // Hngf88Ut1+EGo0Ouq5NnpwKBgQDh9NGb5ZIDIYVwqtXWHbUodNjH4ucAOshBGD5z
    // y88WrA2iT/70lFQI/QCygdehJ6qWL3rNlvQkR5clKngW09vDpOOApzRVMIUA5tCm
    // Vz/Bj/QaJTnj/QzP2DoGH7NQg+maSloCUSUl+LauwvAXEgmKz6AWlNoViwN6Orgn
    // 9P099wKBgQCgjk0y7c0TwroY94+dXkqeThkL0bdf0fMgE91j0M5Kh8U3pU5z/y8N
    // S5pAUbCDR4hJTC99ncg/NxfGNEcy52badJ59E19+Z5CR5SBHTRjdSCOokYAg+YCt
    // eZn5MprHKIb3tGD+8gXaO+gUxVSAJW80MIybrD1Gwd1zCDP+A0TB2Q==
    // -----END RSA PRIVATE KEY-----

    // -----BEGIN RSA PUBLIC KEY-----
    // MIIBCgKCAQEAt1mW0nKnohavlih5BGi+XXe2SB4LlmuVwVKhmmhq14SapxM3/pk1
    // N2MTHcahRlErhgC74SLBAd9ZFEbJD/zITU2H6wmAis1ZwA4H2Om6304aztaUSmN7
    // CLJC9F3hYna/Ful0pcsxhNBympBZv4xL3NG33ySY03tpro2XkEOnsjAWfEGHD6zF
    // un9NX4b+h7/iuSIHYikj5WzJXZ3wSzRIBrP9SinMeVX9/qn7n6uvypaFwCSUzPms
    // Ij/LvdhpUI3CV2m1yToAguyaqUQf54dyQv+VKzHTuM4fewFOoFv+8agpuxG6n3gD
    // ILvqbviXyQUrxufsAVZDDqkYofQ2ChWwfQIDAQAB
    // -----END RSA PUBLIC KEY-----
    #[test]
    fn test_gen_private_key_pem() {
        let (private_key, public_key) = gen_private_key_pem();
        println!("{}\n{}", private_key, public_key);
    }

    // cargo test test_sign_and_verify -- --nocapture
    // output:
    // private key PEM:
    // -----BEGIN RSA PRIVATE KEY-----
    // MIIEowIBAAKCAQEA0MrdwLNCOFc3AFImrdjzPg7WlY9zrTeueegI7Y2HZ2faogH+
    // OVMnfe1sRS8NED0Prc88/ekhcOGFtgm6hFDjieCd1QHosoTj8ew8fxuAQlBMDomc
    // 9BU4yFAbytPnWeHTzOVyEeti7i3xVGeWH1n4K8/fJ3lJZgo0LSmEZWbcKU5m4nUp
    // cU5hx4SmYa6ul/jwMbE87iogd9ZdOsxzetncs4fI0e18V7n9ooQGFDKZALEEevpg
    // oXLQtYBQerdDI7d3IqMdFjqAieNJ4B/s7H5UjdNJPhdLX8Df1Mjgph9PrRiekTM/
    // cJ421dEOhfL8EqApIh4fR88P7qwuYmVEx0WK2wIDAQABAoIBAH/by2JHJAUme3sJ
    // 07/gPEzDf2rFFxx7HbBvhJAcfE+5jGxrdggawPNfok7XmlNYYTKZ1wrSafUbVet9
    // F6gRdNWpJF4dtickNAahGQbpi2iQjZLVeLUMDeK1E9/oViN3pGE0HN+WWtXqcn+y
    // k38NdGk3+brNjkIbe4owg3ApRidQTY5VzZwl1xu8iAxhzkoQWonPboTwsPzrURcb
    // gcEM94H2wJA+U7gtU4NC8/U0R1O1q3xfHHA5kKb7nAN8ChNK0otfz5U20fjXeY2H
    // MunUTJJpGrqxQiR1p9rskBUunVSpWGTIHXz/pF4g2ziGGTp4td+gTy0fHOVTSxoV
    // h7cX+BECgYEA5u7YUJaYbfi7BHTYIrl67GqZ+Y1gm9SRgDEzCrv25I6PmETxoEGr
    // Q+h0Fj8OZE0kKi/GNw6e/umR0BdSzSXfqizTzXVnH5W65rJFw/a+zIzDQhO2MYO8
    // oK8P1LyQM6PuwPl9Dbxo2BVHxCPdoogLU9UAtinnFXpig4AckPjbLekCgYEA53TH
    // 6SB/i8j/e8/7OfxDq6W0WF5JDLtT7hkaL2Csc94tD2GLgqO5SjppzmTapGtpmIwc
    // S08PIv0d6KE9p7fwPsTyLYOox6xuifFoicVLJCvq2fxhxZdOODDWoZ4oQtEsLkVQ
    // SO2q9FRu+c9p8MQcsV+tDkyae3gH6YcR07CdpCMCgYB4O3hCFPYNo4dzHYZ+JcWb
    // GzFJXVMkLrsGXBcwCobTnmmipJqMjkQl2fu+rVvH2uXuVOtFNQaQv3icN9hzWLmn
    // dyla+joTlrg1bjGmgmv6QXtThsG/68+kdSCv6PDHAh3HON3j7elEP2ga9XVqLpx/
    // LBHvxcc7RnnN70BwNjCfKQKBgE9M7alsHwcPqKkNCzBExBKtRWr5cuHP1OPA6f3N
    // i4hvWNTqQNhTrApIlTPHzjmDK7y+VHtg7Pi57GNlyzAJj2CSLb92Wn9/Dqhoc76w
    // QBx2h5KELCN8wany4bah731lGVQJH4a9F1N7EkK4071QE9yZwTsi99LRvzQ9uNfk
    // rT4tAoGBAJoeVQT3nwstB6WWhGh+VYPE8TUso5Krme3bUIdoe/P+pwlshxMlNuFJ
    // vDG9SDRIjsc8oV+WClkCqMKeTacxCCVxuUvq/kpe8lLcSqLoiFj0IYNM17ryD6Xe
    // bnU5fNCmJumlS+Jgmbu1frTA3Fe9LKCADDm/2I1ny+3GCHlu3lzo
    // -----END RSA PRIVATE KEY-----
    //
    // public key PEM:
    // -----BEGIN RSA PUBLIC KEY-----
    // MIIBCgKCAQEA0MrdwLNCOFc3AFImrdjzPg7WlY9zrTeueegI7Y2HZ2faogH+OVMn
    // fe1sRS8NED0Prc88/ekhcOGFtgm6hFDjieCd1QHosoTj8ew8fxuAQlBMDomc9BU4
    // yFAbytPnWeHTzOVyEeti7i3xVGeWH1n4K8/fJ3lJZgo0LSmEZWbcKU5m4nUpcU5h
    // x4SmYa6ul/jwMbE87iogd9ZdOsxzetncs4fI0e18V7n9ooQGFDKZALEEevpgoXLQ
    // tYBQerdDI7d3IqMdFjqAieNJ4B/s7H5UjdNJPhdLX8Df1Mjgph9PrRiekTM/cJ42
    // 1dEOhfL8EqApIh4fR88P7qwuYmVEx0WK2wIDAQAB
    // -----END RSA PUBLIC KEY-----
    //
    // signature: [152, 168, 170, 95, 95, 252, 3, 7, 125, 26, 245, 0, 105, 82, 60, 130, 120, 34, 24, 68, 175, 88, 12, 125, 203, 234, 255, 79, 113, 182, 141, 52, 73, 202, 254, 35, 184, 117, 46, 49, 251, 57, 131, 197, 170, 188, 149, 232, 157, 157, 38, 175, 244, 156, 236, 73, 236, 13, 88, 103, 200, 204, 116, 77, 20, 201, 141, 224, 215, 91, 146, 141, 203, 118, 204, 154, 218, 60, 103, 19, 50, 232, 164, 89, 113, 55, 4, 88, 111, 86, 154, 227, 103, 26, 163, 201, 63, 79, 45, 63, 97, 231, 41, 100, 185, 243, 100, 225, 161, 174, 94, 198, 238, 235, 227, 229, 34, 123, 124, 5, 166, 200, 127, 85, 100, 117, 44, 24, 52, 2, 5, 24, 133, 245, 181, 238, 196, 69, 32, 157, 255, 195, 37, 164, 230, 192, 200, 254, 39, 34, 250, 208, 63, 216, 227, 119, 80, 116, 161, 19, 241, 155, 108, 72, 174, 137, 45, 160, 136, 241, 152, 58, 120, 134, 228, 62, 50, 93, 60, 203, 92, 147, 132, 22, 120, 250, 222, 140, 164, 22, 75, 242, 180, 203, 97, 101, 16, 77, 233, 149, 222, 192, 206, 143, 86, 254, 181, 59, 240, 87, 240, 43, 195, 189, 88, 47, 197, 47, 41, 141, 30, 232, 129, 154, 76, 146, 35, 111, 92, 215, 206, 105, 136, 30, 128, 189, 230, 107, 1, 12, 35, 236, 95, 224, 89, 0, 155, 73, 108, 128, 23, 160, 143, 196, 137, 24]
    // verify success
    #[test]
    fn test_sign_and_verify() {
        // gen key
        let (private_key, public_key) = gen_private_key_pem();
        println!(
            "private key PEM:\n{}\npublic key PEM:\n{}",
            private_key, public_key
        );

        // calculate nonce
        let user_name = "Terry".to_string();
        let difficulty = "0000".to_string();

        let result = pow::pow(user_name.clone(), difficulty).unwrap();
        let name_nonce_str = result.user_name.clone() + &result.nonce.to_string();

        let sign_result =
            sign_message_with_private_key_pem(&private_key, name_nonce_str.as_bytes());
        assert!(sign_result.is_ok());
        let sign = sign_result.unwrap();
        println!("signature: {:?}", sign);
        let verify_result =
            verify_sign_with_public_key_pem(&public_key, name_nonce_str.as_bytes(), &sign);
        assert!(verify_result.is_ok());
        println!("verify success");
    }
}
