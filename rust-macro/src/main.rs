
macro_rules! hashmap {
    ($($key:expr => $value:expr),*) => {
        {
            let mut _map = ::std::collections::HashMap::new();
            $(
                _map.insert($key, $value);
            )*
            _map
        }
    };
}

fn main() {
    let map = hashmap!{
        "a" => 1,
        "b" => 2,
    };
    println!("{:?}", map);
}
