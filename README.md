# SurrealDB Rust SDK Serialization/Deserialization Testing

### TLDR
1. ``u128::MIN `` serialization behavior is inconsistent with other Rust integer types.
2. ``rust_decimal::Decimal`` serializes to ``string`` type and fails when attempting to serialize to ``decimal`` type.

### Questions
1. When creating types to interface with a SurrealDB table containing a field with the ``decimal`` type, which Rust
should be used?
2. Which other Rust/SurrealDB type combinations should be added to tests?
3. Are tests interacting with SurrealDB SDK correctly?

## Testing
The serialization and deserialization of Rust primitive number types, as well as the ``Decimal`` type from the 
``rust_decimal`` crate, to and from SurrealDB was tested using the SurrealDB Rust SDK.

For each Rust type (and ``Decimal``), ten tests were generated: five each for the ``MIN`` and ``MAX`` values for each 
type (i.e. ``u8::MIN`` and ``u8::MAX``). 

Of the five tests generated for each type and value combination, one consisted of serializing and inserting into 
a "schemaless" table, and the remaining four tests first defined "schemafull" tables, one test for each of the
``int``, ``float``, ``decimal``, and ``string`` SurrealDB types.

Example generated tests:
- [Schemafull (int, float, decimal, string)](#example-generated-schemafull-test-int-float-decimal-string)
- [Schemaless (default)](#example-generated-schemaless-test-default)

Generated tests which ``panic!`` are marked ``#[should_panic]``. 

## Results
Most tests ran (or didn't) as expected. 

However, there are two items which stand out:

1. When tested against a "schemaless" table, the deserialization of ``u128::MIN`` fails. It passes, however, when
tested against a "schemafull" table with where the corresponding field is defined ``int``. This is inconsistent with
the deserialization behavior of ``MIN`` for other unsigned integer Rust types, particularly ``u64::MIN``, which can
also represent values outside the range of SurrealDB's ``int`` type.\
\
``Api(FromValue { value: Array(Array([Object(Object({"id": Thing(Thing { tb: "test", id: String("5vse9sjy9t8a8d2a4i31") }), "test_value": Number(Decimal(0))}))])), error: "invalid type: string \"0\", expected u128" })``


2. The ``Decimal`` type from the ``rust_decimal`` crate has initially unexpected, but understandable, behavior. It would 
be expected that the ``Decimal`` type would serialize to the SurrealDB ``decimal`` type, but it instead serializes 
to the ``string`` type, and fails when attempting to serialize to a "schemafull" table with a corresponding field
defined ``decimal``.\
\
Not only is this behavior initially unexpected because both types are decimal, but the confusion is compounded when
inspecting the docs for information on serialization/deserialization targets, and (mistakenly) thinking that 
``surrealdb::sql::Value``, which implements ``From<Decimal> for Value`` as ``Value::from(Decimal) -> Value::Number(Number::Decimal)``,
may be used as a guide.\
\
``Db(FieldCheck { thing: "test:ngja3b4ez0m2x1uhwsn1", value: "'79228162514264337593543950335'", field: Idiom([Field(Ident("test_value"))]), check: "decimal" })``

The table below summarizes the test results.

| Rust Type               | Test Value       |    default    | int | float | decimal | string |
|-------------------------|------------------|:-------------:|:---:|:-----:|:-------:|:------:|
| ``u8``                  | ``u8::MIN``      |       ✅       |  ✅  |   ❌   |    ❌    |   ❌    |
|                         | ``u8::MAX``      |       ✅       |  ✅  |   ❌   |    ❌    |   ❌    |
| ``i8``                  | ``i8::MIN``      |       ✅       |  ✅  |   ❌   |    ❌    |   ❌    |
|                         | ``i8::MAX``      |       ✅       |  ✅  |   ❌   |    ❌    |   ❌    |
| ``u16``                 | ``u16::MIN``     |       ✅       |  ✅  |   ❌   |    ❌    |   ❌    |
|                         | ``u16::MAX``     |       ✅       |  ✅  |   ❌   |    ❌    |   ❌    |
| ``i16``                 | ``i16::MIN``     |       ✅       |  ✅  |   ❌   |    ❌    |   ❌    |
|                         | ``i16::MAX``     |       ✅       |  ✅  |   ❌   |    ❌    |   ❌    |
| ``u32``                 | ``u32::MIN``     |       ✅       |  ✅  |   ❌   |    ❌    |   ❌    |
|                         | ``u32::MAX``     |       ✅       |  ✅  |   ❌   |    ❌    |   ❌    |
| ``i32``                 | ``i32::MIN``     |       ✅       |  ✅  |   ❌   |    ❌    |   ❌    |
|                         | ``i32::MAX``     |       ✅       |  ✅  |   ❌   |    ❌    |   ❌    |
| ``u64``                 | ``u64::MIN``     |       ✅       |  ✅  |   ❌   |    ❌    |   ❌    |
|                         | ``u64::MAX``     |       ❌       |  ❌  |   ❌   |    ❌    |   ❌    |
| ``i64``                 | ``i64::MIN``     |       ✅       |  ✅  |   ❌   |    ❌    |   ❌    |
|                         | ``i64::MAX``     |       ✅       |  ✅  |   ❌   |    ❌    |   ❌    |
| ``u128``                | ``u128::MIN``    | ❌<sup>1</sup> |  ✅  |   ❌   |    ❌    |   ❌    |
|                         | ``u128::MAX``    |       ❌       |  ❌  |   ❌   |    ❌    |   ❌    |
| ``i128``                | ``i128::MIN``    |       ✅       |  ✅  |   ❌   |    ❌    |   ❌    |
|                         | ``i128::MAX``    |       ✅       |  ✅  |   ❌   |    ❌    |   ❌    |
| ``f32``                 | ``f32::MIN``     |       ✅       |  ❌  |   ✅   |    ❌    |   ❌    |
|                         | ``f32::MAX``     |       ✅       |  ❌  |   ✅   |    ❌    |   ❌    |
| ``f64``                 | ``f64::MIN``     |       ✅       |  ❌  |   ✅   |    ❌    |   ❌    |
|                         | ``f64::MAX``     |       ✅       |  ❌  |   ✅   |    ❌    |   ❌    |
| ``Decimal``<sup>2</sup> | ``Decimal::MIN`` |       ✅       |  ❌  |   ❌   |    ❌    |   ✅    |
|                         | ``Decimal::MAX`` |       ✅       |  ❌  |   ❌   |    ❌    |   ✅    |


## Example generated schemafull test (int, float, decimal, string)
```rust
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
```

## Example generated schemaless test (default)
```rust
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
```