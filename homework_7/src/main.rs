
// define 2 structs undergrad and grad student

// define trait show info

// grad student should have a thesis component 
// gpa and major will be shared

// create another struct called Enrollment
// inside enrollment store undergrad and grads together
// implement show_info for all enrolled students

// everywhere use generics and traits, no if or match statment
// program to behavior only

pub trait ShowInfo {
    fn show_info(&self) -> String;
}

pub struct Undergrad {
    pub gpa: f64,
    pub major: String,
}

impl ShowInfo for Undergrad {
    fn show_info(&self) -> String {
        format!("Undergrad - GPA: {:.2}, Major: {}", self.gpa, self.major)
    }
}

pub struct Grad {
    pub gpa: f64, 
    pub major: String,
    pub thesis: String,
}

impl ShowInfo for Grad {
    fn show_info(&self) -> String {
        format!("Grad - GPA: {:.2}, Major: {}, Thesis: {}", self.gpa, self.major, self.thesis)
    }
}

fn main() {
    let undergrad_stu = Undergrad {gpa: 3.78, major: "Computer Science".into()};
    let grad_stu = Grad {gpa: 3.95, major: "Computer Science".into(), thesis: "Cybersecurity".into()};

    let students: Vec<&dyn ShowInfo> = vec![&undergrad_stu, &grad_stu];

    for s in &students {
        println!("{}", s.show_info());
    }
}

