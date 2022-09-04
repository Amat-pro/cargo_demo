// todo: config

#[derive(Clone)]
pub struct Config {
    pub basic_authentication: BasicAuthentication,
    pub server: Server,
}

#[derive(Clone)]
pub struct BasicAuthentication {
    pub username: String,
    pub password: String,
}

#[derive(Clone)]
pub struct Server {
    pub ip: String,
    pub port: String,
}

impl Config {
    pub fn new() -> Self {
        Config {
            basic_authentication: BasicAuthentication {
                username: "".to_string(),
                password: "".to_string(),
            },
            server: Server {
                ip: "".to_string(),
                port: "".to_string(),
            },
        }
    }

    pub fn from(basic: BasicAuthentication, server: Server) -> Self {
        Config {
            basic_authentication: basic,
            server,
        }
    }
}

impl BasicAuthentication {
    pub fn new(username: String, password: String) -> Self {
        BasicAuthentication {
            username,
            password,
        }
    }

    pub fn base64(&self) -> String {
        let str = format!("{}:{}", self.username, self.password);
        base64::encode(str)
    }
}

impl Server {
    pub fn new(ip: String, port: String) -> Self {
        Server {
            ip,
            port,
        }
    }
}


#[cfg(test)]
mod test {
    use crate::config::BasicAuthentication;

    #[test]
    fn basic_base64() {
        let basic = BasicAuthentication::new("username".to_string(), "password".to_string());
        println!("==> {}", basic.base64());
        assert_eq!(basic.base64(), "dXNlcm5hbWU6cGFzc3dvcmQ=")
    }
}