// use super::{
//     authority::Authority
// };

// #[allow(unused_variables)]
// #[allow(dead_code)]
// pub struct Uri {
//     scheme: Option<String>,
//     authority: Option<Authority>,
//     path: String,
// }

// #[allow(unused_variables)]
// #[allow(dead_code)]
// impl Uri {
//     fn parse_scheme(uri: String) -> (Option<String>, String) {
//         let authority_or_path_delimiter_position = uri.find('/');
//         let colon_after_scheme_position = uri.find(':');

//         match colon_after_scheme_position {
//             None => (None, uri),
//             Some(colon_index) => match authority_or_path_delimiter_position {
//                 None => (
//                     Some(uri[..colon_index].to_string()),
//                     uri[colon_index..].to_string(),
//                 ),
//                 Some(index) => {
//                     if index > colon_index {
//                         (
//                             Some(uri[..colon_index].to_string()),
//                             uri[colon_index..].to_string(),
//                         )
//                     } else {
//                         (None, uri)
//                     }
//                 }
//             },
//         }
//     }

//     pub fn from_string(uri: String) -> Result<Uri, String> {
//         let mut _uri = Uri {
//             scheme: None,
//             authority: None,
//             path: uri.to_string(),
//         };

//         let (scheme, uri) = Uri::parse_scheme(uri);
//         _uri.scheme = scheme;

//         let (auth, uri) = Authority::from_string(&uri);
//         _uri.authority = auth;
//         match &_uri.authority {
//             Some(authority) => {
//                 println!("Authority host: {}", authority.get_host());
//                 println!("Authority user info: {:?}", authority.get_user_info());
//             },
//             None => ()
//         }
//         Ok(_uri)
//     }

//     pub fn get_scheme(&self) -> Option<&String> {
//         match &self.scheme {
//             None => None,
//             Some(scheme) => Some(scheme),
//         }
//     }

//     pub fn get_path(&self) -> &String {
//         &self.path
//     }
// }
