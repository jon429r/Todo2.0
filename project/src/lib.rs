struct Project {
    ID: i32,
    Name: String,
    WorkingDirectory: String,
    ProjectLink: String,
    Description: String,
}
impl Project {
    fn new(
        id: i32,
        name: String,
        working_directory: String,
        project_link: String,
        description: String,
    ) -> Self {
        Project {
            ID: id,
            Name: name,
            WorkingDirectory: working_directory,
            ProjectLink: project_link,
            Description: description,
        }
    }
}
fn add_project(&mut self, project: Project) {
    self.Projects.push(project);
    // TODO Add project to database in project table
}
fn finish_project(&mut self, project: Project) {
    self.FinishedProjects.push(project);
    self.Projects.retain(|p| p.ID != project.ID);
    // TODO Remove project from database in project table and add it to finished project table
}
fn fetch_projects(&self, project: Project) /* -> Vec<Project> */
{
    // TODO Fetch projects from database
    // return projects
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
