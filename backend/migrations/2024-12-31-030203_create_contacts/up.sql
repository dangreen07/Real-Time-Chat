CREATE TABLE IF NOT EXISTS contacts (
    id uuid DEFAULT gen_random_uuid() PRIMARY KEY,
    user_id uuid NOT NULL,
    contact_id uuid NOT NULL,
    CONSTRAINT fk_users FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    CONSTRAINT fk_contacts FOREIGN KEY (contact_id) REFERENCES users(id) ON DELETE CASCADE
)