struct Uri{
    scheme: String,
    path: String
}

#[allow(unused_variables)]
#[allow(dead_code)]
impl Uri{
    fn from_string(uri: String) -> Self {
        
        let mut _uri = Uri {
            scheme : String::from(""),
            path : uri.to_string()
        };
        
        let authority_or_path_delimiter_position = uri.find('/');
        let colon_after_scheme_position = uri.find(':');
        
        _uri.scheme = String::from("");

        match colon_after_scheme_position {
            None => (),
            Some(colon_index) => {
                match authority_or_path_delimiter_position {
                    None => (),
                    Some(index) => {
                        if index > colon_index {
                            _uri.scheme = uri[..colon_index].to_string();
                        }
                    }
                }
            }
        }

        _uri
    }

    fn print_info(&self) {
        println!("***********************");
        println!("Scheme: {}", self.scheme);        
        println!("Path: {}", self.path);
        println!("***********************");
    }

    fn get_scheme(&self) -> String {
        self.scheme.to_string()
    }

    fn get_path(&self) -> String {
        self.path.to_string()
    }
}


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
        for test_case in test_cases {
            let uri = Uri::from_string(test_case.to_string());
            assert_eq!("".to_string(), uri.get_scheme());
        }
        
    }

    #[test]
    fn nonempty_scheme_parse(){
        let test_cases = vec![
            ("http://foo:bar@www.example.com/", "http"),
            ("https://foo:bar@www.example.com/", "https"),
            ("ftp://192.168.0.100", "ftp"),
            ("sftp://192.168.0.100", "sftp"),
            ("file:///home/example", "file")
        ];

        for test_case in test_cases {
            let (_uri, _scheme) = test_case;
            let uri = Uri::from_string(_uri.to_string());
            assert_eq!(_scheme.to_string(), uri.get_scheme());
        }
        
    }
}
