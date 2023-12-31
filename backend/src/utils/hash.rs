use bcrypt::{hash, verify, BcryptResult, DEFAULT_COST};

pub struct BcryptHash;

impl BcryptHash {
    pub fn hash_string<S>(str: S) -> BcryptResult<String>
    where
        S: Into<String>,
    {
        let plain_text: String = str.into();
        hash(plain_text, DEFAULT_COST)
    }

    pub fn verify_hash<'a, S, T>(hash: S, str: T) -> BcryptResult<bool>
    where
        S: Into<&'a String>,
        T: Into<String>,
    {
        let ref_str: String = str.into();
        let hash: &String = hash.into();

        verify(ref_str.as_bytes(), hash)
    }
}
