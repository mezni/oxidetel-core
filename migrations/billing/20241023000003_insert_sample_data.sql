INSERT INTO billing.invoices (id, account_id, amount_cents, currency, status, due_date, created_at, updated_at) VALUES
('00000000-0000-0000-0000-000000000001', '00000000-0000-0000-0000-000000000001', 1250, 'USD', 'open', now() + interval '7 days', now(), now());
INSERT INTO billing.payments (id, invoice_id, amount_cents, currency, status, paid_at, created_at) VALUES
('00000000-0000-0000-0000-000000000001', '00000000-0000-0000-0000-000000000001', 1250, 'USD', 'completed', now(), now());
INSERT INTO billing.ledger_entries (id, account_id, entry_type, amount_cents, currency, description, created_at) VALUES
('00000000-0000-0000-0000-000000000001', '00000000-0000-0000-0000-000000000001', 'charge', 1250, 'USD', 'Invoice 125', now());
