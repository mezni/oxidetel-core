-- Seeded by synthetic-data generator (seed=42, reproducible)
INSERT INTO crm.customers (id, name, email, phone, company, created_at, updated_at) VALUES
('00000000-0000-0000-0000-000000000001', 'Acme Corp', 'contact@acme.com', '+1-555-0100', 'Acme Industries', now(), now()),
('00000000-0000-0000-0000-000000000002', 'Beta LLC', 'info@beta.com', '+1-555-0101', 'Beta Group', now(), now()),
('00000000-0000-0000-0000-000000000003', 'Gamma Ltd', 'sales@gamma.com', '+1-555-0102', 'Gamma Corp', now(), now()),
('00000000-0000-0000-0000-000000000004', 'Delta Inc', 'support@delta.com', '+1-555-0103', 'Delta Corp', now(), now()),
('00000000-0000-0000-0000-000000000005', 'Epsilon SAS', 'contact@eps.com', '+1-555-0104', 'Epsilon SAS', now(), now());

INSERT INTO crm.contracts (id, customer_id, name, value, status, created_at, updated_at) VALUES
('00000000-0000-0000-0000-000000000001', '00000000-0000-0000-0000-000000000001', 'Standard Contract', 15000.00, 'active', now(), now()),
('00000000-0000-0000-0000-000000000002', '00000000-0000-0000-0000-000000000002', 'Premium Contract', 25000.00, 'active', now(), now()),
('00000000-0000-0000-0000-000000000003', '00000000-0000-0000-0000-000000000003', 'Basic Contract', 8000.00, 'pending', now(), now()),
('00000000-0000-0000-0000-000000000004', '00000000-0000-0000-0000-000000000004', 'Enterprise Contract', 50000.00, 'active', now(), now()),
('00000000-0000-0000-0000-000000000005', '00000000-0000-0000-0000-000000000005', 'Trial Contract', 1000.00, 'cancelled', now(), now());