extern crate menu_generator;
use menu_generator::read_input as io;
use menu_generator::student_registration as sg;
fn main() {
    println!("Hello \nDo you want to enroll in IOT?\nPress 'Y' for yes Press 'N' for no");
    let ans:char=io::char_read();
    if ans=='y'||ans=='Y'{
        println!("Enter your Name: ");
        let name:String=io::str_read();
        println!("Enter your Course: ");
        let course:String=io::str_read();
        println!("Enter your City: ");
        let city:String=io::str_read();
        println!("Do you want Distance learning: ");
        let dl:String=io::str_read();
        let student1=sg::Student::register(name,course,city,dl);
        student1.display_card();
    }
    else{
        println!("Get Lost!");
    }
}
