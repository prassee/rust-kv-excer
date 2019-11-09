mod store;
use store::KeyValeRepo;

fn main() {
    let mut kv_store = KeyValeRepo::new();
    kv_store.add(format!("key"), format!("value"));
    println!("{}", kv_store.get(format!("key")));
    kv_store.del(format!("key"));
}
