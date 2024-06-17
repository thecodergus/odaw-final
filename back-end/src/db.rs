use postgres::{Client, NoTls};

pub fn get_client() -> Client {
    match Client::connect("host=localhost port=5051 user=admin password=admin", NoTls) {
        Ok(valor) => valor,
        Err(err) => panic!("Deu treta: {}", err),
    }
}

#[cfg(test)]
mod tests {
    use postgres::{Client, NoTls};

    use super::*;

    #[test]
    fn test_get_client() {
        let client = get_client();
        assert!(!client.is_closed());
    }
}
