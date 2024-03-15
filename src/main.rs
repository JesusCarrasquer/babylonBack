pub struct User{
   pub uid: String,
   pub email: String,
   pub pw: String,
   pub role: String
}

pub struct LoginRequest{
    pub email:String,
    pub pw: String
}

pub struct LoginResponse{
    pub token: String
}

fn main() {
    println!("Hello, world!");
}
