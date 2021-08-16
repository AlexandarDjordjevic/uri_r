use super::authority::Authority;

#[allow(unused_variables)]
#[allow(dead_code)]
pub struct Uri {
    scheme: Option<String>,
    authority: Option<Authority>,
}

#[allow(unused_variables)]
#[allow(dead_code)]
impl Uri {
    fn parse_scheme(uri: &str) -> Option<String> {
        let authority_or_path_delimiter_position = uri.find('/');
        let colon_after_scheme_position = uri.find(':');

        match colon_after_scheme_position {
            None => None,
            Some(colon_index) => match authority_or_path_delimiter_position {
                None => Some(uri[..colon_index].to_string()),
                Some(index) => {
                    if index > colon_index {
                        Some(uri[..colon_index].to_string())
                    } else {
                        None
                    }
                }
            },
        }
    }

    pub fn from_string(uri: &str) -> Result<Uri, String> {
        let mut _uri = Uri {
            scheme: None,
            authority: None,
        };

        let scheme = Uri::parse_scheme(uri);
        _uri.scheme = scheme;

        let auth = Authority::from_string(uri);
        _uri.authority = auth;
        Ok(_uri)
    }

    pub fn get_scheme(&self) -> Option<&String> {
        match &self.scheme {
            None => None,
            Some(scheme) => Some(scheme),
        }
    }

    pub fn get_authority(&self) -> Option<&Authority> {
        match &self.authority {
            None => None,
            Some(authority) => Some(authority),
        }
    }

    // pub fn get_path(&self) -> &String {
    //     &self.path
    // }
}

#[test]
fn test_uri_authority() {
    use super::*;

    let test_cases = [
        ("https://john:qwer123!@www.example.com:32423/index", "https",  Some("www.example.com")),
        ("ftp://ftp.is.co.za/rfc/rfc1808.txt", "ftp",  Some("ftp.is.co.za")),
        ("http://www.ietf.org/rfc/rfc2396.txt", "http",  Some("www.ietf.org")),
        ("ldap://[2001:db8::7]/c=GB?objectClass?one", "ldap",  Some("[2001:db8::7]")),
        ("mailto:John.Doe@example.com", "mailto", None),
        ("telnet://192.0.2.16:80/", "telnet", Some("192.0.2.16"))
        // ("news:comp.infosystems.www.servers.unix", "news"),
        // ("tel:+1-816-555-1212", "tel"),
        // ("urn:oasis:names:specification:docbook:dtd:xml:4.1.2", "urn"),
    ];

    for test in test_cases.iter() {
        println!("Testing {}", test.0);
        let uri_result = Uri::from_string(test.0);
        assert!(uri_result.is_ok());

        let uri = uri_result.ok();
        assert!(uri.is_some());

        assert_eq!(uri.as_ref().unwrap().get_scheme().unwrap(), test.1);

        let authority = uri.as_ref().unwrap().get_authority();
        assert!(authority.is_some() == test.2.is_some());

        if authority.is_some() {
            let host = authority.as_ref().unwrap().get_host();
            assert_eq!(host, test.2.unwrap());
        }
        

        // let port = authority.as_ref().unwrap().get_port();
        // assert!(port.is_some());
        // assert_eq!(port.unwrap(), 32423);

        // let user_info = authority.as_ref().unwrap().get_user_info();
        // assert!(user_info.is_some());
        // let username = user_info.unwrap().get_username();
        // assert_eq!(username, "john");

        // let password = user_info.unwrap().get_password();
        // assert!(password.is_some());
        // assert_eq!(password.unwrap(), "qwer123!");
    }
    
}
