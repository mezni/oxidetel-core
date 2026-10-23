INSERT INTO inventory.devices (id, serial_number, device_type, status, created_at, updated_at) VALUES
('00000000-0000-0000-0000-000000000001', 'DEV-001', 'router', 'active', now(), now());
INSERT INTO inventory.sims (id, iccid, status, created_at, updated_at) VALUES
('00000000-0000-0000-0000-000000000001', '89441234567891234123', 'active', now(), now());
INSERT INTO inventory.assignments (id, device_id, sim_id, account_id, assigned_at, status) VALUES
('00000000-0000-0000-0000-000000000001', '00000000-0000-0000-0000-000000000001', NULL, '00000000-0000-0000-0000-000000000001', now(), 'active');
