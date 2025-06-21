CREATE EXTENSION "uuid-ossp";

CREATE TABLE subscriptions (
    id uuid PRIMARY KEY DEfAULT uuid_generate_v4(),
    email TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    subscribed_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

SELECT diesel_manage_updated_at('subscriptions');
