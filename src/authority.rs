use regex::Regex;

/// User information URI component.  <br />
/// May consist of a user name and, optionally, scheme-specific information 
/// about how to gain authorization to access the resource
#[allow(unused_variables)]
#[allow(dead_code)]
pub struct UserInfo{
    username: String,
    password: Option<String>
}

#[allow(unused_variables)]
#[allow(dead_code)]
impl UserInfo {
    fn from_string(string: &str) -> Self{
        let regex = Regex::new(r"[:]").unwrap();
        let tokens: Vec<&str> = regex.split(string).collect();
        if tokens.len() == 2 {
            UserInfo {
                username: tokens[0].to_string(),
                password: Some(tokens[1].to_string())
            }
        } else {
            UserInfo {
                username: tokens[0].to_string(),
                password: None
            }
        }
    }

    /// Method returns username <br />
    /// - ***return \[ &str \]***: Username string reference
    fn get_username(&self) -> &str {
        self.username.as_ref()
    }

    /// Method returns password <br />
    /// - ***return \[ Option\<&String\> \]***: Password if it is available, otherwise **None**
    fn get_password(&self) -> Option<&String>{
        match self.password.as_ref() {
            None => None,
            Some(password) => Some(password)
        }
    }
}

pub struct Authority {
    user_info: Option<UserInfo>,
    host: String,
    port: Option<u16>,
}

#[allow(unused_variables)]
#[allow(dead_code)]
impl Authority{
    /// Constructor for Authority object
    /// - ***return \[Authority\]*** New, empty object 
    pub fn new() -> Self {
        Authority{
            user_info: None,
            host: String::new(),
            port: None
        }
    }

    /// This method is used to parse URI authority from string
    /// - ***param \[ &str ]***: authority string reference meant for parsing 
    /// - ***return \[ Option\<Authority\>\]***: Authority, if it is part of the string, otherwise ***None***
    pub fn from_string(auth_string: &str) -> Option<Authority> {
        let mut authority= Authority::new();
        

        if auth_string.is_empty() {
            return None;
        }

        //Convert to lower case string
        let auth_lc = &auth_string.to_lowercase();

        //Create regex
        let regex = Regex::new(r"[@]").unwrap();
        let tokens: Vec<&str> = regex.split(auth_lc).collect();

        match tokens.len() {
            1 => authority.host = tokens[0].to_string(),
            2 => {
                authority.user_info = Some(UserInfo::from_string(tokens[0]));
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
    
    /// This method is used to get authority host  
    /// - ***return \[&str\]***: Reference to the host string
    pub fn get_host(&self) -> &str {
        &self.host
    }

    /// This method is used to get authority port number
    /// - ***return \[ Option\<u16\>\]***: port number if authority string contains port, otherwise **None**
    pub fn get_port(&self) -> Option<u16> {
       match self.port {
           None => None,
           Some(port) => Some(port)
       }    
    }

    /// This method is used to get authority user info  
    /// - ***return \[Option\<&UserInfo\>\]***: If user info is available returns &UserInfo, otherwise ***None***
    pub fn get_user_info(&self) -> Option<&UserInfo> {
        match self.user_info.as_ref() {
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
    assert_eq!(authority.as_ref().unwrap().get_user_info().unwrap().get_username(), "foo");
    assert_eq!(authority.as_ref().unwrap().get_user_info().unwrap().get_password().unwrap(), "bar");
    assert_eq!(authority.as_ref().unwrap().get_port(), None);

    let authority = Authority::from_string( "john.doe@192.168.1.1:123");

    assert_eq!(authority.as_ref().unwrap().get_user_info().unwrap().get_username(), "john.doe");
    assert_eq!(authority.as_ref().unwrap().get_user_info().unwrap().get_password(), None);
    assert_eq!(authority.as_ref().unwrap().get_host(), "192.168.1.1");
    assert_eq!(authority.as_ref().unwrap().get_port().unwrap(), 123);

    let authority = Authority::from_string("");

    assert!(authority.is_none());
}