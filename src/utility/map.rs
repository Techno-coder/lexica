macro_rules! map (
    { $($key:expr => $value:expr),+ } => {
        {
            let mut map = hashbrown::HashMap::new();
            $(
                map.insert($key, $value);
            )+
            map
        }
     };
);
