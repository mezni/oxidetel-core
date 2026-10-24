-- Seeded by synthetic-data generator (seed=42, reproducible)
INSERT INTO billing.invoices (id, customer_id, subtotal, tax, total, status, issued_at, due_at, paid_at) VALUES
('00000000-0000-0000-0000-000000000001', '00000000-0000-0000-0000-000000000001', 1000.00, 180.00, 1180.00, 'paid', now() - Interval('30 days'), now() - Interval('20 days'), now() - Interval('25 days')),
('00000000-0000-0000-0000-000000000002', '00000000-0000-0000-0000-000000000002', 2000.00, 360.00, 2360.00, 'paid', now() - Interval('60 days'), now() - Interval('50 days'), now() - Interval('55 days')),
('00000000-0000-0000-0000-000000000003', '00000000-0000-0000-0000-000000000003', 500.00, 90.00, 590.00, 'pending', now() - Interval('10 days'), now(), NULL),
('00000000-0000-0000-0000-000000000004', '00000000-0000-0000-0000-000000000004', 3000.00, 540.00, 3540.00, 'overdue', now() - Interval('90 days'), now() - Interval('60 days'), NULL),
('00000000-0000-0000-0000-000000000005', '00000000-0000-0000-0000-000000000005', 150.00, 27.00, 177.00, 'paid', now() - Interval('5 days'), now() - Interval('2 days'), now() - Interval('4 days'));

INSERT INTO billing.payments (id, invoice_id, amount, payment_method, status, transaction_id, created_at) VALUES
('00000000-0000-0000-0000-000000000001', '00000000-0000-0000-0000-000000000001', 1180.00, 'credit_card', 'success', 'txn_001', now() - Interval('25 days')),
('00000000-0000-0000-0000-000000000002', '00000000-0000-0000-0000-000000000002', 2360.00, 'bank_transfer', 'success', 'txn_002', now() - Interval('55 days')),
('00000000-0000-0000-0000-000000000005', '00000000-0000-0000-0000-000000000005', 177.00, 'credit_card', 'success', 'txn_005', now() - Interval('4 days'));