use dioxus::prelude::*;

#[post("/api/tag")]
pub async fn create(input: String) -> Result<String, ServerFnError> {
    Ok(input)
}

#[get("/api/tag/:id")]
pub async fn read(id: u32) -> Result<u32, ServerFnError> {
    Ok(id)
}

#[get("/api/tag")]
pub async fn read_all() -> Result<String, ServerFnError> {
    Ok("Hello, World!".to_string())
}


#[put("/api/tag/:id")]
pub async fn update(id: u32, input: String) -> Result<String, ServerFnError> {
    Ok(input)
}

#[delete("/api/tag/:id")]
pub async fn delete(id: u32) -> Result<u32, ServerFnError> {
    Ok(id)
}
