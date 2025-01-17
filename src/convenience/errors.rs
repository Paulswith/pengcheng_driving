/*
  create at 2019/12/4 by 'itachy'
*/
error_chain! {
    foreign_links {
        IoError(::std::io::Error);
        JsonError(serde_json::error::Error);
        ReqwestError(reqwest::Error);
        TomlParserError(toml::de::Error);
    }

    errors {
        InvalidWrapResponse(wrap_rsp: String) {
            description("invalid wrap response"),
            display("invalid wrap response, raw rsp content: '{}'", wrap_rsp),
        }
        XmlParseFailed(content: String) {
            description("Xml parse failed"),
            display("Xml parse failed from str: {}", content),
        }
    }
}

