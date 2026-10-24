-- Seeded by synthetic-data generator (seed=42, reproducible)
INSERT INTO inventory.resources (id, name, type, status, location, capacity, created_at, updated_at) VALUES
('00000000-0000-0000-0000-000000000001', 'Server Rack A', 'rack', 'active', 'Data Center East', 42, now(), now()),
('00000000-0000-0000-0000-000000000002', 'Server Rack B', 'rack', 'active', 'Data Center East', 42, now(), now()),
('00000000-0000-0000-0000-000000000003', 'Server Rack C', 'rack', 'maintenance', 'Data Center West', 42, now(), now()),
('00000000-0000-0000-0000-000000000004', 'Server Rack D', 'rack', 'retired', 'Data Center West', 42, now(), now()),
('00000000-0000-0000-0000-000000000005', 'Network Switch 1', 'network', 'active', 'Server Room', 24, now(), now());

INSERT INTO inventory.assignments (id, resource_id, assigned_to, assigned_at, returned_at) VALUES
('00000000-0000-0000-0000-000000000001', '00000000-0000-0000-0000-000000000001', 'user1', now() - Interval('30 days'), NULL),
('00000000-0000-0000-0000-0000-000000000002', '00000000-0000-0000-0000-000000000002', 'user2', now() - Interval('15 days'), now() - Interval('5 days')),
('00000000-0000-0000-0000-0000-000000000003', '00000000-0000-0000-0000-000000000003', 'user3', now() - Interval('5 days'), NULL),
('00000000-0000-0000-0000-000000000004', '00000000-0000-0000-0000-000000000004', 'user4', now() - Interval('2 days'), NULL),
('00000000-0000-0000-0000-000000000005', '00000000-0000-0000-0000-000000000005', 'user5', now() - Interval('10 days'), now() - Interval('3 days'));