pub mod client;

pub mod network;

#[cfg(test)]
mod tests {
    use super::client;
    // super는 부모
    // ::는 루트... 하지만 해당 부분에서 use ::client; 를 사용했을 때 동작이 안 된다.
    
    #[test]
    fn it_works() {
        client::connect();
    }
}