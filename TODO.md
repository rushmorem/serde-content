# serde-content Behavioral Alignment with serde's ContentDeserializer

This tracks work to align serde-content's `Deserializer` with serde's internal `ContentDeserializer` behavior.

Reference file: `de.rs.txt` (copy of serde's `serde/src/private/de.rs` at commit d17902059e77e371d8a7f83ff403f9e760b70f45)

## Completed

- [x] **#1: `deserialize_option` with `Value::Unit`**
  - serde calls `visitor.visit_unit()` for `Content::Unit` (lines 1289-1299)
  - serde-content was falling through to `visitor.visit_some(self)` which failed
  - Fix: Added `Value::Unit => visitor.visit_unit()` case
  - Tests: `deserialize_unit_as_option_none`, `deserialize_map_with_null_values_as_option`, `deserialize_struct_with_optional_map_containing_nulls`

## TODO

- [ ] **#2: `deserialize_unit` - Missing empty Map/Seq handling**
  - serde allows empty Map to deserialize as unit (lines 1304-1317)
  - Needed for internally tagged enums like `{"result":"Success"}` → `Response<()>`

- [ ] **#3: `deserialize_unit_struct` - Missing empty Map/Seq handling**
  - serde allows empty Map/Seq to deserialize as unit struct (lines 1329-1348)
  - Needed for internally tagged enums with unit struct variants

- [ ] **#4: `deserialize_char` - Missing String fallback**
  - serde allows String/Str content to be deserialized via `deserialize_char` (lines ~1235-1246)

- [ ] **#5: `deserialize_str`/`deserialize_string` - Missing Bytes fallback**
  - serde allows ByteBuf/Bytes to be deserialized as string (lines ~1267-1285)

- [ ] **#6: `deserialize_bytes`/`deserialize_byte_buf` - Missing String and Seq fallback**
  - serde allows String/Str and Seq to be deserialized as bytes (lines ~1267-1285)

- [ ] **#7: `deserialize_struct` - Missing Seq fallback**
  - serde allows Seq to deserialize as struct (lines ~1403-1418)
  - Needed for tuple-style struct deserialization

## Process for each item

1. Create a test case that fails describing the issue
2. Implement the fix
3. Validate that the test passes
4. Commit with message citing relevant serde line numbers
