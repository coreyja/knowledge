CREATE TABLE PageSnapshot (
    page_snapshot_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    page_id UUID REFERENCES Pages (page_id) NOT NULL,
    raw_html TEXT NOT NULL,
    fetched_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
    -- title TEXT NOT NULL,
    -- cleaned_html TEXT NOT NULL,
    -- cleaned_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
    -- markdown_id UUID
);
