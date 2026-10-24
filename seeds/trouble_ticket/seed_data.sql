-- Seeded by synthetic-data generator (seed=42, reproducible)
INSERT INTO trouble_ticket.tickets (id, title, description, status, priority, created_by, assigned_to, created_at, updated_at) VALUES
('00000000-0000-0000-0000-000000000001', 'Network outage', 'Complete network outage affecting floor 3', 'open', 'P1', 'admin', 'user1', now() - Interval('2 hours'), now() - Interval('2 hours')),
('00000000-0000-0000-0000-000000000002', 'Slow performance', 'API response times are degraded', 'in_progress', 'P2', 'user2', 'user2', now() - Interval('1 day'), now() - Interval('5 hours')),
('00000000-0000-0000-0000-000000000003', 'Password reset', 'User cannot login to system', 'resolved', 'P3', 'admin', 'user3', now() - Interval('3 days'), now() - Interval('1 day')),
('00000000-0000-0000-0000-000000000004', 'Printer issue', 'Printer not responding in office', 'open', 'P3', 'user4', NULL, now() - Interval('6 hours'), now() - Interval('6 hours')),
('00000000-0000-0000-0000-000000000005', 'Login lockout', 'Account locked after failed attempts', 'in_progress', 'P2', 'admin', 'user5', now() - Interval('12 hours'), now() - Interval('2 hours'));

INSERT INTO trouble_ticket.comments (id, ticket_id, user_id, comment, created_at) VALUES
('00000000-0000-0000-0000-000000000001', '00000000-0000-0000-0000-000000000001', 'admin', 'Investigating root cause', now() - Interval('1 hour')),
('00000000-0000-0000-0000-000000000002', '00000000-0000-0000-0000-000000000001', 'user1', 'Affects critical operations', now() - Interval('30 minutes')),
('00000000-0000-0000-0000-000000000003', '00000000-0000-0000-0000-000000000002', 'user2', 'Restarting services', now() - Interval('1 hour')),
('00000000-0000-0000-0000-000000000004', '00000000-0000-0000-0000-000000000003', 'user3', 'Reset password successfully', now() - Interval('2 days')),
('00000000-0000-0000-0000-000000000005', '00000000-0000-0000-0000-000000000004', 'user4', 'Replaced printer', now() - Interval('1 day'));