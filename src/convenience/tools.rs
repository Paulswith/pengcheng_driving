/*
create at 2019/12/4 by itachy
*/
use std::collections::HashMap;
use rand::Rng;
use super::errors;


/// nonce
pub fn generate_nonce() -> String {
    let mut t = rand::thread_rng();
    format!("{}", t.gen_range(100_000_000, 1_000_000_000 - 1))
}

/// flat multi params to 'a=1&b=2'
pub fn flat_to_url_query_param(params: &HashMap<&str, &str>) -> String {
    let flat_param_vec: Vec<_> = params.into_iter()
        .map(|(k, v)|
            format!("{}={}", k, v))
        .collect();
    flat_param_vec.join("&")
}

/// parse wrap
pub fn unwrap_body(wrap_content: &str) -> errors::Result<String> {
    match dummy_xml::parser::parse_str(wrap_content) {
        Ok(ref document) if document.root().first_child().is_some() => {
            let child = document.root().first_child().unwrap();
            Ok(child.value().to_string())
        }
        _ => bail!(errors::ErrorKind::XmlParseFailed(format!("{:?}", wrap_content))),
    }
}
