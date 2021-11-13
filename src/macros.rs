use std::collections::HashMap;
macro_rules! avec {
    () => {
        Vec::<i32>::new()
    };
    ( $($e: expr),* $(,)? ) => {{
        let mut v = Vec::new();
        $(v.push($e);)*
        v
    }};
    ($e:expr; $count:expr) => {{
        let cnt = $count;
        let e = $e;
        let mut v = Vec::with_capacity(cnt);
        v.resize(cnt, e);
        v
    }};
}

#[macro_export]
macro_rules! counter {
    ($($e: expr),*) => {
        <[()]>::len(&[ $($crate::counter![$e]),* ])
    };
    ($e:expr) => { () };
}

#[test]
fn test_avec() {
    assert_eq!(avec![], vec![]);
    assert_eq!(avec![1], vec![1]);
    assert_eq!(avec![1, 2], vec![1, 2]);
    assert_eq!(avec![1, 2,], vec![1, 2,]);
    assert_eq!(avec![1; 3], vec![1, 1, 1]);
}
macro_rules! amap {
    () => {
        HashMap::<&str, i32>::new()
    };
    ($($key:expr => $val:expr),* $(,)?) => {{
        let mut m = HashMap::new();
        $(m.insert($key, $val);)*
        m
    }};
}

#[test]
fn test_amap() {
    assert_eq!(amap!(), HashMap::new());
    let mut m = HashMap::new();
    m.insert("a", 1);
    assert_eq!(amap!("a" => 1), m);
    m.insert("b", 2);
    assert_eq!(amap!("a" => 1, "b" => 2,), m);
}