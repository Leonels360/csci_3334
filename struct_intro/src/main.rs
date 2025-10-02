
impl Student {
        fn new(name: String, major: String) -> Student{
                Student{
                        name: name,
                        major: major,
                }
        }
        fn set_major(&mut self, new_major: String){
                self.major = new_major

        }
        fn get_major(&self) -> String{
                self.major.clone()

        }
}
struct Student{
        name: String,
        major: String,
}

fn main(){
        let mut student1 = Student::new(
                "Leonel Saenz".to_string(),
                "Computer Science".to_string()
        );

        println!("Major for {}: {}",student1.name, student1.get_major());

        student1.set_major("Business".to_string());

        let updated_major = student1.get_major();

        println!("Updated Major for {}: {}", student1.name, updated_major);
        
}