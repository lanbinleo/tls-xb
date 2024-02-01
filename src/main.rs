mod gpa;
mod login;
mod semester;
mod subject;

use gpa::get_gpa;
use login::login;
use semester::*;
use subject::*;
use std::sync::Arc;
use text_io::read;

#[tokio::main]
async fn main() {
    let client = login().await;

    let semesters = get_semesters(&client).await;
    print_semesters(&semesters);
    let i: usize = read!();
    let semester_id = semesters[i].id;

    let subject_ids = get_subject_ids(&client, semester_id).await;
    let mut handles = Vec::new();
    let arc_client = Arc::new(client.clone());
    for subject_id in subject_ids {
        let client = Arc::clone(&arc_client);
        let handle =
            tokio::spawn(async move { get_subject(&client, semester_id, subject_id).await });
        handles.push(handle);
    }
    let mut subjects = Vec::new();
    for handle in handles {
        subjects.push(handle.await.unwrap());
    }
    for subject in subjects {
        print_subject(subject);
    }

    let gpa = get_gpa(&client, semester_id).await;
    println!("GPA: {}", gpa);
}

fn print_semesters(semesters: &[Semester]) {
    for (i, semester) in semesters.iter().enumerate().rev() {
        println!("{:2}: {}.{}", i, semester.year, semester.semester);
    }
    print!("Choose a semester: ");
}

fn print_subject(subject: Subject) {
    if subject.total_score.is_nan() {
        return;
    }
    println!("{}: {}", subject.subject_name, subject.total_score);
    for evaluation_project in subject.evaluation_projects {
        if !evaluation_project.score_is_null {
            println!(
                "{}: {} / {} / {} ({}%)",
                evaluation_project.evaluation_project_e_name,
                evaluation_project.score,
                evaluation_project.score_level,
                evaluation_project.gpa,
                evaluation_project.proportion,
            );
        }
    }
    println!();
}
