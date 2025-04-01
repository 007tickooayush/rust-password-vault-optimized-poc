use crate::vault::Vault;

// TODO: Provide a trait and struct to define and implement the functions based on the storage type

pub fn search_credentials(search_term: &str) -> Result<Vec<Vault>, std::io::Error> {
    unimplemented!()
}

pub fn read_complete_passwords_file<'vl>() -> Result<Vec<Vault<'vl>>, std::io::Error> {
    unimplemented!()
}