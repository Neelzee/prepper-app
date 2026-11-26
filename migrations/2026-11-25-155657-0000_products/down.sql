-- This file should undo anything in `up.sql`
DROP TABLE IF EXISTS
  product_nutritions,
  product_events,
  product_instances,
  product_members,
  products,
  member_needs,
  household_members
  CASCADE;

ALTER TABLE users DROP COLUMN IF EXISTS household_id;
DROP TABLE IF EXISTS households CASCADE;
DROP TYPE IF EXISTS
  household_member_type,
  product_event_type,
  unit,
  product_instance_state,
  product_type;