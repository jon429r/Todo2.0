use diesel::mysql::MysqlConnection;
use diesel::prelude::*;

fn connect_to_database() {
    //connect to aws cloud database using desiel
    let database_url = "mysql://root:Ds0OyRFTBgOHC5dWdmwX@todo-database.ciitmbvlwert.us-east-2.rds.amazonaws.com:3306/todo-database"
    .except("DATABASE_URL must be set");
    mysql::MysqlConnection::establish(database_url)
        .unwrap()
        .except("Error connecting to database: {}", database_url);
}

fn start_database() {
    connect_to_database();
    //create_tables using /Users/jonathanday/Todo2.0/Todo/todo.sql
}

fn create_tables() {
    println!("Tables created");
}

fn db_add_project(&mut self, project: Project) {
    // TODO Add project to database in project table
}

fn db_finish_project(&mut self, project: Project) {
    // TODO Remove project from database in project table and add it to finished project table
}

fn db_fetch_projects(&self, project: Project) {
    // TODO Fetch projects from database
    // return projects
}

fn db_fetch_project(&self, project: Project) {
    // TODO Fetch project from database
    // return project
}

fn db_add_user(&mut self, user: User) {
    // TODO Add user to database in user table
}

fn db_fetch_user(&self, user: User) {
    // TODO Fetch user from database
    // return user
}

db_remove_user(&mut self, user: User) {
    // TODO Remove user from database in user table
}

db_fetch_users(&self, user: User) {
    // TODO Fetch users from database
    // return users
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
