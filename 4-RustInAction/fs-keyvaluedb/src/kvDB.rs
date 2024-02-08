use libactionkv::ActionKV;
use std::collections::HashMap;

#[cfg(target_os = "windows")]             // Conditional compiling for windows (so that we can have correct extension)
const USAGE: &str = "
Usage:
    akv_mem.exe FILE get KEY
    akv_mem.exe FILE delete KEY
    akv_mem.exe FILE insert KEY VALUE
    akv_mem.exe FILE update KEY VALUE
";

#[cfg(not(target_os = "windows"))]
const USAGE: &str = "
Usage:
    akv_mem FILE get KEY
    akv_mem FILE delete KEY
    akv_mem FILE insert KEY VALUE
    akv_mem FILE update KEY VALUE
";

type ByteStr = [u8];
type ByteString = Vec<u8>;

fn store_index_on_disk(store: &mut ActionKV, index_key: &[u8]) {
    store.index.remove(index_key);
    let index_as_bytes = bincode::serialize(&store.index).unwrap();
    store.index = std::collections::HashMap::new();
    store.insert(index_key, &index_as_bytes).unwrap();
  }

fn main() {
    const INDEX_KEY: &[u8] = b"+index";

    let args: Vec<String> = std::env::args().collect();
    let fname = args.get(1).expect(&USAGE);
    let action:&str = args.get(2).expect(&USAGE).as_ref();
    let key:&[u8] = args.get(3).expect(&USAGE).as_ref();
    let maybe_value = args.get(4);

    let path = std::path::Path::new(&fname);
    let mut store = ActionKV::open(path).expect("unable to open file");
    store.load().expect("unable to load data");

    match action {
        "get" => {
            let index_as_bytes = store.get(INDEX_KEY).unwrap().unwrap();
            let index_decoded = bincode::deserialize(&index_as_bytes);

            let index: HashMap<ByteString, u64> = index_decoded.unwrap();

            //now to retrieve the information, we have to get the idx first (idx=location of disk)
            match index.get(key) {
                None => eprintln!("{:?} not found", key),
                Some(&i) => {
                    let kv = store.get_at(i).unwrap();
                    println!("{:?}", kv.value)               
                }   
            }
        }
    
        "delete" => store.delete(key).unwrap(),
    
        "insert" => {
          let value = maybe_value.expect(&USAGE).as_ref();
          store.insert(key, value).unwrap();
          store_index_on_disk(&mut store, INDEX_KEY); //update idx when insert
        }
    
        "update" => {
          let value = maybe_value.expect(&USAGE).as_ref();
          store.update(key, value).unwrap();
          store_index_on_disk(&mut store, INDEX_KEY); //update idx when update
        }
    
        _ => eprintln!("{}", &USAGE),
      }
}