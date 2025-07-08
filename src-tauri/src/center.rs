pub struct User {
    pub username: String,
    pub password: String,
    pub islogin: String,
}

impl User {
    pub fn new(username: String, password: String) -> User {
        User {
            username,
            password,
            islogin:"no".to_string(),
        }
    }
    pub fn get_username(&self) -> &String {
        &self.username
    }
    pub fn get_password(&self) -> &String {
        &self.password
    }
    pub fn get_islogin(&mut self) -> &String {
        &self.islogin
    }
    pub fn set_username(&mut self, username: String) {
        self.username = username;
    }
    pub fn set_password(&mut self, password: String) {
        self.password = password;
    }
    pub fn set_islogin(&mut self, islogin: String) {
        self.islogin = islogin;
    }
}

pub struct Room {
    pub id: String,
    pub owner: String,
}

impl Room {
    pub fn new(id: String, owner: String) -> Room {
        Room {
            id,
            owner,
        }
    }
    pub fn get_id(&self) -> &String {
        &self.id
    }
    pub fn get_owner(&self) -> &String {
        &self.owner
    }
    pub fn set_id(&mut self, id: String) {
        self.id = id;
    }
    pub fn set_owner(&mut self, owner: String) {
        self.owner = owner;
    }
}