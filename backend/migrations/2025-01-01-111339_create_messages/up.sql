CREATE TABLE IF NOT EXISTS messages (
    id uuid DEFAULT gen_random_uuid() PRIMARY KEY,
    user_id uuid NOT NULL,
    recipient_id uuid NOT NULL,
    message varchar NOT NULL,
    sent_at timestamp NOT NULL,
    CONSTRAINT fk_users FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    CONSTRAINT fk_recipients FOREIGN KEY (recipient_id) REFERENCES users(id) ON DELETE CASCADE
)