CREATE TABLE IF NOT EXISTS rooms (
    id           INTEGER PRIMARY KEY AUTOINCREMENT,
    name         TEXT    NOT NULL CHECK (name <> ''),
    max_capacity INTEGER NOT NULL CHECK (max_capacity > 0)
);

CREATE TABLE IF NOT EXISTS bookings (
    id        INTEGER PRIMARY KEY AUTOINCREMENT,
    room_id   INTEGER NOT NULL REFERENCES rooms(id) ON DELETE CASCADE,
    start_at  TEXT    NOT NULL,
    end_at    TEXT    NOT NULL,
    nb_people INTEGER NOT NULL CHECK (nb_people > 0),
    CHECK (start_at < end_at)
);

CREATE INDEX IF NOT EXISTS idx_bookings_room_start ON bookings (room_id, start_at);
CREATE INDEX IF NOT EXISTS idx_bookings_start ON bookings (start_at);
