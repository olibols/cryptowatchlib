mod api;

fn main() {
    let asdf = api::CryptoWatch::new();
    match asdf.get_price("Binance", "BTCUSDT") {
        Ok(T) => {
            println!("{}", T);
        }

        Err(E) => {
            println!("L");
        }
    }    
}
