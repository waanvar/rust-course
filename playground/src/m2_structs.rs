
#[derive(Debug)]
struct User{
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn chage_username(user: &mut User, new_username: &str) {
    user.username = String::from(new_username);
}

impl User {

    fn increment_sign_in_count(&mut self) {
        self.sign_in_count += 1;
    }

    fn change_email(&mut self, new_email: &str) {
        self.email = String::from(new_email);
    }

    fn chage_username(&mut self, new_username: &str) {
        self.username = String::from(new_username);
    }
    
}

#[cfg(test)]
mod tests { 
    use super::*; 

    #[test]
    fn test_struct() {
        let mut user1 = User {
            username: String::from("user1"),
            email: String::from("test@email.com"),
            sign_in_count: 1,
            active: true,
        };

        //user1.username = String::from("user2");

        chage_username(&mut user1, "user2");

        let mut impl_user1 = User {
            username: String::from("imp_user1"),
            email: String::from("imp_user1@email.com"),
            sign_in_count: 0,
            active: true,
        };

        impl_user1.increment_sign_in_count();
        impl_user1.change_email("impl_user@email.com");
        impl_user1.chage_username("impl_user1");

        dbg!(user1);
        dbg!(impl_user1);
    }
}