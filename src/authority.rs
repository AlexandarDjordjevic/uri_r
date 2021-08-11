use regex::Regex;
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
    pub fn new() -> Self {
        Authority{
            user_info: None,
            host: "".to_string(),
            port: None
        }
    }

    //Public functions ------------------------------

    pub fn from_string(auth_string: &str) -> Option<Authority> {
        let mut authority= Authority {
            user_info: None,
            host: "".to_string(),
            port: None
        };
        //Convert to lower case
        let auth_lc = &auth_string.to_lowercase();

        //Create regex
        let regex = Regex::new(r"[@]").unwrap();
        let tokens: Vec<&str> = regex.split(auth_lc).collect();

        match tokens.len() {
            1 => authority.host = tokens[0].to_string(),
            2 => {
                authority.user_info = Some(tokens[0].to_string());
                let regex = Regex::new(r"[:]").unwrap();
                let host_port_tokens: Vec<&str> = regex.split(tokens[1]).collect();
                match host_port_tokens.len() {
                    1 => {
                        authority.host = host_port_tokens[0].to_string();
                    },
                    2 => {
                        authority.host = host_port_tokens[0].to_string();
                        match host_port_tokens[1].parse::<u16>() {
                            Ok(port) => {
                                authority.port = Some(port);
                            },
                            Err(_) => ()
                        }
                    },
                    _ => ()
                }
            },
            _ => ()
        }
        Some(authority)
    }

    pub fn get_host(&self) -> &str {
        &self.host
    }

    pub fn get_port(&self) -> Option<&u16> {
       match &self.port {
           None => None,
           Some(port) => Some(port)
       }
    }

    pub fn get_user_info(&self) -> Option<&String> {
        match &self.user_info {
            None => None,
            Some(user_info) => Some(user_info)
        }
    }
}

#[test]
fn test_authority_parser(){
    use super::*;

    let authority = Authority::from_string("foo:bar@www.example.com");
    assert_eq!(authority.as_ref().unwrap().get_host(), "www.example.com");
    assert_eq!(authority.as_ref().unwrap().get_user_info().unwrap(), "foo:bar");
    assert_eq!(authority.as_ref().unwrap().get_port(), None);

    let authority = Authority::from_string( "john.doe@192.168.1.1:123");

    assert_eq!(authority.as_ref().unwrap().get_user_info().unwrap(), "john.doe");
    assert_eq!(authority.as_ref().unwrap().get_host(), "192.168.1.1");
    assert_eq!(*authority.as_ref().unwrap().get_port().unwrap(), 123);

}