-- Rating Schema: Call Detail Records (CDRs) and Data Usage Records
CREATE TABLE IF NOT EXISTS rating.cdrs (
    id UUID PRIMARY KEY,
    customer_id UUID NOT NULL REFERENCES crm.customers(id),
    usage_type VARCHAR(20) NOT NULL,            -- 'DATA', 'VOICE', 'SMS'
    bytes_used BIGINT NOT NULL,                 -- Data volume in bytes
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Index for high-throughput querying and billing aggregation
CREATE INDEX IF NOT EXISTS idx_cdrs_customer_created 
ON rating.cdrs(customer_id, created_at);
