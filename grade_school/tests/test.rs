extern crate grade_school as school;

#[cfg(test)]
mod tests
{
    #[test]
    fn testByGrades()
    {
        let mut Students = school::StudentsBase::new();
        Students.addStudent(2, "Beatka");
        Students.addStudent(2, "Cyprian");
        Students.addStudent(2, "Alinka");
        Students.addStudent(3, "Kasia");
        Students.addStudent(3, "Tomek");
        Students.addStudent(3, "Kacper");
        Students.addStudent(1, "Zdzisław");
        Students.addStudent(1, "Kamil");
        Students.addStudent(1, "Wiesław");
        let output = vec!["Kamil", "Wiesław", "Zdzisław"];
        let result: Vec<String> = Students.byGrades(1).unwrap();
        assert_eq!(result, output);

    }

    #[test]
    fn sortedtStudentsGrades()
    {
        let mut Students = school::StudentsBase::new();
        Students.addStudent(2, "Beatka");
        Students.addStudent(2, "Cyprian");
        Students.addStudent(2, "Alinka");
        Students.addStudent(3, "Kasia");
        Students.addStudent(3, "Tomek");
        Students.addStudent(3, "Kacper");
        Students.addStudent(1, "Zdzisław");
        Students.addStudent(1, "Kamil");
        Students.addStudent(1, "Wiesław");
        let output = vec!["Kamil", "Wiesław", "Zdzisław", "Alinka","Beatka", "Cyprian", "Kacper", "Kasia", "Tomek" ];
        let result: Vec<String> = Students.allStudents();
        assert_eq!(result, output);

    }

    #[test]
    fn sortedtNotStudentsGrades()
    {
        let mut Students = school::StudentsBase::new();
        Students.addStudent(2, "Beatka");
        Students.addStudent(2, "Cyprian");
        Students.addStudent(2, "Alinka");
        Students.addStudent(3, "Kasia");
        Students.addStudent(3, "Tomek");
        Students.addStudent(3, "Kacper");
        Students.addStudent(1, "Zdzisław");
        Students.addStudent(1, "Kamil");
        Students.addStudent(1, "Wiesław");
        let output = vec!["Kamil", "Zdzisław", "Wiesław", "Alinka","Kacper", "Cyprian", "Beatka", "Kasia", "Tomek" ];
        let result: Vec<String> = Students.allStudents();
        assert_ne!(result, output);
    }

    #[test]
    fn emptyName()
    {
        let mut Students = school::StudentsBase::new();
        Students.addStudent(2, "Beatka");
        assert_eq!(Students.addStudent(2, " "),());

    }


}