// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

use std::collections::HashMap;
use std::sync::Arc;

use arrow_schema::TimeUnit;
use datafusion::arrow::datatypes::{DataType, Field, Schema};
use datafusion::assert_batches_eq;
use datafusion::catalog::TableProvider;
use datafusion::error::DataFusionError;
use datafusion::prelude::SessionContext;
use iceberg::{Catalog, TableIdent};
use iceberg_catalog_rest::RestCatalog;
use iceberg_datafusion::IcebergTableProvider;
use parquet::arrow::PARQUET_FIELD_ID_META_KEY;

use crate::get_shared_containers;

fn metadata_for_field_id(id: i32) -> HashMap<String, String> {
    HashMap::from([(PARQUET_FIELD_ID_META_KEY.to_string(), id.to_string())])
}

#[tokio::test]
async fn test_spark_types() -> Result<(), DataFusionError> {
    let fixture = get_shared_containers();
    let rest_catalog = RestCatalog::new(fixture.catalog_config.clone());

    let table = rest_catalog
        .load_table(&TableIdent::from_strs(["default", "types_test"]).unwrap())
        .await
        .unwrap();

    let ctx = SessionContext::new();

    let table_provider = Arc::new(
        IcebergTableProvider::try_new_from_table(table)
            .await
            .unwrap(),
    );

    let schema = table_provider.schema();

    assert_eq!(
        schema.as_ref(),
        &Schema::new(vec![
            Field::new("cboolean", DataType::Boolean, true).with_metadata(metadata_for_field_id(1)),
            Field::new("ctinyint", DataType::Int32, true).with_metadata(metadata_for_field_id(2)),
            Field::new("csmallint", DataType::Int32, true).with_metadata(metadata_for_field_id(3)),
            Field::new("cint", DataType::Int32, true).with_metadata(metadata_for_field_id(4)),
            Field::new("cbigint", DataType::Int64, true).with_metadata(metadata_for_field_id(5)),
            Field::new("cfloat", DataType::Float32, true).with_metadata(metadata_for_field_id(6)),
            Field::new("cdouble", DataType::Float64, true).with_metadata(metadata_for_field_id(7)),
            Field::new("cdecimal", DataType::Decimal128(8, 2), true)
                .with_metadata(metadata_for_field_id(8)),
            Field::new("cdate", DataType::Date32, true).with_metadata(metadata_for_field_id(9)),
            Field::new(
                "ctimestamp_ntz",
                DataType::Timestamp(TimeUnit::Microsecond, None),
                true
            )
            .with_metadata(metadata_for_field_id(10)),
            Field::new(
                "ctimestamp",
                DataType::Timestamp(TimeUnit::Microsecond, Some(Arc::from("+00:00"))),
                true
            )
            .with_metadata(metadata_for_field_id(11)),
            Field::new("cstring", DataType::Utf8, true).with_metadata(metadata_for_field_id(12)),
            Field::new("cbinary", DataType::LargeBinary, true)
                .with_metadata(metadata_for_field_id(13)),
            Field::new(
                "carray",
                DataType::List(
                    Field::new("element", DataType::Int32, true)
                        .with_metadata(metadata_for_field_id(15))
                        .into(),
                ),
                true
            )
            .with_metadata(metadata_for_field_id(14)),
        ])
    );

    ctx.register_table("types_table", table_provider)?;

    let batches = ctx
        .sql("SELECT * FROM types_table ORDER BY cbigint LIMIT 3")
        .await?
        .collect()
        .await?;
    let expected = [
        "+----------+----------+-----------+------+---------+--------+---------+----------+------------+---------------------+----------------------+---------+----------+-----------+",
        "| cboolean | ctinyint | csmallint | cint | cbigint | cfloat | cdouble | cdecimal | cdate      | ctimestamp_ntz      | ctimestamp           | cstring | cbinary  | carray    |",
        "+----------+----------+-----------+------+---------+--------+---------+----------+------------+---------------------+----------------------+---------+----------+-----------+",
        "| false    | -128     | 0         | 0    | 0       | 0.0    | 0.0     | 0.00     | 1970-01-01 | 1970-01-01T00:00:00 | 1970-01-01T00:00:00Z | 0       | 00000000 | [0, 0, 0] |",
        "| true     | -127     | 1         | 1    | 1       | 1.0    | 1.0     | 0.01     | 1970-01-02 | 1970-01-01T00:00:01 | 1970-01-01T00:00:01Z | 1       | 00000001 | [1, 1, 1] |",
        "| false    | -126     | 2         | 2    | 2       | 2.0    | 2.0     | 0.02     | 1970-01-03 | 1970-01-01T00:00:02 | 1970-01-01T00:00:02Z | 2       | 00000002 | [2, 2, 2] |",
        "+----------+----------+-----------+------+---------+--------+---------+----------+------------+---------------------+----------------------+---------+----------+-----------+",
    ];
    assert_batches_eq!(expected, &batches);
    Ok(())
}

#[tokio::test]
async fn test_pyiceberg_types() -> Result<(), DataFusionError> {
    let fixture = get_shared_containers();
    let rest_catalog = RestCatalog::new(fixture.catalog_config.clone());

    let table = rest_catalog
        .load_table(&TableIdent::from_strs(["default", "types_test_pyiceberg"]).unwrap())
        .await
        .unwrap();

    let ctx = SessionContext::new();

    let table_provider = Arc::new(
        IcebergTableProvider::try_new_from_table(table)
            .await
            .unwrap(),
    );

    let schema = table_provider.schema();

    assert_eq!(
        schema.as_ref(),
        &Schema::new(vec![
            Field::new("cboolean", DataType::Boolean, true).with_metadata(metadata_for_field_id(1)),
            Field::new("cint8", DataType::Int32, true).with_metadata(metadata_for_field_id(2)),
            Field::new("cint16", DataType::Int32, true).with_metadata(metadata_for_field_id(3)),
            Field::new("cint32", DataType::Int32, true).with_metadata(metadata_for_field_id(4)),
            Field::new("cint64", DataType::Int64, true).with_metadata(metadata_for_field_id(5)),
            Field::new("cfloat32", DataType::Float32, true).with_metadata(metadata_for_field_id(6)),
            Field::new("cfloat64", DataType::Float64, true).with_metadata(metadata_for_field_id(7)),
            Field::new("cdecimal128", DataType::Decimal128(8, 2), true)
                .with_metadata(metadata_for_field_id(8)),
            Field::new("cdate32", DataType::Date32, true).with_metadata(metadata_for_field_id(9)),
            Field::new(
                "ctimestamp",
                DataType::Timestamp(TimeUnit::Microsecond, None),
                true
            )
            .with_metadata(metadata_for_field_id(10)),
            Field::new(
                "ctimestamptz",
                DataType::Timestamp(TimeUnit::Microsecond, Some(Arc::from("+00:00"))),
                true
            )
            .with_metadata(metadata_for_field_id(11)),
            Field::new("cutf8", DataType::Utf8, true).with_metadata(metadata_for_field_id(12)),
            Field::new("cbinary", DataType::LargeBinary, true)
                .with_metadata(metadata_for_field_id(13)),
            Field::new(
                "clist",
                DataType::List(
                    Field::new("element", DataType::Int32, true)
                        .with_metadata(metadata_for_field_id(15))
                        .into(),
                ),
                true
            )
            .with_metadata(metadata_for_field_id(14)),
        ])
    );

    ctx.register_table("types_table", table_provider)?;

    let batches = ctx
        .sql("SELECT * FROM types_table LIMIT 3")
        .await?
        .collect()
        .await?;
    let expected = [
        "+----------+-------+--------+--------+--------+----------+----------+-------------+------------+---------------------+----------------------+-------+---------+-----------+",
        "| cboolean | cint8 | cint16 | cint32 | cint64 | cfloat32 | cfloat64 | cdecimal128 | cdate32    | ctimestamp          | ctimestamptz         | cutf8 | cbinary | clist     |",
        "+----------+-------+--------+--------+--------+----------+----------+-------------+------------+---------------------+----------------------+-------+---------+-----------+",
        "| false    | -128  | 0      | 0      | 0      | 0.0      | 0.0      | 0.00        | 1970-01-01 | 1970-01-01T00:00:00 | 1970-01-01T00:00:00Z | 0     | 30      | [0, 0, 0] |",
        "| true     | -127  | 1      | 1      | 1      | 1.0      | 1.0      | 0.01        | 1970-01-02 | 1970-01-01T00:00:01 | 1970-01-01T00:00:01Z | 1     | 31      | [1, 1, 1] |",
        "| false    | -126  | 2      | 2      | 2      | 2.0      | 2.0      | 0.02        | 1970-01-03 | 1970-01-01T00:00:02 | 1970-01-01T00:00:02Z | 2     | 32      | [2, 2, 2] |",
        "+----------+-------+--------+--------+--------+----------+----------+-------------+------------+---------------------+----------------------+-------+---------+-----------+",
    ];
    assert_batches_eq!(expected, &batches);

    // TODO: this isn't OK, and should be fixed with https://github.com/apache/iceberg-rust/issues/813
    let err = ctx
        .sql("SELECT cdecimal128 FROM types_table WHERE cint16 <= 2")
        .await?
        .collect()
        .await
        .unwrap_err();
    assert!(err
        .to_string()
        .contains("Invalid comparison operation: Int16 <= Int32"));

    Ok(())
}
