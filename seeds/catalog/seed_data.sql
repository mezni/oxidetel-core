-- Seeded by synthetic-data generator (seed=42, reproducible)
INSERT INTO catalog.products (id, name, description, type, price, status, created_at, updated_at) VALUES
('00000000-0000-0000-0000-000000000001', 'Premium Plan', 'Premium service plan', 'subscription', 99.99, 'active', now(), now()),
('00000000-0000-0000-0000-000000000002', 'Standard Plan', 'Standard service plan', 'subscription', 49.99, 'active', now(), now()),
('00000000-0000-0000-0000-000000000003', 'Basic Plan', 'Basic service plan', 'subscription', 29.99, 'active', now(), now()),
('00000000-0000-0000-0000-000000000004', 'Enterprise Add-on', 'Enterprise features add-on', 'one-time', 1999.99, 'active', now(), now()),
('00000000-0000-0000-0000-000000000005', 'Support Package', '24/7 support package', 'subscription', 19.99, 'active', now(), now());

INSERT INTO catalog.offers (id, product_id, name, description, price, effective_from, effective_until, status) VALUES
('00000000-0000-0000-0000-0000-000000000001', '00000000-0000-0000-0000-000000000001', 'Summer Promotion', '20% discount on premium', 79.99, now(), now() + Interval('90 days'), 'active'),
('00000000-0000-0000-0000-0000-000000000002', '00000000-0000-0000-0000-000000000002', 'Winter Promotion', '15% discount on standard', 42.49, now(), now() + Interval('90 days'), 'active'),
('00000000-0000-0000-0000-0000-000000000003', '00000000-0000-0000-0000-000000000003', 'Spring Promotion', '10% discount on basic', 26.99, now(), now() + Interval('90 days'), 'active'),
('00000000-0000-0000-0000-0000-000000000004', '00000000-0000-0000-0000-000000000004', 'Early Bird', 'Discount for early signup', 1799.99, now(), now() + Interval('90 days'), 'active'),
('00000000-0000-0000-0000-000000000005', '00000000-0000-0000-0000-000000000005', 'Support Upgrade', 'Premium support upgrade', 17.99, now(), now() + Interval('90 days'), 'active');