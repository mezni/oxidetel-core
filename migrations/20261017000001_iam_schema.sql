-- 1. Roles Table (RBAC)
CREATE TABLE IF NOT EXISTS iam.roles (
    id UUID PRIMARY KEY,
    name VARCHAR(50) NOT NULL UNIQUE,          -- 'ADMIN', 'CSR', 'CUSTOMER'
    description TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- 2. Users Table (Unified login credentials)
CREATE TABLE IF NOT EXISTS iam.users (
    id UUID PRIMARY KEY,
    email VARCHAR(255) NOT NULL UNIQUE,
    password_hash VARCHAR(255) NOT NULL,       -- Argon2 hash
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    customer_id UUID,                          -- Linked to crm.customers later
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- 3. User Roles Mapping (Many-to-Many)
CREATE TABLE IF NOT EXISTS iam.user_roles (
    user_id UUID NOT NULL REFERENCES iam.users(id) ON DELETE CASCADE,
    role_id UUID NOT NULL REFERENCES iam.roles(id) ON DELETE CASCADE,
    PRIMARY KEY (user_id, role_id)
);

-- Seed default system roles
INSERT INTO iam.roles (id, name, description) VALUES 
('a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a11', 'ADMIN', 'System administrator with full access'),
('a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a22', 'CSR', 'Customer Service Representative'),
('a0eebc99-9c0b-4ef8-bb6d-6bb9bd380a33', 'CUSTOMER', 'End-subscriber portal user')
ON CONFLICT (id) DO NOTHING;
