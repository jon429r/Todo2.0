struct user {
    ID: i32,
    Username: String,
    Password: String,
    Projects: Vec<Project>,
    FinishedProjects: Vec<Project>,
}

impl user {
    fn new(id: i32, username: String, password: String) -> user {
        user {
            ID: id,
            Username: username,
            Password: password,
            Projects: Vec::new(),
            FinishedProjects: Vec::new(),
        }
    }
    fn add_user(&mut self, user: user) {
        self.Users.push(user);
        // TODO Add user to database in user table
    }
    fn fetch_user(&self, user: user) {
        // TODO Fetch user from database
        // return user
    }
    fn remove_user(&mut self, user: user) {
        self.Users.retain(|u| u.ID != user.ID);
        // TODO Remove user from database in user table
    }
}
