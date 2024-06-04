#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

impl User {
    fn increment_signin_count(&mut self) {
        self.sign_in_count += 1;
    }

    fn new_email(&mut self, new_email: &str) {
        self.email = new_email.to_string();
    }
    fn change_username(&mut self, new_username: &str) {
        self.username = new_username.to_string();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_structs() {
        let mut user_1: User = User {
            username: String::from("someusername1"),
            email: String::from("someone@example.com"),
            sign_in_count: 1,
            active: true,
        };

        //  user_1.username = String::from("pants");
        //  dbg!(user_1);

        user_1.change_username("This is a new username");
        user_1.increment_signin_count();
        user_1.new_email("bollocks@com.com");
        dbg!(user_1);
    }
}
