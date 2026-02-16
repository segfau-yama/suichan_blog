use dioxus::prelude::*;

#[post("/api/article")]
pub async fn create(input: String) -> Result<String, ServerFnError> {
    Ok(input)
}

#[get("/api/article/:id")]
pub async fn read(id: u32) -> Result<u32, ServerFnError> {
    Ok(id)
}

#[get("/api/article")]
pub async fn read_all() -> Result<String, ServerFnError> {
    Ok("Hello, World!".to_string())
}


#[put("/api/article/:id")]
pub async fn update(id: u32, input: String) -> Result<String, ServerFnError> {
    Ok(input)
}

#[delete("/api/article/:id")]
pub async fn delete(id: u32) -> Result<u32, ServerFnError> {
    Ok(id)
}
