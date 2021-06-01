use std::collections::HashMap;

pub struct StudentsBase{
    Students: HashMap<i32, Vec<String>>
}


impl StudentsBase
{
    pub fn new () -> StudentsBase{
        StudentsBase{
            Students: HashMap::new()
        }
    }

    /*method to add students*/

    pub fn addStudent(&mut self,grade: i32, name: &str)
    {
        let students = self.Students.entry(grade).or_insert(Vec::new());
        students.push(name.to_string());
        students.sort();

    }
    /*    method to get list of all students enrolled in given grade */
    pub fn byGrades(self, grade: i32) -> Option<Vec<String>>
    {
        self.Students.get(&grade).cloned()
    }

    /*method to get all students sorted by increasing order grade*/
    pub fn allStudents(&mut self) -> Vec<String>
    {
        let mut grades: Vec<_> = self.Students.keys().cloned().collect();
        // sort the  grades increasing order
        grades.sort();
        let mut allstudents:Vec<String> = Vec::new();
        // get students names by increasing grades
        for index in grades.iter(){
            allstudents.append(&mut self.Students.get(index).cloned().unwrap());
        }
        allstudents
    }
}




fn main() {
    let mut students = StudentsBase::new();
    students.addStudent( 5, "tom");
    students.addStudent( 2, "kat");
    students.addStudent( 3, "mary");
    students.addStudent( 1, "john");
    students.addStudent( 2, "winston");
    students.addStudent( 3, "philip");
    students.addStudent( 1, "meghan");
    students.addStudent( 2, "jim");
    students.addStudent( 3, "peter");
    students.addStudent( 3, "william");
    students.addStudent( 1, "harry");
    students.addStudent( 1, "ron");


    let student = students.allStudents();
    println!("allstudents: {:?}", student);
}


