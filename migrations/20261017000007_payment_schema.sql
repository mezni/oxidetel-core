-- Billing Schema: Payments Ledger
CREATE TABLE IF NOT EXISTS billing.payments (
    id UUID PRIMARY KEY,
    invoice_id UUID NOT NULL REFERENCES billing.invoices(id),
    customer_id UUID NOT NULL REFERENCES crm.customers(id),
    amount_cents BIGINT NOT NULL,
    gateway_reference VARCHAR(100) NOT NULL UNIQUE, -- e.g., 'ch_3Mxyz123...'
    status VARCHAR(50) NOT NULL DEFAULT 'SUCCESS',   -- 'SUCCESS', 'FAILED'
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- Index for transaction lookups
CREATE INDEX IF NOT EXISTS idx_payments_invoice 
ON billing.payments(invoice_id);
