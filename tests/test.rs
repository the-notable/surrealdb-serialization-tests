use rust_decimal::Decimal;
use serde::Serialize;
use surrealdb_serialization_tests::test_surreal_serialization;

// (test_type, test_value, surreal_schema_type, sdk (p)ass or (f)ail, raw query (p)ass or (f)ail)
test_surreal_serialization!([
    (u8, u8::MIN, int, p),
    (u8, u8::MIN, flt, f),
    (u8, u8::MIN, dec, f),
    (u8, u8::MIN, str, f),
    (u8, u8::MIN, def, p),
    (u8, u8::MAX, int, p),
    (u8, u8::MAX, flt, f),
    (u8, u8::MAX, dec, f),
    (u8, u8::MAX, str, f),
    (u8, u8::MAX, def, p),
    
    (i8, i8::MIN, int, p),
    (i8, i8::MIN, flt, f),
    (i8, i8::MIN, dec, f),
    (i8, i8::MIN, str, f),
    (i8, i8::MIN, def, p),
    (i8, i8::MAX, int, p),
    (i8, i8::MAX, flt, f),
    (i8, i8::MAX, dec, f),
    (i8, i8::MAX, str, f),
    (i8, i8::MAX, def, p),
    
    (u16, u16::MIN, int, p),
    (u16, u16::MIN, flt, f),
    (u16, u16::MIN, dec, f),
    (u16, u16::MIN, str, f),
    (u16, u16::MIN, def, p),
    (u16, u16::MAX, int, p),
    (u16, u16::MAX, flt, f),
    (u16, u16::MAX, dec, f),
    (u16, u16::MAX, str, f),
    (u16, u16::MAX, def, p),
    
    (i16, i16::MIN, int, p),
    (i16, i16::MIN, flt, f),
    (i16, i16::MIN, dec, f),
    (i16, i16::MIN, str, f),
    (i16, i16::MIN, def, p),
    (i16, i16::MAX, int, p),
    (i16, i16::MAX, flt, f),
    (i16, i16::MAX, dec, f),
    (i16, i16::MAX, str, f),
    (i16, i16::MAX, def, p),
    
    (u32, u32::MIN, int, p),
    (u32, u32::MIN, flt, f),
    (u32, u32::MIN, dec, f),
    (u32, u32::MIN, str, f),
    (u32, u32::MIN, def, p),
    (u32, u32::MAX, int, p),
    (u32, u32::MAX, flt, f),
    (u32, u32::MAX, dec, f),
    (u32, u32::MAX, str, f),
    (u32, u32::MAX, def, p),
    
    (i32, i32::MIN, int, p),
    (i32, i32::MIN, flt, f),
    (i32, i32::MIN, dec, f),
    (i32, i32::MIN, str, f),
    (i32, i32::MIN, def, p),
    (i32, i32::MAX, int, p),
    (i32, i32::MAX, flt, f),
    (i32, i32::MAX, dec, f),
    (i32, i32::MAX, str, f),
    (i32, i32::MAX, def, p),
    
    (u64, u64::MIN, int, p),
    (u64, u64::MIN, flt, f),
    (u64, u64::MIN, dec, f),
    (u64, u64::MIN, str, f),
    (u64, u64::MIN, def, p),
    (u64, u64::MAX, int, f),
    (u64, u64::MAX, flt, f),
    (u64, u64::MAX, dec, f),
    (u64, u64::MAX, str, f),
    (u64, u64::MAX, def, f),
    
    (i64, i64::MIN, int, p),
    (i64, i64::MIN, flt, f),
    (i64, i64::MIN, dec, f),
    (i64, i64::MIN, str, f),
    (i64, i64::MIN, def, p),
    (i64, i64::MAX, int, p),
    (i64, i64::MAX, flt, f),
    (i64, i64::MAX, dec, f),
    (i64, i64::MAX, str, f),
    (i64, i64::MAX, def, p),
    
    (u128, u128::MIN, int, p),
    (u128, u128::MIN, flt, f),
    (u128, u128::MIN, dec, f),
    (u128, u128::MIN, str, f),
    (u128, u128::MIN, def, f), // Api(FromValue { value: Array(Array([Object(Object({"id": Thing(Thing { tb: "test", id: String("5vse9sjy9t8a8d2a4i31") }), "test_value": Number(Decimal(0))}))])), error: "invalid type: string \"0\", expected u128" })
    (u128, u128::MAX, int, f),
    (u128, u128::MAX, flt, f),
    (u128, u128::MAX, dec, f),
    (u128, u128::MAX, str, f),
    (u128, u128::MAX, def, f),
    
    (i128, i128::MIN, int, f),
    (i128, i128::MIN, flt, f),
    (i128, i128::MIN, dec, f),
    (i128, i128::MIN, str, f),
    (i128, i128::MIN, def, f),
    (i128, i128::MAX, int, f),
    (i128, i128::MAX, flt, f),
    (i128, i128::MAX, dec, f),
    (i128, i128::MAX, str, f),
    (i128, i128::MAX, def, f),

    (f32, f32::MIN, int, f),
    (f32, f32::MIN, flt, p),    
    (f32, f32::MIN, dec, f),
    (f32, f32::MIN, str, f),
    (f32, f32::MIN, def, p),
    (f32, f32::MAX, int, f),
    (f32, f32::MAX, flt, p),
    (f32, f32::MAX, dec, f),
    (f32, f32::MAX, str, f),
    (f32, f32::MAX, def, p),

    (f64, f64::MIN, int, f),
    (f64, f64::MIN, flt, p),
    (f64, f64::MIN, dec, f),
    (f64, f64::MIN, str, f),
    (f64, f64::MIN, def, p),
    (f64, f64::MAX, int, f),
    (f64, f64::MAX, flt, p),
    (f64, f64::MAX, dec, f),
    (f64, f64::MAX, str, f),
    (f64, f64::MAX, def, p),

    (Decimal, Decimal::MIN, int, f),
    (Decimal, Decimal::MIN, flt, f),
    // SDK TEST:
    // Db(FieldCheck { thing: "test:ngja3b4ez0m2x1uhwsn1", value: "'-79228162514264337593543950335'", 
    // field: Idiom([Field(Ident("test_value"))]), check: "decimal" })
    (Decimal, Decimal::MIN, dec, f),
    (Decimal, Decimal::MIN, str, p),
    (Decimal, Decimal::MIN, def, p),
    
    (Decimal, Decimal::MAX, int, f),
    (Decimal, Decimal::MAX, flt, f),
    // SDK TEST:
    // Db(FieldCheck { thing: "test:ngja3b4ez0m2x1uhwsn1", value: "'79228162514264337593543950335'", 
    // field: Idiom([Field(Ident("test_value"))]), check: "decimal" })
    (Decimal, Decimal::MAX, dec, f),
    (Decimal, Decimal::MAX, str, p),
    (Decimal, Decimal::MAX, def, p)
]);

#[tokio::test]
async fn it_works_w_schema() {
    use surrealdb::Surreal;
    use surrealdb::sql::Thing;
    use surrealdb::engine::local::Mem;
    use serde::{Serialize, Deserialize};

    let db = Surreal::new::<Mem>(()).await.unwrap();
    db.use_ns("test").use_db("test").await.unwrap();

    let schema_sql = r#"
        DEFINE TABLE test_table SCHEMAFULL;
        DEFINE FIELD test_value ON TABLE test_table TYPE int;
    "#;
    db.query(schema_sql).await.unwrap();

    #[derive(Serialize)]
    struct RecordIn {
        test_value: u8
    }

    #[derive(Deserialize)]
    struct RecordOut {
        id: Thing,
        test_value: u8
    }

    let src = RecordIn { test_value: u8::MAX };

    let created: Vec<RecordOut> = db
        .create("test_table")
        .content(&src)
        .await
        .expect("Error creating record");

    let target = created.first().unwrap();
    assert_eq!(src.test_value, target.test_value);
}

#[tokio::test]
async fn it_works_wo_schema() {
    use surrealdb::Surreal;
    use surrealdb::sql::Thing;
    use surrealdb::engine::local::Mem;
    use serde::{Serialize, Deserialize};

    let db = Surreal::new::<Mem>(()).await.unwrap();
    db.use_ns("test").use_db("test").await.unwrap();

    #[derive(Serialize)]
    struct RecordIn {
        test_value: u8
    }

    #[derive(Deserialize)]
    struct RecordOut {
        id: Thing,
        test_value: u8
    }

    let src = RecordIn { test_value: u8::MAX };

    let created: Vec<RecordOut> = db
        .create("test_table")
        .content(&src)
        .await
        .expect("Error creating record");

    let target = created.first().unwrap();
    assert_eq!(src.test_value, target.test_value);
}