-- Dunning Schema: Collections and Enforcement Cases
CREATE TABLE IF NOT EXISTS dunning.cases (
    id UUID PRIMARY KEY,
    invoice_id UUID NOT NULL REFERENCES billing.invoices(id),
    customer_id UUID NOT NULL REFERENCES crm.customers(id),
    stage VARCHAR(50) NOT NULL DEFAULT 'PAST_DUE', -- 'PAST_DUE', 'SOFT_SUSPENSION', 'HARD_SUSPENSION', 'RESOLVED'
    days_overdue INT NOT NULL DEFAULT 0,
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Index for background dunning processor sweeps
CREATE INDEX IF NOT EXISTS idx_dunning_stage 
ON dunning.cases(stage);
