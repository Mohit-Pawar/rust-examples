use std::io;

#[derive(Debug)]
struct Student {
    name: String,
    id: String,
    physics_marks: i64,
    math_marks: i64,
    english_marks: i64,
}

fn main() {
    let mut number_of_student = String::new();
    
    println!("enter number of student");
    io::stdin().read_line(&mut number_of_student).expect("unable to get number of student");
    let number_of_student: usize = number_of_student.trim().parse::<usize>().unwrap();
    println!("number of students {}", number_of_student);


    let mut students = Vec::new();

    for _number in 0..number_of_student {
        let mut name = String::new();
        let mut id = String::new();
        let mut physics_marks = String::new();
        let mut math_marks = String::new();
        let mut english_marks = String::new();

        println!("enter student name");
        io::stdin().read_line(&mut name).expect("unable to read name");
        println!("enter student id");
        io::stdin().read_line(&mut id).expect("unable to read id");
        println!("ender student physics marks");
        io::stdin().read_line(&mut physics_marks).expect("unable to read physics marks");
        let physics_marks: i64 = physics_marks.trim().parse::<i64>().unwrap();
        println!("ender student math marks");
        io::stdin().read_line(&mut math_marks).expect("unable to read math marks");
        let math_marks: i64 = math_marks.trim().parse::<i64>().unwrap();
        println!("ender student physics marks");
        io::stdin().read_line(&mut english_marks).expect("unable to read english marks");
        let english_marks: i64 = english_marks.trim().parse::<i64>().unwrap();

        let student = Student{
            name: name,
            id: id,
            physics_marks: physics_marks,
            math_marks: math_marks,
            english_marks: english_marks,
        };

        students.push(student)
    }

    loop {
        println!("enter your choice");
        println!("1) sort by name");
        println!("2) sort by total");
        println!("3) exit");

        let mut sort_by = String::new();
        io::stdin().read_line(&mut sort_by).expect("enable to get sort_by choice");
        let sort_by: usize = sort_by.trim().parse::<usize>().unwrap();
        
        if sort_by == 1 {
            students.sort_by_key(|d| d.name.clone());
        } else if sort_by == 2 {
            students.sort_by_key(|d| {d.physics_marks + d.math_marks + d.english_marks});
        } else {
            break;
        }
        

        for student in students.iter() {
            println!("{:?}", student);
        }
    }
}
