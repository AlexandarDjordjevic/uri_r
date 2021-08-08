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
    fn scheme_parse(){
        let uri = Uri::from_string("http://www.example.com/index".to_string());
        assert_eq!("http".to_string(), uri.get_scheme());
    }
}
