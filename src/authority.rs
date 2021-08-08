#[allow(unused_variables)]
#[allow(dead_code)]
pub struct Authority {
    user_info: Option<String>,
    host: String,
    port: Option<u16>,
}

impl Authority{
    pub fn from_string(uri: &str) -> (Option<Authority> , &str){
        let start_index = uri.find("//");
        match start_index {
            Some(start_index) => {
                let auth = &uri[start_index + 2 ..];
                let end_index = auth.find("/").unwrap_or(
                    auth.find("#").unwrap_or(
                        auth.find("?").unwrap_or(auth.len())
                    )
                );
                (Some(
                    Authority{
                        user_info: None,
                        host: auth[..end_index].to_string(),
                        port: None,
                    }
                ), &uri[end_index..])
            },
            None => (None, uri)
        }
    }

    // fn parse_user_info(&self, string: &String) {
    
    // }

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