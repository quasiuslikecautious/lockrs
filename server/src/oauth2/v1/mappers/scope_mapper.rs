pub struct ScopeMapper;

impl ScopeMapper {
    pub fn pg_list_to_vec(scopes: &[Option<String>]) -> Vec<String> {
        scopes
            .iter()
            .filter_map(|s| s.as_ref().map(|s| s.to_owned()))
            .collect::<Vec<String>>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_map_pg() {
        let pg_scopes = vec![
            Some(String::from("read")),
            Some(String::from("write")),
            Some(String::from("offline_access")),
        ];

        let actual_scopes = ScopeMapper::pg_list_to_vec(&pg_scopes);

        let expected_scopes = vec!["read", "write", "offline_access"];

        assert_eq!(actual_scopes, expected_scopes);
    }
}
