//
// Copyright (C) 2020 Signal Messenger, LLC.
// All rights reserved.
//
// SPDX-License-Identifier: GPL-3.0-only
//

#![allow(non_snake_case)]

use crate::common::sho::*;
use crate::common::simple_types::*;
use crate::crypto::profile_key_credential_request;
use crate::crypto::uid_struct;
use curve25519_dalek::constants::RISTRETTO_BASEPOINT_POINT;
use curve25519_dalek::ristretto::RistrettoPoint;
use curve25519_dalek::scalar::Scalar;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SystemParams {
    pub(crate) G_w: RistrettoPoint,
    pub(crate) G_wprime: RistrettoPoint,
    pub(crate) G_x0: RistrettoPoint,
    pub(crate) G_x1: RistrettoPoint,
    pub(crate) G_y1: RistrettoPoint,
    pub(crate) G_y2: RistrettoPoint,
    pub(crate) G_y3: RistrettoPoint,
    pub(crate) G_y4: RistrettoPoint,
    pub(crate) G_m1: RistrettoPoint,
    pub(crate) G_m2: RistrettoPoint,
    pub(crate) G_m3: RistrettoPoint,
    pub(crate) G_m4: RistrettoPoint,
    pub(crate) G_V: RistrettoPoint,
    pub(crate) G_z: RistrettoPoint,
}

#[derive(Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct KeyPair {
    // private
    pub(crate) w: Scalar,
    pub(crate) wprime: Scalar,
    pub(crate) W: RistrettoPoint,
    pub(crate) x0: Scalar,
    pub(crate) x1: Scalar,
    pub(crate) y1: Scalar,
    pub(crate) y2: Scalar,
    pub(crate) y3: Scalar,
    pub(crate) y4: Scalar,

    // public
    pub(crate) C_W: RistrettoPoint,
    pub(crate) I: RistrettoPoint,
}

#[derive(Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct PublicKey {
    pub(crate) C_W: RistrettoPoint,
    pub(crate) I: RistrettoPoint,
}

#[derive(Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuthCredential {
    pub(crate) t: Scalar,
    pub(crate) U: RistrettoPoint,
    pub(crate) V: RistrettoPoint,
}

#[derive(Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProfileKeyCredential {
    pub(crate) t: Scalar,
    pub(crate) U: RistrettoPoint,
    pub(crate) V: RistrettoPoint,
}

#[derive(Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct BlindedProfileKeyCredentialWithSecretNonce {
    pub(crate) rprime: Scalar,
    pub(crate) t: Scalar,
    pub(crate) U: RistrettoPoint,
    pub(crate) S1: RistrettoPoint,
    pub(crate) S2: RistrettoPoint,
}

#[derive(Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct BlindedProfileKeyCredential {
    pub(crate) t: Scalar,
    pub(crate) U: RistrettoPoint,
    pub(crate) S1: RistrettoPoint,
    pub(crate) S2: RistrettoPoint,
}

pub(crate) fn convert_to_points_uid_struct(
    uid: uid_struct::UidStruct,
    redemption_time: RedemptionTime,
) -> Vec<RistrettoPoint> {
    let system = SystemParams::get_hardcoded();
    let redemption_time_scalar = encode_redemption_time(redemption_time);
    vec![uid.M1, uid.M2, redemption_time_scalar * system.G_m3]
}

impl SystemParams {
    pub fn generate() -> Self {
        let mut sho = Sho::new(
            b"Signal_ZKGroup_20200424_Constant_Credentials_SystemParams_Generate",
            b"",
        );
        let G_w = sho.get_point();
        let G_wprime = sho.get_point();

        let G_x0 = sho.get_point();
        let G_x1 = sho.get_point();

        let G_y1 = sho.get_point();
        let G_y2 = sho.get_point();
        let G_y3 = sho.get_point();
        let G_y4 = sho.get_point();

        let G_m1 = sho.get_point();
        let G_m2 = sho.get_point();
        let G_m3 = sho.get_point();
        let G_m4 = sho.get_point();

        let G_V = sho.get_point();
        let G_z = sho.get_point();

        SystemParams {
            G_w,
            G_wprime,
            G_x0,
            G_x1,
            G_y1,
            G_y2,
            G_y3,
            G_y4,
            G_m1,
            G_m2,
            G_m3,
            G_m4,
            G_V,
            G_z,
        }
    }

    pub fn get_hardcoded() -> SystemParams {
        bincode::deserialize::<SystemParams>(&SystemParams::SYSTEM_HARDCODED).unwrap()
    }

    const SYSTEM_HARDCODED: [u8; 448] = [
        0x9a, 0xe7, 0xc8, 0xe5, 0xed, 0x77, 0x9b, 0x11, 0x4a, 0xe7, 0x70, 0x8a, 0xa2, 0xf7, 0x94,
        0x67, 0xa, 0xdd, 0xa3, 0x24, 0x98, 0x7b, 0x65, 0x99, 0x13, 0x12, 0x2c, 0x35, 0x50, 0x5b,
        0x10, 0x5e, 0x6c, 0xa3, 0x10, 0x25, 0xd2, 0xd7, 0x6b, 0xe7, 0xfd, 0x34, 0x94, 0x4f, 0x98,
        0xf7, 0xfa, 0xe, 0x37, 0xba, 0xbb, 0x2c, 0x8b, 0x98, 0xbb, 0xbd, 0xbd, 0x3d, 0xd1, 0xbf,
        0x13, 0xc, 0xca, 0x2c, 0x8a, 0x9a, 0x3b, 0xdf, 0xaa, 0xa2, 0xb6, 0xb3, 0x22, 0xd4, 0x6b,
        0x93, 0xec, 0xa7, 0xb0, 0xd5, 0x1c, 0x86, 0xa3, 0xc8, 0x39, 0xe1, 0x14, 0x66, 0x35, 0x82,
        0x58, 0xa6, 0xc1, 0xc, 0x57, 0x7f, 0xc2, 0xbf, 0xfd, 0x34, 0xcd, 0x99, 0x16, 0x4c, 0x9a,
        0x6c, 0xd2, 0x9f, 0xab, 0x55, 0xd9, 0x1f, 0xf9, 0x26, 0x93, 0x22, 0xec, 0x34, 0x58, 0x60,
        0x3c, 0xc9, 0x6a, 0xd, 0x47, 0xf7, 0x4, 0x5, 0x82, 0x88, 0xf6, 0x2e, 0xe0, 0xac, 0xed,
        0xb8, 0xaa, 0x23, 0x24, 0x21, 0x21, 0xd9, 0x89, 0x65, 0xa9, 0xbb, 0x29, 0x91, 0x25, 0xc,
        0x11, 0x75, 0x80, 0x95, 0xec, 0xe0, 0xfd, 0x2b, 0x33, 0x28, 0x52, 0x86, 0xfe, 0x1f, 0xcb,
        0x5, 0x61, 0x3, 0xb6, 0x8, 0x17, 0x44, 0xb9, 0x75, 0xf5, 0x50, 0xd0, 0x85, 0x21, 0x56,
        0x8d, 0xd3, 0xd8, 0x61, 0x8f, 0x25, 0xc1, 0x40, 0x37, 0x5a, 0xf, 0x40, 0x24, 0xc3, 0xaa,
        0x23, 0xbd, 0xff, 0xfb, 0x27, 0xfb, 0xd9, 0x82, 0x20, 0x8d, 0x3e, 0xcd, 0x1f, 0xd3, 0xbc,
        0xb7, 0xac, 0xc, 0x3a, 0x14, 0xb1, 0x9, 0x80, 0x4f, 0xc7, 0x48, 0xd7, 0xfa, 0x45, 0x6c,
        0xff, 0xb4, 0x93, 0x4f, 0x98, 0xb, 0x6e, 0x9, 0xa2, 0x48, 0xa6, 0xf, 0x44, 0xa6, 0x15, 0xa,
        0xe6, 0xc1, 0x3d, 0x7e, 0x3c, 0x6, 0x26, 0x1d, 0x7e, 0x4e, 0xed, 0x37, 0xf3, 0x9f, 0x60,
        0xcc, 0x60, 0x37, 0xdc, 0x31, 0xc2, 0xe8, 0xd4, 0x47, 0x4f, 0xb5, 0x19, 0x58, 0x7a, 0x44,
        0x86, 0x93, 0x18, 0x2a, 0xd9, 0xd6, 0xd8, 0x6b, 0x53, 0x59, 0x57, 0x85, 0x8f, 0x54, 0x7b,
        0x93, 0x40, 0x12, 0x7d, 0xa7, 0x5f, 0x80, 0x74, 0xca, 0xee, 0x94, 0x4a, 0xc3, 0x6c, 0xa,
        0xc6, 0x62, 0xd3, 0x8c, 0x9b, 0x3c, 0xcc, 0xe0, 0x3a, 0x9, 0x3f, 0xcd, 0x96, 0x44, 0x4,
        0x73, 0x98, 0xb8, 0x6b, 0x6e, 0x83, 0x37, 0x2f, 0xf1, 0x4f, 0xb8, 0xbb, 0xd, 0xea, 0x65,
        0x53, 0x12, 0x52, 0xac, 0x70, 0xd5, 0x8a, 0x4a, 0x8, 0x10, 0xd6, 0x82, 0xa0, 0xe7, 0x9,
        0xc9, 0x22, 0x7b, 0x30, 0xef, 0x6c, 0x8e, 0x17, 0xc5, 0x91, 0x5d, 0x52, 0x72, 0x21, 0xbb,
        0x0, 0xda, 0x81, 0x75, 0xcd, 0x64, 0x89, 0xaa, 0x8a, 0xa4, 0x92, 0xa5, 0x0, 0xf9, 0xab,
        0xee, 0x56, 0x90, 0xb9, 0xdf, 0xca, 0x88, 0x55, 0x4, 0xb6, 0x16, 0xc7, 0x6, 0xc8, 0xc,
        0x75, 0x6c, 0x11, 0xa3, 0x1, 0x6b, 0xbf, 0xb6, 0x9, 0x77, 0xf4, 0x64, 0x8b, 0x5f, 0x23,
        0x95, 0xa4, 0xb4, 0x28, 0xb7, 0x21, 0x19, 0x40, 0x81, 0x3e, 0x3a, 0xfd, 0xe2, 0xb8, 0x7a,
        0xa9, 0xc2, 0xc3, 0x7b, 0xf7, 0x16, 0xe2, 0x57, 0x8f, 0x95, 0x65, 0x6d, 0xf1, 0x2c, 0x2f,
        0xb6, 0xf5, 0xd0, 0x63, 0x1f, 0x6f, 0x71, 0xe2, 0xc3, 0x19, 0x3f, 0x6d,
    ];
}

impl KeyPair {
    pub fn generate(sho: &mut Sho, num_attributes: usize) -> Self {
        if !(3..=4).contains(&num_attributes) {
            panic!();
        }

        let system = SystemParams::get_hardcoded();
        let w = sho.get_scalar();
        let W = w * system.G_w;
        let wprime = sho.get_scalar();
        let x0 = sho.get_scalar();
        let x1 = sho.get_scalar();
        let y1 = sho.get_scalar();
        let y2 = sho.get_scalar();
        let y3 = sho.get_scalar();
        let y4 = sho.get_scalar();

        let C_W = (w * system.G_w) + (wprime * system.G_wprime);
        let mut I = system.G_V
            - (x0 * system.G_x0)
            - (x1 * system.G_x1)
            - (y1 * system.G_y1)
            - (y2 * system.G_y2)
            - (y3 * system.G_y3);

        if num_attributes > 3 {
            I -= y4 * system.G_y4;
        }

        KeyPair {
            w,
            wprime,
            W,
            x0,
            x1,
            y1,
            y2,
            y3,
            y4,
            C_W,
            I,
        }
    }

    pub fn get_public_key(&self) -> PublicKey {
        PublicKey {
            C_W: self.C_W,
            I: self.I,
        }
    }

    pub fn create_auth_credential(
        &self,
        uid: uid_struct::UidStruct,
        redemption_time: RedemptionTime,
        sho: &mut Sho,
    ) -> AuthCredential {
        let M = convert_to_points_uid_struct(uid, redemption_time);
        let (t, U, V) = self.credential_core(M, sho);
        AuthCredential { t, U, V }
    }

    fn credential_core(
        &self,
        M: Vec<RistrettoPoint>,
        sho: &mut Sho,
    ) -> (Scalar, RistrettoPoint, RistrettoPoint) {
        if M.len() > 4 {
            panic!();
        }
        let t = sho.get_scalar();
        let U = sho.get_point();

        let mut V = self.W + (self.x0 + self.x1 * t) * U;
        V += self.y1 * M[0];
        V += self.y2 * M[1];
        if M.len() > 2 {
            V += self.y3 * M[2];
        }
        if M.len() > 3 {
            V += self.y4 * M[3];
        }
        (t, U, V)
    }

    pub fn create_blinded_profile_key_credential(
        &self,
        uid: uid_struct::UidStruct,
        public_key: profile_key_credential_request::PublicKey,
        ciphertext: profile_key_credential_request::Ciphertext,
        sho: &mut Sho,
    ) -> BlindedProfileKeyCredentialWithSecretNonce {
        let M = vec![uid.M1, uid.M2];

        let (t, U, Vprime) = self.credential_core(M, sho);
        let rprime = sho.get_scalar();
        let R1 = rprime * RISTRETTO_BASEPOINT_POINT;
        let R2 = rprime * public_key.Y + Vprime;
        let S1 = R1 + (self.y3 * ciphertext.D1) + (self.y4 * ciphertext.E1);
        let S2 = R2 + (self.y3 * ciphertext.D2) + (self.y4 * ciphertext.E2);
        BlindedProfileKeyCredentialWithSecretNonce {
            rprime,
            t,
            U,
            S1,
            S2,
        }
    }
}

impl BlindedProfileKeyCredentialWithSecretNonce {
    pub fn get_blinded_profile_key_credential(&self) -> BlindedProfileKeyCredential {
        BlindedProfileKeyCredential {
            t: self.t,
            U: self.U,
            S1: self.S1,
            S2: self.S2,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::constants::*;
    use crate::crypto::proofs;

    #[test]
    fn test_system() {
        let params = SystemParams::generate();
        println!("PARAMS = {:#x?}", bincode::serialize(&params));
        assert!(SystemParams::generate() == SystemParams::get_hardcoded());
    }

    #[test]
    fn test_mac() {
        let mut sho = Sho::new(b"Test_Credentials", b"");
        let keypair = KeyPair::generate(&mut sho, NUM_AUTH_CRED_ATTRIBUTES);

        let uid_bytes = TEST_ARRAY_16;
        let redemption_time = 37;
        let uid = uid_struct::UidStruct::new(uid_bytes);
        let credential = keypair.create_auth_credential(uid, redemption_time, &mut sho);
        let proof = proofs::AuthCredentialIssuanceProof::new(
            keypair,
            credential,
            uid,
            redemption_time,
            &mut sho,
        );

        let public_key = keypair.get_public_key();
        proof
            .verify(public_key, credential, uid, redemption_time)
            .unwrap();

        let keypair_bytes = bincode::serialize(&keypair).unwrap();
        let keypair2 = bincode::deserialize(&keypair_bytes).unwrap();
        assert!(keypair == keypair2);

        let public_key_bytes = bincode::serialize(&public_key).unwrap();
        let public_key2 = bincode::deserialize(&public_key_bytes).unwrap();
        assert!(public_key == public_key2);

        let mac_bytes = bincode::serialize(&credential).unwrap();

        println!("mac_bytes = {:#x?}", mac_bytes);
        assert!(
            mac_bytes
                == vec![
                    0xe0, 0xce, 0x21, 0xfe, 0xb7, 0xc3, 0xb8, 0x62, 0x3a, 0xe6, 0x20, 0xab, 0x3e,
                    0xe6, 0x5d, 0x94, 0xa3, 0xf3, 0x40, 0x53, 0x31, 0x63, 0xd2, 0x4c, 0x5d, 0x41,
                    0xa0, 0xd6, 0x7a, 0x40, 0xb3, 0x2, 0x8e, 0x50, 0xa2, 0x7b, 0xd4, 0xda, 0xe9,
                    0x9d, 0x60, 0x0, 0xdb, 0x97, 0x3d, 0xbc, 0xc5, 0xad, 0xe1, 0x32, 0xbc, 0x56,
                    0xb0, 0xe1, 0xac, 0x16, 0x7b, 0xb, 0x2c, 0x9, 0xe2, 0xb6, 0xc8, 0x5b, 0x68,
                    0xc8, 0x8e, 0x7d, 0xfd, 0x58, 0x97, 0x51, 0xe9, 0x8, 0x1f, 0x81, 0xb0, 0x24,
                    0xea, 0xa0, 0xaf, 0x29, 0x6, 0xed, 0xb3, 0x9, 0x32, 0xed, 0x65, 0x28, 0x2f,
                    0xa1, 0x79, 0x9e, 0x1, 0x24,
                ]
        );
    }
}
