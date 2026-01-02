# OFML Table Schemas Quick Reference

Complete table schema definitions for all common OFML EBase tables.

---

## Standard OCD Tables

### ocd_version

Version and metadata information.

| Column | Type | Offset | Description |
|--------|------|--------|-------------|
| format_version | string_ref | 0 | Format version (e.g., "4.3") |
| rel_coding | string_ref | 4 | Relation coding scheme |
| data_version | string_ref | 8 | Data version string |
| date_from | string_ref | 12 | Validity start date |
| date_to | string_ref | 16 | Validity end date |
| region | string_ref | 20 | Region code |
| varcond_var | string_ref | 24 | Variant condition variable |
| placeholder_on | uint8 | 28 | Placeholder enabled |
| tables | string_ref | 32 | Table list |
| comment | string_ref | 36 | Comments |

**Record Size**: 40 bytes
**Typical Records**: 1

---

### ocd_article

Article (product) definitions.

| Column | Type | Offset | Description |
|--------|------|--------|-------------|
| article_nr | string_ref | 0 | Article number (unique ID) |
| art_type | string_ref | 4 | Article type ("B"=base, "V"=variant) |
| manufacturer | string_ref | 8 | Manufacturer name |
| series | string_ref | 12 | Product series name |
| short_textnr | string_ref | 16 | Short text reference |
| long_textnr | string_ref | 20 | Long text reference |
| rel_obj | uint32 | 24 | Relation object ID |
| fast_supply | uint16 | 28 | Fast supply flag |
| discountable | uint8 | 30 | Discount allowed (0/1) |
| order_unit | string_ref | 32 | Order unit ("ST"=piece) |
| scheme_id | string_ref | 36 | Variant code scheme ID |

**Record Size**: 40 bytes
**Typical Records**: 2-2000

---

### ocd_artshorttext

Short article descriptions (multilingual).

| Column | Type | Offset | Description |
|--------|------|--------|-------------|
| textnr | string_ref | 4 | Text number (key) |
| language | string_ref | 8 | Language code ("DE", "EN", etc.) |
| line_nr | uint8 | 12 | Line number (for multi-line) |
| line_fmt | string_ref | 16 | Format code (optional) |
| text | string_ref | 20 | Actual text content |

**Record Size**: 24 bytes
**Typical Records**: 12-2000

---

### ocd_artlongtext

Long article descriptions (multilingual).

Same schema as `ocd_artshorttext`.

**Record Size**: 24 bytes
**Typical Records**: 24-1000

---

### ocd_price

Pricing information (base prices and surcharges).

| Column | Type | Offset | Description |
|--------|------|--------|-------------|
| article_nr | string_ref | 4 | Article number |
| var_cond | string_ref | 8 | Variant condition code |
| price_type | string_ref | 12 | Price type ("P"=sales) |
| price_level | string_ref | 16 | "B"=base, "X"=surcharge |
| price_rule | string_ref | 20 | Calculation rule |
| price_textnr | string_ref | 24 | Price description reference |
| price | float64 | 32 | Price amount |
| is_fix | uint8 | 40 | Fixed price (0/1) |
| currency | string_ref | 44 | Currency code ("EUR", "USD") |
| date_from | string_ref | 48 | Validity start |
| date_to | string_ref | 52 | Validity end |
| scale_quantity | uint16 | 56 | Quantity discount threshold |
| rounding_id | string_ref | 60 | Rounding rule reference |

**Record Size**: 64 bytes
**Typical Records**: 60-10000+

**Note**: Knoll uses 56-byte variant without `scale_quantity` and `rounding_id`.

---

### ocd_pricetext

Price descriptions (multilingual).

Same schema as `ocd_artshorttext`.

**Record Size**: 24 bytes (Sedus) or 20 bytes (Knoll)
**Typical Records**: 60-1000

---

### ocd_propertyclass

Property classes (configuration groups).

| Column | Type | Offset | Description |
|--------|------|--------|-------------|
| article_nr | string_ref | 4 | Article number |
| pos_class | uint16 | 8 | Position/order |
| prop_class | string_ref | 12 | Property class ID |
| textnr | string_ref | 16 | Description reference |
| rel_obj | uint32 | 20 | Relation object ID |

**Record Size**: 24 bytes
**Typical Records**: 4-500

---

### ocd_property

Property definitions within a class.

| Column | Type | Offset | Description |
|--------|------|--------|-------------|
| prop_class | string_ref | 4 | Property class ID |
| property | string_ref | 8 | Property ID (e.g., "COLOR") |
| pos_prop | uint16 | 12 | Position/order |
| prop_textnr | string_ref | 16 | Label reference |
| rel_obj | uint32 | 20 | Relation object ID |
| prop_type | string_ref | 24 | Type ("S"=selection, "I"=integer, etc.) |
| digits | uint16 | 28 | Max digits |
| dec_digits | uint8 | 30 | Decimal places |
| need_input | uint8 | 31 | Required input (0/1) |
| add_values | uint8 | 32 | Allow additional values |
| restrictable | uint8 | 33 | Can be restricted |
| multi_option | uint8 | 34 | Multiple selection allowed |
| scope | string_ref | 36 | Scope/visibility |
| txt_control | string_ref | 40 | Text control type |
| hint_text_id | string_ref | 44 | Hint text reference |

**Record Size**: 48 bytes
**Typical Records**: 26-500

---

### ocd_propertyvalue

Available values for properties.

| Column | Type | Offset | Description |
|--------|------|--------|-------------|
| prop_class | string_ref | 0 | Property class ID |
| property | string_ref | 4 | Property ID |
| pos_pval | uint16 | 8 | Position/order |
| pval_textnr | string_ref | 12 | Value label reference |
| rel_obj | uint32 | 16 | Relation object ID |
| is_default | uint8 | 20 | Default value (0/1) |
| suppress_txt | uint8 | 21 | Hide text (0/1) |
| op_from | string_ref | 24 | Range operator (">=", etc.) |
| value_from | string_ref | 28 | Value or range start |
| op_to | string_ref | 32 | Range end operator |
| value_to | string_ref | 36 | Range end value |
| raster | string_ref | 40 | Increment value |
| date_from | string_ref | 44 | Availability start |
| date_to | string_ref | 48 | Availability end |

**Record Size**: 52 bytes
**Typical Records**: 100-10000+

---

### ocd_propertytext

Property labels (multilingual).

Same schema as `ocd_artshorttext`.

**Record Size**: 24 bytes (Sedus) or 20 bytes (Knoll)
**Typical Records**: 400-2000

---

### ocd_propvaluetext

Property value labels (multilingual).

Same schema as `ocd_artshorttext`.

**Record Size**: 24 bytes (Sedus) or 20 bytes (Knoll)
**Typical Records**: 1000-50000+

---

### ocd_relation

Relation definitions (constraints, dependencies).

| Column | Type | Offset | Description |
|--------|------|--------|-------------|
| rel_name | string_ref | 4 | Relation name |
| rel_blocknr | uint16 | 8 | Block number |
| rel_block | string_ref | 12 | Block data (encoded) |

**Record Size**: 16 bytes
**Typical Records**: 100-50000

---

### ocd_relationobj

Relation objects (relationship instances).

| Column | Type | Offset | Description |
|--------|------|--------|-------------|
| rel_obj | uint32 | 0 | Relation object ID |
| position | uint16 | 4 | Position/order |
| rel_name | string_ref | 8 | Relation name reference |
| rel_type | string_ref | 12 | Relation type |
| rel_domain | string_ref | 16 | Relation domain |

**Record Size**: 20 bytes
**Typical Records**: 100-10000

---

### ocd_packaging

Packaging specifications.

| Column | Type | Offset | Description |
|--------|------|--------|-------------|
| article_nr | string_ref | 4 | Article number |
| var_cond | string_ref | 8 | Variant condition |
| width | string_ref | 12 | Package width |
| height | string_ref | 16 | Package height |
| depth | string_ref | 20 | Package depth |
| measure_unit | string_ref | 24 | Measurement unit ("MM", "CM") |
| volume | string_ref | 28 | Package volume |
| volume_unit | string_ref | 32 | Volume unit ("M3") |
| tara_weight | string_ref | 36 | Tare weight |
| net_weight | string_ref | 40 | Net weight |
| weight_unit | string_ref | 44 | Weight unit ("KG") |
| items_per_unit | string_ref | 48 | Items per package |
| pack_units | string_ref | 52 | Packaging units |

**Record Size**: 56 bytes
**Typical Records**: 0-100

---

### ocd_articletaxes

Article tax assignments.

| Column | Type | Offset | Description |
|--------|------|--------|-------------|
| article_nr | string_ref | 4 | Article number |
| tax_id | string_ref | 8 | Tax scheme ID |
| date_from | string_ref | 12 | Validity start |
| date_to | string_ref | 16 | Validity end |

**Record Size**: 20 bytes
**Typical Records**: 0-50

---

### ocd_taxscheme

Tax scheme definitions.

| Column | Type | Offset | Description |
|--------|------|--------|-------------|
| tax_id | string_ref | 0 | Tax ID |
| country | string_ref | 4 | Country code |
| region | string_ref | 8 | Region/state |
| number | uint16 | 12 | Tax number |
| tax_type | string_ref | 16 | Tax type |
| tax_category | string_ref | 20 | Tax category |

**Record Size**: 24 bytes
**Typical Records**: 0-5

---

### ocd_codescheme

Variant code generation schemes.

| Column | Type | Offset | Description |
|--------|------|--------|-------------|
| scheme_id | string_ref | 4 | Scheme ID |
| scheme | string_ref | 8 | Scheme definition |
| varcode_sep | string_ref | 12 | Variant code separator |
| value_sep | string_ref | 16 | Value separator |
| visibility | string_ref | 20 | Visibility rules |
| invisible_char | string_ref | 24 | Hidden character |
| unselect_char | string_ref | 28 | Unselected character |
| trim | uint8 | 32 | Trim whitespace (0/1) |
| mo_sep | string_ref | 36 | Multi-option separator |
| mo_bracket | string_ref | 40 | Multi-option brackets |

**Record Size**: 44 bytes
**Typical Records**: 0-10

---

### ocd_rounding

Price rounding rules.

| Column | Type | Offset | Description |
|--------|------|--------|-------------|
| id | string_ref | 4 | Rounding ID |
| nr | uint16 | 8 | Rule number |
| min | string_ref | 12 | Minimum value |
| max | string_ref | 16 | Maximum value |
| type | string_ref | 20 | Rounding type |
| precision | float64 | 24 | Precision value |
| add_before | float64 | 32 | Add before rounding |
| add_after | float64 | 40 | Add after rounding |

**Record Size**: 48 bytes
**Typical Records**: 0-20

---

### ocd_artbase

Article base properties (optional).

| Column | Type | Offset | Description |
|--------|------|--------|-------------|
| article_nr | string_ref | 4 | Article number |
| prop_class | string_ref | 8 | Property class |
| property | string_ref | 12 | Property ID |
| prop_value | string_ref | 16 | Base value |

**Record Size**: 20 bytes
**Typical Records**: 0-100

---

## Extended OCD Tables (Vitra, etc.)

### ocd_composite

Composite article definitions.

| Column | Type | Offset | Description |
|--------|------|--------|-------------|
| composite_nr | string_ref | 4 | Composite article number |
| component_nr | string_ref | 8 | Component article number |
| quantity | float64 | 12 | Component quantity |
| unit | string_ref | 20 | Unit of measure |

**Record Size**: 24 bytes
**Typical Records**: 0-500

---

### ocd_billofitems

Bill of materials.

| Column | Type | Offset | Description |
|--------|------|--------|-------------|
| article_nr | string_ref | 4 | Article number |
| var_cond | string_ref | 8 | Variant condition |
| component_nr | string_ref | 12 | Component article |
| quantity | float64 | 16 | Quantity |
| unit | string_ref | 24 | Unit |

**Record Size**: 28 bytes
**Typical Records**: 0-1000

---

### ocd_propertygroup

Property grouping (for UI organization).

| Column | Type | Offset | Description |
|--------|------|--------|-------------|
| group_id | string_ref | 4 | Group ID |
| position | uint16 | 8 | Display order |
| group_textnr | string_ref | 12 | Group label reference |

**Record Size**: 16 bytes
**Typical Records**: 0-50

---

### ocd_article2propgroup

Article-to-property-group mapping.

| Column | Type | Offset | Description |
|--------|------|--------|-------------|
| article_nr | string_ref | 4 | Article number |
| group_id | string_ref | 8 | Property group ID |
| prop_class | string_ref | 12 | Property class |

**Record Size**: 16 bytes
**Typical Records**: 0-200

---

### ocd_identification

Product identification data.

| Column | Type | Offset | Description |
|--------|------|--------|-------------|
| article_nr | string_ref | 4 | Article number |
| id_type | string_ref | 8 | ID type (e.g., "EAN", "GTIN") |
| id_value | string_ref | 12 | ID value |

**Record Size**: 16 bytes
**Typical Records**: 0-100

---

## ODB Tables

### odb3d

3D object definitions.

| Column | Type | Offset | Description |
|--------|------|--------|-------------|
| odb_name | string(7) | 0 | Object database name |
| obj_name | string(25) | 7 | Object file name |
| visible | string(1) | 32 | Visibility ("V", "H", or expression) |
| x_offs | string(15) | 49 | X offset (value or expression) |
| y_offs | string(7) | 64 | Y offset |
| z_offs | string(15) | 71 | Z offset |
| x_rot | string(7) | 86 | X rotation |
| y_rot | string(7) | 93 | Y rotation |
| z_rot | string(7) | 100 | Z rotation |
| ctor | string_ref | 108 | Constructor CLS code |
| mat | string_ref | 112 | Material reference |
| attrib | string_ref | 116 | Attributes |
| link | string(4) | 120 | Link ID |

**Record Size**: 124 bytes
**Typical Records**: 10-500

**Note**: Inline strings, not string_ref!

---

### odb2d

2D drawing objects.

| Column | Type | Offset | Description |
|--------|------|--------|-------------|
| odb_name | string(8) | 0 | Object name |
| level | uint16 | 8 | Drawing level |
| visible | string(1) | 10 | Visibility |
| x_offs | string(1) | 11 | X offset |
| y_offs | string(7) | 18 | Y offset |
| rot | string(2) | 20 | Rotation |
| x_scale | string(2) | 22 | X scale |
| y_scale | string(4) | 24 | Y scale |
| ctor | string_ref | 28 | Constructor |
| attrib | string_ref | 32 | Attributes |

**Record Size**: 36 bytes
**Typical Records**: 0-50

---

### layer

Layer definitions for 2D/3D organization.

| Column | Type | Offset | Description |
|--------|------|--------|-------------|
| layer_name | string_ref | 0 | Layer name |
| attributes | string_ref | 4 | Layer attributes/properties |

**Record Size**: 8 bytes
**Typical Records**: 0-50

---

### funcs

CLS function definitions.

| Column | Type | Offset | Description |
|--------|------|--------|-------------|
| name | string(20) | 0 | Function name |
| body | string_ref | 20 | CLS code body |

**Record Size**: 24 bytes
**Typical Records**: 0-100

---

### attpt

Attachment point definitions.

| Column | Type | Offset | Description |
|--------|------|--------|-------------|
| odb_name | string_ref | 4 | Object name |
| name | string(4) | 8 | Attachment point name |
| select | string_ref | 12 | Selection expression |
| text_idx | uint32 | 16 | Text index |
| x_pos | string(1) | 20 | X position |
| y_pos | string(1) | 21 | Y position |
| z_pos | string(1) | 22 | Z position |
| direction | string(1) | 23 | Direction |
| rotation | string(1) | 24 | Rotation |
| mode | string(1) | 25 | Mode |

**Record Size**: 28 bytes
**Typical Records**: 0-100

---

## OFML Tables

### proginfo

Program/series information.

| Column | Type | Offset | Description |
|--------|------|--------|-------------|
| type | string_ref | 4 | Info type |
| args | string_ref | 8 | Arguments |
| value | string_ref | 12 | Value |

**Record Size**: 16 bytes
**Typical Records**: 5-20

---

### plelement

Product element definitions.

Same schema as `proginfo`.

**Record Size**: 16 bytes
**Typical Records**: 1-10

---

### epdfproductdb

ePDF product database configuration.

Same schema as `proginfo`.

**Record Size**: 16 bytes
**Typical Records**: 1-10

---

## OAM Tables

### oam_article2ofml

Article-to-OFML object mapping.

| Column | Type | Offset | Description |
|--------|------|--------|-------------|
| article | string_ref | 0 | Article number |
| ofml_type | string_ref | 8 | OFML type ("OAP_ARTICLE", etc.) |
| odb_name | string_ref | 16 | ODB object name |
| params | string_ref | 20 | Additional parameters |

**Record Size**: 24 bytes
**Typical Records**: 1-100

---

### oam_property2mat

Property-to-material mapping.

| Column | Type | Offset | Description |
|--------|------|--------|-------------|
| article | string_ref | 0 | Article number |
| property | string_ref | 4 | Property ID |
| prop_value | string_ref | 8 | Property value |
| mat_layer | string_ref | 12 | Material layer name |
| material | string_ref | 16 | Material file reference |

**Record Size**: 20 bytes
**Typical Records**: 0-500

---

### oam_article2odbparams

Article-to-ODB parameter mapping.

| Column | Type | Offset | Description |
|--------|------|--------|-------------|
| article | string_ref | 4 | Article number |
| vc_type | string_ref | 8 | Variant condition type |
| varcode | string_ref | 12 | Variant code |
| params | string_ref | 16 | ODB parameters |

**Record Size**: 20 bytes
**Typical Records**: 0-100

---

## Manufacturer-Specific Tables

### vitra_availability

Vitra-specific availability data.

**Record Size**: Varies
**Typical Records**: 0-200

---

### propvalue2varcond

Property value to variant condition mapping (Vitra).

**Record Size**: Varies
**Typical Records**: 0-1000

---

### property_map

Custom property mapping (Vitra).

**Record Size**: Varies
**Typical Records**: 0-500

---

### propinfo

Additional property information (Vitra).

**Record Size**: Varies
**Typical Records**: 0-200

---

### s_mod_var_stuhl_tbl

Sedus chair modifier table.

| Column | Type | Offset | Description |
|--------|------|--------|-------------|
| line | string | 4 | Line identifier |
| name | string | 12 | Modifier name |
| value | string | 20 | Modifier value |

**Record Size**: 24 bytes
**Typical Records**: 1000-10000

---

### vb_elektro_ex_matn_tbl

Knoll electrical material table.

| Column | Type | Offset | Description |
|--------|------|--------|-------------|
| line | string | 4 | Line identifier |
| name | string | 12 | Material name |
| value | string | 24 | Material value |

**Record Size**: 40 bytes
**Typical Records**: 1000-10000

---

### farben_conline_tbl

Knoll ConLine color table.

| Column | Type | Offset | Description |
|--------|------|--------|-------------|
| line | string | 4 | Line identifier |
| name | string | 12 | Color name |
| value | string | 32 | Color value/code |

**Record Size**: 64 bytes
**Typical Records**: 100-1000

---

### art2aclass_map

Article-to-class mapping (Knoll).

| Column | Type | Offset | Description |
|--------|------|--------|-------------|
| article | string | 0 | Article number |
| aclass | string | 20 | Article class |
| params | string_ref | 52 | Parameters |

**Record Size**: 56 bytes
**Typical Records**: 100-1000

---

## Notes on Schema Variations

### Record Size Differences
- **Sedus**: Most text tables use 24-byte records (with `line_fmt`)
- **Knoll**: Most text tables use 20-byte records (no `line_fmt`)
- **Vitra**: Standard sizes + additional custom tables

### String Types
- **string_ref**: 4-byte reference to string table
- **string(N)**: N-byte inline string (not null-terminated, space-padded)

### Common Patterns

**Text Tables** (all follow similar pattern):
- textnr (reference key)
- language (DE, EN, IT, etc.)
- line_nr (for multi-line text)
- line_fmt (optional format code)
- text (actual content)

**Custom Tables** (manufacturer-specific):
- Usually follow `{prefix}_{name}_tbl` naming
- Often have: line, name, value columns
- Variable record sizes

---

**Version**: 1.0
**Last Updated**: December 31, 2024
**Coverage**: Standard OCD 4.3 + common extensions
