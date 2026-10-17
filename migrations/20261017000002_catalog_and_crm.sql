-- 1. CRM Schema: Customer Profiles
CREATE TABLE IF NOT EXISTS crm.customers (
    id UUID PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL UNIQUE,
    status VARCHAR(50) NOT NULL DEFAULT 'ACTIVE', -- 'ACTIVE', 'SUSPENDED', 'TERMINATED'
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- Link iam.users to crm.customers now that the table exists
ALTER TABLE iam.users 
    ADD CONSTRAINT fk_users_customer 
    FOREIGN KEY (customer_id) REFERENCES crm.customers(id) ON DELETE SET NULL;

-- 2. Catalog Schema: Rate Plans (Tariffs & Allowances)
CREATE TABLE IF NOT EXISTS catalog.rate_plans (
    id UUID PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    code VARCHAR(50) NOT NULL UNIQUE,          -- e.g., 'PLAN_5G_10GB'
    monthly_fee_cents BIGINT NOT NULL,          -- e.g., 4999 ($49.99)
    included_data_bytes BIGINT NOT NULL,        -- e.g., 10GB in bytes
    overage_rate_per_mb_cents BIGINT NOT NULL,  -- e.g., 5 cents per MB
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- 3. Catalog Schema: SIM Cards (Inventory)
CREATE TABLE IF NOT EXISTS catalog.sim_cards (
    id UUID PRIMARY KEY,
    customer_id UUID REFERENCES crm.customers(id),
    iccid VARCHAR(30) NOT NULL UNIQUE,          -- SIM serial number
    imsi VARCHAR(30) NOT NULL UNIQUE,           -- International Mobile Subscriber Identity
    msisdn VARCHAR(20) NOT NULL UNIQUE,         -- Phone number
    status VARCHAR(50) NOT NULL DEFAULT 'IN_STOCK' -- 'IN_STOCK', 'ACTIVATED', 'SUSPENDED'
);

-- 4. Catalog Schema: Devices & Inventory
CREATE TABLE IF NOT EXISTS catalog.devices (
    id UUID PRIMARY KEY,
    brand VARCHAR(100) NOT NULL,                -- e.g., 'Apple', 'Samsung'
    model VARCHAR(100) NOT NULL,                -- e.g., 'iPhone 16 Pro'
    sku VARCHAR(50) NOT NULL UNIQUE,
    retail_price_cents BIGINT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS catalog.device_inventory (
    id UUID PRIMARY KEY,
    device_id UUID NOT NULL REFERENCES catalog.devices(id),
    imei VARCHAR(18) NOT NULL UNIQUE,
    serial_number VARCHAR(100) NOT NULL UNIQUE,
    customer_id UUID REFERENCES crm.customers(id),
    status VARCHAR(50) NOT NULL DEFAULT 'IN_STOCK' -- 'IN_STOCK', 'ASSIGNED'
);

-- 5. Catalog Schema: Active Subscriptions
CREATE TABLE IF NOT EXISTS catalog.subscriptions (
    id UUID PRIMARY KEY,
    customer_id UUID NOT NULL REFERENCES crm.customers(id),
    rate_plan_id UUID NOT NULL REFERENCES catalog.rate_plans(id),
    sim_id UUID NOT NULL REFERENCES catalog.sim_cards(id),
    device_inventory_id UUID REFERENCES catalog.device_inventory(id),
    status VARCHAR(50) NOT NULL DEFAULT 'ACTIVE',
    billing_cycle_day INT NOT NULL CHECK (billing_cycle_day BETWEEN 1 AND 28),
    started_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    ended_at TIMESTAMPTZ
);
