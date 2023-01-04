#[cfg(test)]
mod test {

    use test::Bencher;
    use crate::db::{decrypt, encrypt, get, get_path};


    #[bench]
    fn check_data(b: &mut Bencher) {
        // {
        //     let mut cool = HostData::get();
        //     cool.connect = "fedora".to_owned();
        // }
        decrypt(encrypt("hello".to_owned()).unwrap()).unwrap();
        // let fetch= get();
        // let path = get_path();
        // panic!("{:#?}, {:#?}", fetch.unwrap(), path.unwrap());
    }
}