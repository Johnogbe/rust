struct StudentRecord {
    name: String,
    mat_no: String,
    scores: Scores,
}

struct Scores {
    maths: u32,
    gst: u32,
    physics: u32,
    chemistry: u32,
}

// Function to sum all scores for each student
fn print_total_scores(students: &Vec<StudentRecord>) {
    println!("--------------------------------------------------");
    println!("| Name   | Mat No | Total Score | Average Score |");
    println!("--------------------------------------------------");

    for student in students {
        let total = student.scores.maths
            + student.scores.gst
            + student.scores.physics
            + student.scores.chemistry;
        let average = total as f32 / 4.0;
        println!(
            "| {:<6} | {:<6} | {:<11} | {:<13.2} |",
            student.name, student.mat_no, total, average
        );
    }

    println!("--------------------------------------------------");
}

fn main() {
    // Creating instances of Scores
    let score1 = Scores {
        maths: 70,
        gst: 81,
        physics: 51,
        chemistry: 62,
    };
    let score2 = Scores {
        maths: 45,
        gst: 60,
        physics: 71,
        chemistry: 49,
    };
    let score3 = Scores {
        maths: 55,
        gst: 72,
        physics: 73,
        chemistry: 57,
    };
    let score4 = Scores {
        maths: 69,
        gst: 93,
        physics: 54,
        chemistry: 72,
    };
    let score5 = Scores {
        maths: 65,
        gst: 80,
        physics: 60,
        chemistry: 55,
    };
    let score6 = Scores {
        maths: 58,
        gst: 75,
        physics: 66,
        chemistry: 70,
    };
    let score7 = Scores {
        maths: 90,
        gst: 88,
        physics: 74,
        chemistry: 85,
    };
    let score8 = Scores {
        maths: 78,
        gst: 85,
        physics: 69,
        chemistry: 80,
    };
    let score9 = Scores {
        maths: 50,
        gst: 70,
        physics: 65,
        chemistry: 60,
    };
    let score10 = Scores {
        maths: 49,
        gst: 61,
        physics: 45,
        chemistry: 60,
    };

    // Creating instances of StudentRecord
    let students = vec![
        StudentRecord {
            name: String::from("John"),
            mat_no: String::from("001"),
            scores: score1,
        },
        StudentRecord {
            name: String::from("Ngozi"),
            mat_no: String::from("002"),
            scores: score2,
        },
        StudentRecord {
            name: String::from("Mary"),
            mat_no: String::from("003"),
            scores: score3,
        },
        StudentRecord {
            name: String::from("Jane"),
            mat_no: String::from("004"),
            scores: score4,
        },
        StudentRecord {
            name: String::from("Widom"),
            mat_no: String::from("005"),
            scores: score5,
        },
        StudentRecord {
            name: String::from("Solomon"),
            mat_no: String::from("006"),
            scores: score6,
        },
        StudentRecord {
            name: String::from("Smith"),
            mat_no: String::from("007"),
            scores: score7,
        },
        StudentRecord {
            name: String::from("Jay"),
            mat_no: String::from("008"),
            scores: score8,
        },
        StudentRecord {
            name: String::from("Philip"),
            mat_no: String::from("009"),
            scores: score9,
        },
        StudentRecord {
            name: String::from("Joses"),
            mat_no: String::from("010"),
            scores: score10,
        },
    ];

    // Call function to print total scores
    print_total_scores(&students);
}
