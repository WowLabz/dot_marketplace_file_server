#[derive(Debug)]
pub struct NftStorageConfig {
    pub base: String,
    pub route: String 
}

impl Default for NftStorageConfig {
    fn default() -> Self {
        Self{
            base: String::from("https://api.nft.storage"),
            route: String::from("/upload"),
        }
    }
}