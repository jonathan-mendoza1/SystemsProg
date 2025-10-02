
/*
1) Define a struct called Student
name,major, and three methods set_major,get_major
and constructor like new() static method, call in main()
*/

struct Student {
    name: String,
    major: String
}

impl Student {
    fn new(n: String, m:String) -> Student {
        Student {
            name: n,
            major: m
        }
    }

    fn get_major(&self) -> &String {
        return &self.major;
    }

    fn set_major(&mut self, new_major: String) {
        self.major = new_major;
    }
}

fn main() {
    let mut stu = Student::new("Jonathan".to_string(), "Computer Science".to_string());

    println!("Name: {}", stu.name);
    println!("Major: {}", stu.get_major());

    stu.set_major("Computer Engineering".to_string());
    
    println!("New Major: {}", stu.get_major())

}
