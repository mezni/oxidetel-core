INSERT INTO crm.customers (id, first_name, last_name, email, phone, created_at, updated_at) VALUES
('00000000-0000-0000-0000-000000000001', 'Acme', 'Corp', 'contact@acme.com', '555-1000', now(), now());
INSERT INTO crm.accounts (id, customer_id, account_number, status, created_at, updated_at) VALUES
('00000000-0000-0000-0000-000000000001', '00000000-0000-0000-0000-000000000001', 'ACC-001', 'active', now(), now());
INSERT INTO crm.customer_accounts (id, customer_id, account_id, relationship_type, created_at) VALUES
('00000000-0000-0000-0000-000000000001', '00000000-0000-0000-0000-000000000001', '00000000-0000-0000-0000-000000000001', 'primary', now());
