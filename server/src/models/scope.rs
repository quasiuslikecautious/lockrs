use diesel::prelude::*;

use crate::db;

pub struct ScopeRequest;

impl ScopeRequest {
    pub fn get_validated_scopes(requested_scopes: &str) -> Option<Scopes> {
        use db::schema::scopes;
        let scopes_list = requested_scopes.split(' ').map(|s| s.to_string()).collect::<Vec<String>>();
    
        let connection = &mut db::establish_connection();
        let Some(validated_scopes) = connection.build_transaction()
            .read_only()
            .run(|conn| {
                scopes::table
                    .select(scopes::name)
                    .filter(scopes::name.eq_any(&scopes_list))
                    .load(conn)
            }).ok()
        else {
            return None;
        };

        return Some(Scopes {
            scopes: validated_scopes,
        });
    }
}

#[derive(Debug)]
pub struct Scopes {
    scopes: Vec<String>,
}

impl Scopes {
    pub fn new(scopes: Vec<String>) -> Self {
        Self {
            scopes,
        }
    }

    pub fn get(&self) -> &Vec<String> {
        return &self.scopes;
    }

    pub fn into_scope_string(&self) -> String {
        return self.scopes.join(" ").to_string();
    }
}

