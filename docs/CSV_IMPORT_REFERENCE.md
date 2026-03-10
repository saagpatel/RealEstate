# CSV Import Reference

## Expected header row

Use the downloadable template from the dashboard, or make sure your CSV includes
these headers:

```text
address,city,state,zip,beds,baths,sqft,price,property_type,year_built,lot_size,parking,key_features,neighborhood,neighborhood_highlights,school_district,nearby_amenities,agent_notes
```

## Required fields

These fields are required for each row:

- `address`
- `city`
- `state`
- `zip`
- `beds`
- `baths`
- `sqft`
- `price`
- `property_type`

## Allowed property type values

Use one of these exact values:

- `single_family`
- `condo`
- `townhouse`
- `multi_family`
- `land`
- `commercial`

## Numeric fields

These fields must contain valid numeric values:

- `beds`
- `baths`
- `sqft`
- `price`
- `year_built` when present

`price` is stored in dollars in the CSV and converted to cents by the app.

## Multi-value fields

These fields must use JSON array syntax when they contain more than one value:

- `key_features`
- `neighborhood_highlights`
- `nearby_amenities`

Examples:

```json
["Pool", "Chef's kitchen", "Large backyard"]
```

```json
["Transit access", "Walkable retail corridor"]
```

## Optional fields

These fields may be left blank:

- `year_built`
- `lot_size`
- `parking`
- `neighborhood`
- `school_district`
- `agent_notes`

## Import behavior

- each row is validated independently
- invalid rows are reported back as import errors
- valid rows continue importing even when some rows fail
- CSV import does not require photos up front
