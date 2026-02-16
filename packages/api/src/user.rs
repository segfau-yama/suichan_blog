use dioxus::prelude::*;

#[post("/api/user")]
pub async fn create(input: String) -> Result<String, ServerFnError> {
    Ok(input)
}

#[get("/api/user/:id")]
pub async fn read(id: u32) -> Result<u32, ServerFnError> {
    Ok(id)
}

#[get("/api/user")]
pub async fn read_all() -> Result<String, ServerFnError> {
    Ok("Hello, World!".to_string())
}


#[put("/api/user/:id")]
pub async fn update(id: u32, input: String) -> Result<String, ServerFnError> {
    Ok(input)
}

#[delete("/api/user/:id")]
pub async fn delete(id: u32) -> Result<u32, ServerFnError> {
    Ok(id)
}
