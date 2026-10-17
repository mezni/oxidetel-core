-- 1. Ordering Schema: Customer Purchase Orders
CREATE TABLE IF NOT EXISTS ordering.orders (
    id UUID PRIMARY KEY,
    customer_id UUID NOT NULL REFERENCES crm.customers(id),
    status VARCHAR(50) NOT NULL DEFAULT 'PENDING', -- 'PENDING', 'PROCESSING', 'COMPLETED', 'FAILED'
    total_amount_cents BIGINT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- 2. Ordering Schema: Order Line Items
CREATE TABLE IF NOT EXISTS ordering.order_items (
    id UUID PRIMARY KEY,
    order_id UUID NOT NULL REFERENCES ordering.orders(id) ON DELETE CASCADE,
    product_type VARCHAR(50) NOT NULL,            -- 'RATE_PLAN', 'DEVICE', 'SIM'
    product_id UUID NOT NULL,                     -- References rate_plan_id, device_id, or sim_id
    price_cents BIGINT NOT NULL
);

-- 3. OSS Provisioning Audit Log
CREATE TABLE IF NOT EXISTS ordering.provisioning_logs (
    id UUID PRIMARY KEY,
    order_id UUID NOT NULL REFERENCES ordering.orders(id),
    sim_id UUID NOT NULL,
    status VARCHAR(50) NOT NULL,                  -- 'ACTIVATED', 'BARRED'
    payload TEXT,
    executed_at TIMESTAMPTZ DEFAULT NOW()
);
