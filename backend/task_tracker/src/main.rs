
// A struct that represent the task.
#[derive(serde::Deserialize, serde::Serialize)]
struct Task {
    id: i32,
    description: String,
    status: String,
    created_at: String,
    updated_at: String
}



fn main() {
    println!("Hello, world!");
}
