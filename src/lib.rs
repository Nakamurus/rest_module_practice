pub mod client;

pub mod network;


#[cfg(test)]
mod tests {
    use super::client;

    #[test]
    fn it_works() {
        // superを使って、現在のモジュールから階層を1つ上がる
        // ::client::connect();のような先頭のコロンは、フルパス列挙に使う
        client::connect();
    }
}
