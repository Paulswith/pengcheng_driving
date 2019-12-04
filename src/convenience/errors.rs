/*
  create at 2019/12/4 by 'itachy'
*/

pub mod convert {
    error_chain! {
        foreign_links {
            IoError(::std::io::Error);
//            CommomError(::std::error::Error);
            JsonError(serde_json::error::Error);
        }
//
//        errors {
//            DeserializeError(v: String) {
//
//            }
//        }
    }
}