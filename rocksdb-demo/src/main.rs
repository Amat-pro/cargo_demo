use rocksdb::{ColumnFamilyDescriptor, DB, Options};

/// rocksdb: kv

fn main() {
    println!("Hello, rocksdb!");

    open_default_db();

    open_db_with_single_column_family();
}

fn open_default_db() {
    const PATH: &str = "_path_for_rocksdb_storage";

    {
        let db = DB::open_default(PATH).unwrap();
        db.put(b"my key", b"my value").unwrap();
        match db.get(b"my key") {
            Ok(Some(value)) => println!("open_default_db: retrieved value {}", String::from_utf8(value).unwrap()),
            Ok(None) => println!("open_default_db: value not found"),
            Err(e) => println!("open_default_db: operational problem encountered: {}", e),
        }
        db.delete(b"my key").unwrap();
    }

    let _ = DB::destroy(&Options::default(), PATH);
}

fn open_db_with_single_column_family() {
    let path = "_path_for_rocksdb_storage_with_cfs";
    let mut cf_opts = Options::default();
    cf_opts.set_max_write_buffer_number(16);
    let cf1 = ColumnFamilyDescriptor::new("cf1", cf_opts.clone());
    let cf2 = ColumnFamilyDescriptor::new("cf2", cf_opts.clone());

    let mut db_opts = Options::default();
    db_opts.create_missing_column_families(true);
    db_opts.create_if_missing(true);
    {
        let db = DB::open_cf_descriptors(&db_opts, path, vec![cf1, cf2]).unwrap();

        // put cf
        if let Some(cf) = db.cf_handle("cf1") {
            db.put_cf(cf, b"my key", b"my value").unwrap();

            // get cf
            match db.get_cf(cf, b"my key") {
                Ok(Some(value)) => println!("open_db_with_single_column_family: retrieved value {}", String::from_utf8(value).unwrap()),
                Ok(None) => println!("open_db_with_single_column_family: value not found"),
                Err(e) => println!("open_db_with_single_column_family: operational problem encountered: {}", e),
            }
        }
    }
    let _ = DB::destroy(&db_opts, path);
}




