mod uri;

pub use crate::{
    uri::Uri
};

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn empty_scheme_parse(){
        let test_cases = vec![
            "//foo:bar@www.example.com/",
            "//www.example.com/a:b",
            "//www.example.com/foo?a:b",
            "//www.example.com/foo#a:b",
            "//[v7.:]/",
            "/:/foo",
            "/home/example"
        ];
        for test_case in test_cases.iter() {
            let uri = uri::Uri::from_string(test_case.to_string());
            assert_eq!(Option::None, uri.get_scheme());
        }
        
    }

    #[test]
    fn nonempty_scheme_parse(){
        let test_cases = vec![
            ("http://foo:bar@www.example.com/", "http"),
            ("https://foo:bar@www.example.com/", "https"),
            ("ftp://192.168.0.100", "ftp"),
            ("sftp://192.168.0.100", "sftp"),
            ("file:///home/example", "file"),
            ("ldap://[2001:db8::7]/c=GB?objectClass?one", "ldap"),
            ("mailto:John.Doe@example.com", "mailto"),
            ("news:comp.infosystems.www.servers.unix", "news"),
            ("tel:+1-816-555-1212", "tel"),
            ("telnet://192.0.2.16:80/", "telnet"),
            ("urn:oasis:names:specification:docbook:dtd:xml:4.1.2", "urn")
        ];

        for test_case in test_cases.iter() {
            let (_uri, _scheme) = test_case;
            let uri = uri::Uri::from_string(_uri.to_string());
            assert_eq!(_scheme.to_string(), uri.get_scheme().unwrap().to_string());
        }
        
    }
}
