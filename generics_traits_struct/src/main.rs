// define 2 struct undegrad and grad student
// define trait show info
// grad student should have a thesis compnent
// gpa and major will be shared
// create another struct called Enrollment
// inside enrollment store undegrad and grads together
// implement show_info for all enrolled student
// everywhere use generics and traits, no if or match statement
// program to behavior only

// Define the common behavior with a trait
pub trait ShowInfo {
    fn show_info(&self) -> String;
}

// Undergraduate struct
pub struct Undergraduate {
    // gpa and major are shared (duplicated here)
    pub gpa: f32,
    pub major: String,
}

// Graduate struct
pub struct Graduate {
    // gpa and major are shared (duplicated here)
    pub gpa: f32,
    pub major: String,
    // grad student should have a thesis component
    pub thesis_component: String,
}

// Implement ShowInfo for Undergraduate
impl ShowInfo for Undergraduate {
    fn show_info(&self) -> String {
        format!(
            "Undergraduate | Major: {} | GPA: {:.2}",
            self.major,
            self.gpa
        )
    }
}

// Implement ShowInfo for Graduate
impl ShowInfo for Graduate {
    fn show_info(&self) -> String {
        format!(
            "Graduate | Major: {} | GPA: {:.2} | Thesis: {}",
            self.major,
            self.gpa,
            self.thesis_component
        )
    }
}

// create another struct called Enrollment
pub struct Enrollment<'a> {
    // inside enrollment store undegrad and grads together
    pub enrolled_students: Vec<&'a dyn ShowInfo>,
}

impl<'a> Enrollment<'a> {
    pub fn new(students: Vec<&'a dyn ShowInfo>) -> Self {
        Enrollment {
            enrolled_students: students,
        }
    }

    // implement show_info for all enrolled student
    pub fn show_all_enrolled_info(&self) {
        println!("\n--- Current Enrollment Roster ---");
        // everywhere use generics and traits, no if or match statement
        // program to behavior only
        for student in self.enrolled_students.iter() {
            println!("* {}", student.show_info());
        }
        println!("\n");
    }
}

// generic function/ trait bound
fn print_any_info<T: ShowInfo>(item: &T) {
    println!("Generic Info: {}", item.show_info());
}

fn main() {
    let undergrad = Undergraduate {
        gpa: 3.8,
        major: String::from("Computer Science"),
    };
    
    let grad = Graduate {
        gpa: 3.55,
        major: String::from("Electrical Engineering"),
        thesis_component: String::from("AI Model Optimization"),
    };

    println!("**Individual Student**");
    println!("Undergrad: {}", undergrad.show_info());
    println!("Graduate: {}", grad.show_info());

    // different types using trait objects for Enrollment
    let students_to_enroll: Vec<&dyn ShowInfo> = vec![
        &undergrad,
        &grad,
    ];

    let enrollment = Enrollment::new(students_to_enroll);
    enrollment.show_all_enrolled_info();

    // generic function
    println!("**Generic Function**");
    print_any_info(&undergrad);
    print_any_info(&grad);
}