# Quickstart: Synthetic Data (Phase 11)

## Load seed data

```bash
# From the repo root, run the schema creation migrations then insert data:
psql "postgres://postgres:postgres@localhost:5432/telecom" -f migrations/auth/20241023000003_insert_sample_data.sql
psql "postgres://postgres:postgres@localhost:5432/telecom" -f migrations/crm/20241023000003_insert_sample_data.sql
psql "postgres://postgres:postgres@localhost:5432/telecom" -f migrations/billing/20241023000003_insert_sample_data.sql
psql "postgres://postgres:postgres@localhost:5432/telecom" -f migrations/inventory/20241023000003_insert_sample_data.sql
psql "postgres://postgres:postgres@localhost:5432/telecom" -f migrations/trouble_ticket/20241023000003_insert_sample_data.sql
psql "postgres://postgres:postgres@localhost:5432/telecom" -f migrations/catalog/20241023000003_insert_sample_data.sql
```

## Verify counts

```bash
psql "postgres://postgres:postgres@localhost:5432/telecom" -c "SELECT COUNT(*) FROM crm.customers;"
psql "postgres://postgres:postgres@localhost:5432/telecom" -c "SELECT COUNT(*) FROM auth.users;"
```
