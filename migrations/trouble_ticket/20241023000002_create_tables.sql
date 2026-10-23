CREATE TABLE IF NOT EXISTS trouble_ticket.tickets (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    account_id UUID,
    customer_id UUID,
    title TEXT NOT NULL,
    description TEXT,
    status TEXT NOT NULL DEFAULT 'open',
    priority TEXT NOT NULL DEFAULT 'medium',
    created_at TIMESTAMP WITH TIME ZONE DEFAULT now(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT now()
);

CREATE TABLE IF NOT EXISTS trouble_ticket.comments (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    ticket_id UUID REFERENCES trouble_ticket.tickets(id) ON DELETE CASCADE,
    author_id UUID,
    content TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT now()
);

CREATE TABLE IF NOT EXISTS trouble_ticket.transitions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    ticket_id UUID REFERENCES trouble_ticket.tickets(id) ON DELETE CASCADE,
    from_status TEXT,
    to_status TEXT NOT NULL,
    actor_id UUID,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT now()
);

CREATE TABLE IF NOT EXISTS trouble_ticket.audit_events (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    ticket_id UUID REFERENCES trouble_ticket.tickets(id) ON DELETE CASCADE,
    event_type TEXT NOT NULL,
    payload JSONB,
    actor_id UUID,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT now()
);
