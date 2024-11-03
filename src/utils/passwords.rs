use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2, Params, PasswordHash, PasswordVerifier,
};

#[derive(Debug, Clone)]
pub struct PasswordManager<'x> {
    argon2: Argon2<'x>,
}

impl PasswordManager<'_> {
    pub fn new() -> Self {
        // Recommended argon2id parameters from https://cheatsheetseries.owasp.org/cheatsheets/Password_Storage_Cheat_Sheet.html#argon2id
        let params = Params::new(12288, 3, 1, None).unwrap();

        let argon2 = Argon2::new(argon2::Algorithm::Argon2id, argon2::Version::V0x13, params);

        PasswordManager { argon2 }
    }

    pub fn hash(&self, password: &String, pepper: &String) -> String {
        let peppered_password = format!("{password}{pepper}");
        let salt = SaltString::generate(&mut OsRng);
        let hash = self
            .argon2
            .hash_password(peppered_password.as_bytes(), &salt)
            .unwrap()
            .to_string();

        hash
    }

    pub fn compare(&self, password: &String, pepper: &String, password_hash: &String) -> bool {
        let parsed_hash = PasswordHash::new(password_hash).unwrap();
        let result = self
            .argon2
            .verify_password(format!("{password}{pepper}").as_bytes(), &parsed_hash);

        if result.is_ok() {
            return true;
        } else {
            return false;
        }
    }
}
