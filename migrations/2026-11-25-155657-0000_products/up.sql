-- Your SQL goes here
CREATE TYPE household_member_type AS ENUM
( 'adult'
, 'child'
, 'infant'
, 'pet'
);

CREATE TYPE product_instance_state AS ENUM
( 'in_store'
, 'consumed'
, 'discarded'
);

CREATE TYPE product_event_type AS ENUM
( 'added'
, 'consumed'
, 'discarded'
, 'edited'
);

CREATE TYPE product_type AS ENUM
( 'food'
, 'medicine'
, 'sanitary'
, 'water'
, 'fuel'
, 'tool'
);

CREATE TYPE unit as ENUM
( 'mL'
, 'dL'
, 'L'
, 'g'
, 'hg'
, 'kg'
, 'pcs'
);

CREATE TABLE households (
  id            SERIAL PRIMARY KEY,
  household_name TEXT NOT NULL
);

ALTER TABLE users
ADD COLUMN
  household_id INTEGER REFERENCES households(id) ON DELETE SET NULL;
CREATE INDEX ON users(household_id);

CREATE TABLE household_members (
  id            SERIAL PRIMARY KEY,
  household_id  INTEGER NOT NULL REFERENCES households(id) ON DELETE CASCADE,
  user_id INTEGER UNIQUE REFERENCES users(id) ON DELETE SET NULL,
  display_name TEXT NOT NULL,
  kind household_member_type NOT NULL DEFAULT 'adult'
);
CREATE INDEX ON household_members(household_id);

CREATE TABLE member_needs (
  id            SERIAL PRIMARY KEY,
  member_id  INTEGER NOT NULL REFERENCES household_members(id) ON DELETE CASCADE,
  need  product_type NOT NULL,
  notes TEXT,
  UNIQUE (member_id, need)
);

CREATE TABLE products (
  id            SERIAL PRIMARY KEY,
  ean           TEXT,
  product_brand TEXT,
  household_id  INTEGER NOT NULL REFERENCES households(id) ON DELETE CASCADE,
  product_name  TEXT NOT NULL,
  product_type product_type NOT NULL,
  package_size  NUMERIC(10, 2) CHECK (package_size > 0),
  package_unit unit,
  deleted_at TIMESTAMPTZ,
  CHECK ((package_size IS NULL) = (package_unit IS NULL)),
  UNIQUE (household_id, id),
  UNIQUE (household_id, ean)
);
CREATE INDEX ON products(household_id);

CREATE TABLE product_nutritions (
  product_id INTEGER PRIMARY KEY REFERENCES products(id) ON DELETE CASCADE,
  energy_kcal NUMERIC(7, 2) NOT NULL CHECK (energy_kcal >= 0),
  fat_g NUMERIC(7, 2) NOT NULL CHECK (fat_g >= 0),
  saturated_g NUMERIC(7, 2) NOT NULL CHECK (saturated_g >= 0),
  carbs_g NUMERIC(7, 2) NOT NULL CHECK (carbs_g >= 0),
  sugars_g NUMERIC(7, 2) NOT NULL CHECK (sugars_g >= 0),
  fiber_g NUMERIC(7, 2) NOT NULL CHECK (fiber_g >= 0),
  protein_g NUMERIC(7, 2) NOT NULL CHECK (protein_g >= 0),
  salt_g NUMERIC(7, 2) NOT NULL CHECK (salt_g >= 0),
  serving_size NUMERIC(10, 2) NOT NULL CHECK (serving_size > 0),
  allergens JSONB,
  micronutrients JSONB
);

CREATE TABLE product_members (
  product_id  INTEGER NOT NULL REFERENCES products(id) ON DELETE CASCADE,
  member_id   INTEGER NOT NULL REFERENCES household_members(id) ON DELETE CASCADE,
  PRIMARY KEY (product_id, member_id)
);
CREATE INDEX ON product_members(member_id);

CREATE TABLE product_instances (
  id            SERIAL PRIMARY KEY,
  slug          TEXT NOT NULL CHECK (slug ~ '^[0-9A-Za-z]{8}$'),
  product_id    INTEGER NOT NULL,
  household_id    INTEGER NOT NULL REFERENCES households(id),
  expires_at    TIMESTAMPTZ,
  state product_instance_state NOT NULL DEFAULT 'in_store',
  ended_at TIMESTAMPTZ,
  added_at      TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
  UNIQUE (household_id, slug),
  FOREIGN KEY (household_id, product_id) REFERENCES products(household_id, id),
  CHECK ((state = 'in_store') = (ended_at IS NULL))
);
CREATE INDEX ON product_instances(product_id);
CREATE INDEX ON product_instances(expires_at) WHERE STATE = 'in_store';

CREATE TABLE product_events (
  id                  SERIAL PRIMARY KEY,
  created_at          TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
  product_event       product_event_type NOT NULL,
  product_instance_id INTEGER NOT NULL REFERENCES product_instances(id) ON DELETE RESTRICT,
  user_id INTEGER REFERENCES users(id) ON DELETE RESTRICT,
  details JSONB
);
CREATE INDEX ON product_events(product_instance_id);