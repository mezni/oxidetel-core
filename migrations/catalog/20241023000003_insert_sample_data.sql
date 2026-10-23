INSERT INTO catalog.products (id, name, description, sku, created_at, updated_at) VALUES
('00000000-0000-0000-0000-000000000001', 'Premium Plan', 'Gold tier subscription', 'GP-001', now(), now());
INSERT INTO catalog.offers (id, name, product_id, active, created_at, updated_at) VALUES
('00000000-0000-0000-0000-000000000001', 'Annual Gold', '00000000-0000-0000-0000-000000000001', true, now(), now());
INSERT INTO catalog.rate_plans (id, offer_id, name, monthly_price_cents, currency, created_at, updated_at) VALUES
('00000000-0000-0000-0000-000000000001', '00000000-0000-0000-0000-000000000001', 'Monthly Gold', 12999, 'USD', now(), now());
INSERT INTO catalog.add_ons (id, name, price_cents, currency, created_at, updated_at) VALUES
('00000000-0000-0000-0000-000000000001', 'Setup Fee', 999, 'USD', now(), now());
