use serde::Serialize;
use rocket::serde::json::Json;

#[derive(Serialize)]
struct Student {
    name: String,
    surname: String,
    age: u8,
}

#[get("/api/view_students")]
pub(crate) fn view_students() -> Json<Student> {
    let student = Student {
        name: "John".to_string(),
        surname: "Doe".to_string(),
        age: 20,
    };
    Json(student)
}

#[get("/api/add_student/<name>")]
pub(crate) fn add_student(name: &str) -> String {
    format!("Student {} added", name)
}
