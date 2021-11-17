#![warn(dead_code)]
use log::info;
// Student Structure
struct Student {
    name: String,
    roll_no: i32,
    marks: i32,
    department: String,
    school: String,
}
// Score Structure
struct Score {
    hindi: f32,
    english: f32,
    maths: f32,
    science: f32,
}
// Function "new" initializes Student objects
//
// #Arguments
//
// Student Structure
//
// #Return
//
// Return the Student type objects
fn _new(){
    let _student1 = Student {
        name: String::from("Kchitiz"),
        roll_no: 44,
        marks: 67,
        department: String::from("IT"),
        school: String::from("AEC"),
    };
    let _student2 = Student {
        name: String::from("Mayank"),
        roll_no: 38,
        marks: 32,
        department: String::from("IT"),
        school: String::from("AEC"),
    };
}
// Function "get_average" is to get average of all scores
//
// #Arguments
//
// Score Structure
//
// #Return
//
// Return the average of marks
fn get_average(avg: &Score) -> f32{
    let avg: f32 = (avg.hindi + avg.english + avg.maths + avg.science) / 4.0;
    avg
}
// Function "pass_student" add numbers to student's score if score < 35
//
// #Arguments
//
// Score Structure
//
// #Return
//
// Return the array of new scores stored in it

fn pass_student(marks: &Score) -> [f32; 4]{
    let mut arr: [f32; 4] = [marks.hindi, marks.english, marks.maths, marks.science];

    let diff1: f32;
    let diff2: f32;
    let diff3: f32;
    let diff4: f32;
    if arr[0] < 35.0 {
        diff1 = 35.0 - arr[0];
        arr[0] += diff1;
    }
    if arr[1] < 35.0 {
        diff2 = 35.0 - arr[1];
        arr[1] += diff2;
    }
    if arr[2] < 35.0 {
        diff3 = 35.0 - arr[2];
        arr[2] += diff3;
    }
    if arr[3] < 35.0 {
        diff4 = 35.0 -
            arr[3];
        arr[3] += diff4;
    }
    return arr;
}

impl Score {
    // Function "compare_student" is to print difference of each subject's score b/w students
    //
    // #Arguments
    //
    // Score Structure
    //
    // #Return
    //
    // No Return

    fn compare(&self, other: &Score) {
        if self.hindi >= other.hindi {
            info!("Student_1 has higher marks in Hindi by: {}", self.hindi - other.hindi);
        }
        else {
            info!("Student_2 has higher marks in Hindi by: {}", other.hindi - self.hindi);
        }

        if self.english >= other.english {
            info!("Student_1 has higher marks in English by: {}", self.english - other.english);
        }
        else {
            info!("Student_2 has higher marks in English by: {}", other.english - self.english);
        }

        if self.maths >= other.maths {
            info!("Student_1 has higher marks in Maths by: {}", self.maths - other.maths);
        }
        else {
            info!("Student_2 has higher marks in Maths by: {}", other.maths - self.maths);
        }

        if self.science >= other.science {
            info!("Student_1 has higher marks in Science by: {}", self.science - other.science);
        }
        else {
            info!("Student_2 has higher marks in Science by: {}", other.science - self.science);
        }

    }
}
// Main function

fn main() {
    env_logger::init();
    // Specifying scores of student1
    let mut student1_score = Score {
        hindi: 32.0,
        english: 72.0,
        maths: 88.0,
        science: 83.0,
    };
    // Specifying scores of student2

    let mut student2_score = Score {
        hindi: 18.0,
        english: 68.0,
        maths: 22.0,
        science: 35.0,
    };

    let arr1 = [student1_score.hindi, student1_score.english, student1_score.maths, student1_score.science];
    let arr2 = [student2_score.hindi, student2_score.english, student2_score.maths, student2_score.science];
    info!("Present scores of Student_1: {:?}", arr1);
    info!("Present scores of Student_2: {:?}", arr2);

    info!("avg of Student_1: {}", get_average(&student1_score));
    info!("avg of Student_2: {}", get_average(&student2_score));

    let array1: [f32; 4] = pass_student(&student1_score);
    let array2: [f32; 4] = pass_student(&student2_score);

    info!("New scores of Student_1: {:?}", array1);
    info!("New scores of Student_2: {:?}", array2);

    student1_score = Score {
        hindi: array1[0],
        english: array1[1],
        maths: array1[2],
        science: array1[3],
    };

    student2_score = Score {
        hindi: array2[0],
        english: array2[1],
        maths: array2[2],
        science: array2[3],
    };


    info!("New avg of Student_1: {}", get_average(&student1_score));
    info!("New avg of Student_2: {}", get_average(&student2_score));


    student1_score.compare(&student2_score);
}
