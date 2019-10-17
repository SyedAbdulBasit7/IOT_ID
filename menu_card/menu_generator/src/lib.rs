
pub mod read_input{
    use std::io;
    pub fn char_read()->char{
        let mut input=String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let input:char=input.trim().parse().expect("Invalid input");
        input
    }
    pub fn str_read()->String{
        let mut input=String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        input
    }
     pub fn int_read()->i64{
        let mut input=String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let input:i64=input.trim().parse().expect("Invalid input");
        input
    }
    pub fn float_read()->f64{
        let mut input=String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let input:f64=input.trim().parse().expect("Invalid input");
        input
    }
}

pub mod student_registration{
    #[derive(Debug)]
    pub struct Student{
        Name:String,
        Course:String,
        City:String,
        Distance_Learning:String,
    }
    impl Student{
        pub fn register(Name:String, Course:String, City:String, Distance_Learning:String)->Student
        {
            Student{
                Name,
                Course,
                City,
                Distance_Learning,
            }
        }
        pub fn display_card(&self){
            println!("*********************************************************************************************");
            println!("**********************************************ID CARD****************************************");
            print!("*\t\tName: {}*\t\tCourse: {}*\t\tCity: {}*\t\tDistance learning:{}",
            self.Name,self.Course,self.City,self.Distance_Learning);
            super::auto_generate::display_auto();
            println!("**********************************************************************************************");
        }
    }
}

pub mod auto_generate{
    pub fn display_auto(){
    let mut RollNo:i32=super::random::random_gen();
    let Center="Bahria Auditorium".to_string();
    let Campus="Karsaz".to_string();
    let Quarter="Q1".to_string();
    let Batch:i32=2;
    let day_time="Sunday - 9 AM to 12 PM".to_string();
    println!("*\t\tRoll No: IOT0{}\t\t\n*\t\tCenter: {}\t\t\n*\t\tCampus: {}\t\t\n*\t\tQuarter: {}\t\t\n*\t\tBatch: {}\t\t\n*\t\tDays / Time: {}\t\t",
    RollNo,Center,Campus,Quarter,Batch,day_time);
    }
}

pub mod random{
    use rand::Rng;
    pub fn random_gen()->i32{
        let number:i32=rand::thread_rng().gen_range(1000,5000);
        number
    }
}