INSERT INTO auth.users (id, username, email, password_hash, created_at, updated_at) VALUES
('00000000-0000-0000-0000-000000000001', 'admin', 'admin@telecom.local', '\$6\$salaryhash1', now(), now());
INSERT INTO auth.roles (id, name, description, created_at) VALUES
('00000000-0000-0000-0000-000000000001', 'admin', 'System administrator', now());
INSERT INTO auth.permissions (id, name, resource, action, created_at) VALUES
('00000000-0000-0000-0000-000000000001', 'manage_users', 'user', 'CRUD', now());
INSERT INTO auth.user_roles (user_id, role_id) VALUES
('00000000-0000-0000-0000-000000000001', '00000000-0000-0000-0000-000000000001');
INSERT INTO auth.role_permissions (role_id, permission_id) VALUES
('00000000-0000-0000-0000-000000000001', '00000000-0000-0000-0000-000000000001');
