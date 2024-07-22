-- Insert "jin" user.
INSERT INTO users (id, email, password_hash)
VALUES (gen_random_uuid(), 'jin', '$argon2id$v=19$m=19456,t=2,p=1$VE0e3g7DalWHgDwou3nuRA$uC6TER156UQpk0lNQ5+jHM0l5poVjPA1he/Tyn9J4Zw');

-- insert admin role
INSERT INTO roles (name) VALUES ('admin');
INSERT INTO roles (name) VALUES ('basic');

-- Set the default user as the admin user in the app
DO $$
DECLARE
    first_user_id UUID;
    admin_role_id INTEGER;
BEGIN
    -- Retrieve the ID of the first user.
    SELECT id INTO first_user_id FROM users ORDER BY created_at LIMIT 1;

    -- Retrieve the ID of the 'admin' role.
    SELECT id INTO admin_role_id FROM roles WHERE name = 'admin';

    -- Assign 'admin' role to first.
    INSERT INTO user_roles (user_id, role_id)
    VALUES (first_user_id, admin_role_id);
END $$;
