use random_number::random;

pub fn generate_random_number(range: (u16, u16)) -> u16 {
    let start = range.0;
    let end = range.1;
    let number: u16 = random!(start, end);
    number
}


// use once_cell::sync::Lazy;
// use std::collections::HashMap;
// use std::sync::Mutex;
// use std::time::{SystemTime, UNIX_EPOCH};

// const OTP_LENGTH: usize = 4; // Length of the OTP
// const USER_ID_LENGTH: usize = 10; // Length of the user ID

// #[derive(Debug)]
// struct User {
//     phone_no: String,
//     id: String,
// }

// static USERS: Lazy<Mutex<Vec<User>>> = Lazy::new(|| Mutex::new(Vec::new()));
// static OTP_ENTRIES: Lazy<Mutex<HashMap<String, String>>> = Lazy::new(|| {
//     Mutex::new(HashMap::new())
// });

// fn generate_otp(phone_no: &str) {
//     let otp = generate_unique_id(OTP_LENGTH);
//     let mut entries = OTP_ENTRIES.lock().unwrap();
//     entries.insert(phone_no.to_string(), otp.to_string());
//     println!("OTP Entries: {:?}", *entries);
// }

// fn add_user(phone_no: &str) {
//     let id: String = generate_unique_id(USER_ID_LENGTH);
//     let mut users = USERS.lock().unwrap();
//     users.push(User { phone_no: phone_no.to_string(), id });
//     println!("User Entries: {:?}", *users);
// }

// fn generate_unique_id(length: usize) -> String {
//     let duration = SystemTime::now()
//         .duration_since(UNIX_EPOCH)
//         .expect("Time went backwards");
//     let mut id = format!("{}", duration.as_nanos());
//     if id.len() > length {
//         id = id[0..length].to_string();
//     }
//     id
// }

// fn get_otp(phone_no: &str) -> String {
//     let entries = OTP_ENTRIES.lock().unwrap();
//     entries.get(phone_no).cloned().unwrap_or_default()
// }

// fn main() {
//     generate_otp("+919833010430");
//     generate_otp("+919876543210");
//     let otp = get_otp("+919833010430");
//     println!("OTP for +919833010430: {:?}", otp);
// }