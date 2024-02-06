#[derive(Debug, Clone)]
struct Ctx {
    user_id: u64,
}

impl Ctx {
    fn new(user_id: u64) -> Self {
        Self { user_id }
    }
    
}

impl Ctx {
    fn user_id(&self) -> u64 {
        self.user_id
    }
}