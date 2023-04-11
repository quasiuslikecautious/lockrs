pub struct ScopeMapper;

impl ScopeMapper {
    pub fn db_list_to_vec(scopes: &[Option<String>]) -> Vec<String> {
        scopes.into_iter()
            .filter_map(|s| {
                match s {
                    Some(s) => Some(s.to_owned()),
                    None => None,
                }
            })
            .collect::<Vec<String>>()
    }
}

