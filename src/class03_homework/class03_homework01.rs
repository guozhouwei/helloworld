use std::f32::consts::E;
use std::fmt::Display;
use std::io::{self, stdout};
use std::collections::HashMap;
use std::{string, result};

/*
 * 课程
 */
#[derive(Debug)]
pub struct Course {
    name: String,
    compulsory: bool,  //true 必修课，false 选修课
    teacher_name: String,  //授课老师
}


/*
 * 社团
 */
#[derive(Debug)]
pub struct Ssociation {
    name: String,   //社团名称
    aim: String,  //社团目标
}


/*
 * 班级
 */
#[derive(Debug)]
pub struct GradeClass {
    name:String, //班级名称
    grade: String,  //年级
    faculty: String,   //院系
    speciality: String,   //专业
    instructor: String, //辅导员
}

/*
 * 性别
 */
#[derive(Debug)]
pub enum Gender {
    Male,   // 男
    Female, // 女
  }

/*
 * 学生
 */
#[derive(Debug)]
pub struct Student {
    name: String,  //姓名
    age: u8,   //年龄
    sex: Gender,   //性别
    grade_class: GradeClass, //班级
    ssociations: Vec<Ssociation>, //参加的社团
    courses: Vec<Course>, //学习的课程
}

/*
 * 学生管理系统
 * 1、创建课程 Course
 */
pub fn student_system() {
    /*
     * step 1  创建课程
     * 英语，计算机，高数，数字电路
     */
    println!("step 1 创建课程:");
    let mut course_map: HashMap<String, &Course>= HashMap::new();
    //
    loop {
        println!("请输入课程名:");
        let mut input = String::new(); //课程名        
        io::stdin().read_line(&mut input).expect("error");
        let name: String = input.trim().to_string();
        //
        println!("是否是必修课（yes/no）:");
        let mut input1 = String::new(); //是否必修课
        io::stdin().read_line(&mut input1).expect("error");
        let mut compulsory = false;
        if "yes" == input1.trim() {
            compulsory = true;
        }
        //
        let course = Course {
                                        name,
                                        compulsory,
                                        teacher_name: String::from("张三"),
                                    };
       //course_map.insert(String::from("数学"), &course);

        //
        println!("是否继续创建课程（yes/no）:");
        let mut input2 = String::new();
        io::stdin().read_line(&mut input2).expect("error");
        let continue_create = input2.trim();
        if "yes" == continue_create {
            println!("继续创建课程。。。");
        } else {
            println!("课程创建完毕。。。"); 
           break course_map
        }
       // course_map
    };
}