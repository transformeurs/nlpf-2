pub struct Candidate {
    pub id: u64,
    pub name: String,
    pub email: String,
    pub password: String,
    pub age: i64,
    pub photo_url: String,
    pub description: String,
}

impl Candidate {
    pub fn new(
        id: u64,
        name: String,
        email: String,
        password: String,
        age: i64,
        photo_url: String,
        description: String,
    ) -> Self {
        Candidate {
            id,
            name,
            email,
            password,
            age,
            photo_url,
            description,
        }
    }

    // TODO methods for password comparison
}
