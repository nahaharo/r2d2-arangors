use std::result::Result;
use r2d2;

#[derive(Clone, Debug)]
pub struct ArangoDBConnectionManager {
    url: String,
    username: String,
    password: String,
    use_jwt: bool,
}

impl ArangoDBConnectionManager {
    pub fn new(url: &str, username: &str, password: &str, use_jwt: bool) -> ArangoDBConnectionManager {
        ArangoDBConnectionManager {
            url: url.to_owned(),
            username: username.to_owned(),
            password: password.to_owned(),
            use_jwt,
        }
    }
}

impl r2d2::ManageConnection for ArangoDBConnectionManager {
    type Connection = arangors::Connection;
    type Error = arangors::ClientError;

    fn connect(&self) -> Result<Self::Connection, Self::Error> {
        if self.use_jwt == true {
            arangors::Connection::establish_jwt(
                &self.url,
                &self.username,
                &self.password
            )
        } else {
            arangors::Connection::establish_basic_auth(
                &self.url,
                &self.username,
                &self.password
            )
        }
    }

    fn is_valid(&self, conn: &mut Self::Connection) -> Result<(), Self::Error> {
        // validate_server()
        Ok(())
    }

    fn has_broken(&self, conn: &mut Self::Connection) -> bool {
        // validate_server().is_err()
        true
    }
}