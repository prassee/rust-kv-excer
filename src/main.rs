mod store;
use store::KeyValeRepo;

fn main() {
    let mut kv_store = KeyValeRepo::new();
    let k = format!("key");
    kv_store.add(k, format!("value"));
    println!("{}", kv_store.get(format!("key")));
    kv_store.del(format!("key"));
}
