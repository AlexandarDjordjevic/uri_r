#[allow(unused_variables)]
#[allow(dead_code)]
pub struct Authority {
    user_info: Option<String>,
    host: String,
    port: Option<u16>,
}

#[allow(unused_variables)]
#[allow(dead_code)]
impl Authority{
    fn get_authority_from_uri(string: &str) -> Option<String> {
        let start_index = string.find("//");
        match start_index {
            Some(start_index) => {
                let auth = &string[start_index + 2 ..];
                let end_index = auth.find("/").unwrap_or(
                    auth.find("#").unwrap_or(
                        auth.find("?").unwrap_or(auth.len())
                    )
                );
                Some(auth[..end_index].to_string())
            },
            None => None
        }
    }

    fn parse_user_info(authority_str: &String) -> Option<String>{
        let tokens : Vec<&str> = authority_str.split("@").collect();
        if tokens.len() == 2 {
            Some(tokens[0].to_string())
        } else {
            None
        }
    }

    pub fn from_string(uri: &String) -> (Option<Authority>, &str) {
        let mut authority= Authority {
            user_info: None,
            host: "".to_string(),
            port: None
        };
        let str_auth = Authority::get_authority_from_uri(uri);
        match &str_auth{
            None => {
                (None, uri)
            },
            Some(auth) => {
                println!("Authority string: {}", &auth);
                authority.user_info = Authority::parse_user_info(auth);
                (Some(authority), uri)
            }
        }
    }



    pub fn get_host(&self) -> &str {
        &self.host
    }

    pub fn get_port(&self) -> &Option<u16> {
        &self.port
    }

    pub fn get_user_info(&self) -> &Option<String> {
        &self.user_info
    }
}

#[test]
fn get_authority_from_uri(){
    use super::*;
    let test_cases = vec![
        ("//foo:bar@www.example.com/", Some("foo:bar@www.example.com".to_string())),
        ("//www.example.com/a:b", Some("www.example.com".to_string())),
        ("//www.example.com/foo?a:b", Some("www.example.com".to_string())),
        ("//www.example.com/foo#a:b", Some("www.example.com".to_string())),
        ("//[v7.:]/", Some("[v7.:]".to_string())),
        ("/:/foo", None),
        ("/home/example", None),
    ];
    for test_case in test_cases.iter() {
        let (uri, authority) = test_case;
        println!("Authority: {:?}", Authority::get_authority_from_uri(uri));
        assert_eq!(*authority, Authority::get_authority_from_uri(uri));

    }
}