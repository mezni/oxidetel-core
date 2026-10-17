-- Billing Schema: Invoices Table
CREATE TABLE IF NOT EXISTS billing.invoices (
    id UUID PRIMARY KEY,
    customer_id UUID NOT NULL REFERENCES crm.customers(id),
    amount_cents BIGINT NOT NULL,
    currency VARCHAR(3) NOT NULL DEFAULT 'USD',
    status VARCHAR(50) NOT NULL DEFAULT 'ISSUED', -- 'ISSUED', 'PAID', 'OVERDUE', 'CANCELLED'
    due_date TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- Billing Schema: Invoice Line Items (Itemized Charges)
CREATE TABLE IF NOT EXISTS billing.invoice_items (
    id UUID PRIMARY KEY,
    invoice_id UUID NOT NULL REFERENCES billing.invoices(id) ON DELETE CASCADE,
    description TEXT NOT NULL,
    amount_cents BIGINT NOT NULL
);

-- Index for efficient customer invoice lookups
CREATE INDEX IF NOT EXISTS idx_invoices_customer 
ON billing.invoices(customer_id);
