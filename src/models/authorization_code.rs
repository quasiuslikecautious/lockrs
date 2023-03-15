use serde::Serialize;

/// The authorization grant code supplied in the authorization grant step of the auth flow
#[derive(Debug, Serialize)]
pub struct Code {
    pub code: String,
}

impl Code {
    pub fn new(client_id: &String, requested_scopes: String)-> Self {
        Self {
            code: Self::generate(&client_id, requested_scopes),
        }
    }

    /// TODO
    /// Connect to database, and generate some code based off of params provided
    pub fn generate(client_id: &String, requested_scopes: String) -> String {
        let code = format!("{}:{:?}", &client_id, &requested_scopes);
        code
    }

    /// TODO
    /// Decrypt/Verify (and remove from db if necessary) provided code
    pub fn verify(code: String) -> bool {
        return true;
    }
}
