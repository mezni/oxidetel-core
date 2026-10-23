INSERT INTO trouble_ticket.tickets (id, account_id, customer_id, title, description, status, priority, created_at, updated_at) VALUES
('00000000-0000-0000-0000-000000000001', '00000000-0000-0000-0000-000000000001', NULL, 'Line issue', 'No dial tone on line 123', 'open', 'high', now(), now());
INSERT INTO trouble_ticket.comments (id, ticket_id, author_id, content, created_at) VALUES
('00000000-0000-0000-0000-000000000001', '00000000-0000-0000-0000-000000000001', NULL, 'Customer reports no dial tone', now());
INSERT INTO trouble_ticket.transitions (id, ticket_id, from_status, to_status, actor_id, created_at) VALUES
('00000000-0000-0000-0000-000000000001', '00000000-0000-0000-0000-000000000001', NULL, 'open', NULL, now());
INSERT INTO trouble_ticket.audit_events (id, ticket_id, event_type, payload, actor_id, created_at) VALUES
('00000000-0000-0000-0000-000000000001', '00000000-0000-0000-0000-000000000001', 'status_change', '{\"from\":null,\"to\":\"open\"}', NULL, now());
