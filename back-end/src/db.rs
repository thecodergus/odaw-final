use postgres::{Client, NoTls};

pub fn get_client() -> Client {
    Client::connect("host=localhost port=5051 user=admin password=admin", NoTls).unwrap()
}
