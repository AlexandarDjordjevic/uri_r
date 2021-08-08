
pub struct Uri{
    scheme: Option<String>,
    path: String
}

#[allow(unused_variables)]
#[allow(dead_code)]
impl Uri{
    pub fn from_string(uri: String) -> Self {
        
        let mut _uri = Uri {
            scheme : None,
            path : uri.to_string()
        };
        
        let authority_or_path_delimiter_position = uri.find('/');
        let colon_after_scheme_position = uri.find(':');
        
        _uri.scheme = None;

        match colon_after_scheme_position {
            None => (),
            Some(colon_index) => {
                match authority_or_path_delimiter_position {
                    None => _uri.scheme = Some(uri[..colon_index].to_string()),
                    Some(index) => {
                        if index > colon_index {
                            _uri.scheme = Some(uri[..colon_index].to_string());
                        }
                    }
                }
            }
        }

        _uri
    }

    pub fn get_scheme(&self) -> Option<&String> {
        match &self.scheme {
            None => None,
            Some(scheme) => Some(scheme)
        }
    }

    pub fn get_path(&self) -> &String {
        &self.path
    }
}
