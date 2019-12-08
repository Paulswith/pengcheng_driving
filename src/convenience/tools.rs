/*
create at 2019/12/4 by itachy
*/
use std::collections::HashMap;


/// flat multi params to 'a=1&b=2'
pub fn flat_to_url_query_param(params: &HashMap<&str, &str>) -> String {
    let flat_param_vec: Vec<_> = params.into_iter()
        .map(|(k, v)|
            format!("{}={}", k, v))
        .collect();
    flat_param_vec.join("&")
}
