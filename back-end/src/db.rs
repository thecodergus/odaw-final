use postgres::{Client, NoTls};

pub fn get_client() -> Client {
    match Client::connect("postgresql://postgres:udesc@0.0.0.0:5432/odaw", NoTls) {
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
