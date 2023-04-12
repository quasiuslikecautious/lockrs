pub struct ScopeMapper;

impl ScopeMapper {
    pub fn db_list_to_vec(scopes: &[Option<String>]) -> Vec<String> {
        scopes
            .iter()
            .filter_map(|s| s.as_ref().map(|s| s.to_owned()))
            .collect::<Vec<String>>()
    }
}
