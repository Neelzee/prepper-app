// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "event"))]
    pub struct Event;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "household_member_type"))]
    pub struct HouseholdMemberType;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "product_event_type"))]
    pub struct ProductEventType;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "product_instance_state"))]
    pub struct ProductInstanceState;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "product_type"))]
    pub struct ProductType;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "unit"))]
    pub struct Unit;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::HouseholdMemberType;

    household_members (id) {
        id -> Int4,
        household_id -> Int4,
        user_id -> Nullable<Int4>,
        display_name -> Text,
        kind -> HouseholdMemberType,
    }
}

diesel::table! {
    households (id) {
        id -> Int4,
        household_name -> Text,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::ProductType;

    member_needs (id) {
        id -> Int4,
        member_id -> Int4,
        need -> ProductType,
        notes -> Nullable<Text>,
    }
}

diesel::table! {
    product (id) {
        id -> Int4,
        ean -> Int4,
        product_name -> Text,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Event;

    product_event (id) {
        id -> Int4,
        created_at -> Timestamptz,
        #[sql_name = "product_event"]
        product_event_ -> Event,
        product_instance_id -> Int4,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::ProductEventType;

    product_events (id) {
        id -> Int4,
        created_at -> Timestamptz,
        product_event -> ProductEventType,
        product_instance_id -> Int4,
        user_id -> Nullable<Int4>,
        details -> Nullable<Jsonb>,
    }
}

diesel::table! {
    product_instance (id) {
        id -> Int4,
        slug -> Uuid,
        product_id -> Int4,
        expires_at -> Timestamptz,
        added_at -> Timestamptz,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::ProductInstanceState;

    product_instances (id) {
        id -> Int4,
        slug -> Text,
        product_id -> Int4,
        household_id -> Int4,
        expires_at -> Nullable<Timestamptz>,
        state -> ProductInstanceState,
        ended_at -> Nullable<Timestamptz>,
        added_at -> Timestamptz,
    }
}

diesel::table! {
    product_members (product_id, member_id) {
        product_id -> Int4,
        member_id -> Int4,
    }
}

diesel::table! {
    product_nutritions (product_id) {
        product_id -> Int4,
        energy_kcal -> Numeric,
        fat_g -> Numeric,
        saturated_g -> Numeric,
        carbs_g -> Numeric,
        sugars_g -> Numeric,
        fiber_g -> Numeric,
        protein_g -> Numeric,
        salt_g -> Numeric,
        serving_size -> Numeric,
        allergens -> Nullable<Jsonb>,
        micronutrients -> Nullable<Jsonb>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::ProductType;
    use super::sql_types::Unit;

    products (id) {
        id -> Int4,
        ean -> Nullable<Text>,
        product_brand -> Nullable<Text>,
        household_id -> Int4,
        product_name -> Text,
        product_type -> ProductType,
        package_size -> Nullable<Numeric>,
        package_unit -> Nullable<Unit>,
        deleted_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    sessions (id) {
        id -> Int4,
        user_id -> Int4,
        expires_at -> Timestamptz,
        created_at -> Timestamptz,
        last_seen_at -> Timestamptz,
        user_agent -> Text,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Text,
        pswd -> Nullable<Text>,
        household_id -> Nullable<Int4>,
    }
}

diesel::joinable!(household_members -> households (household_id));
diesel::joinable!(household_members -> users (user_id));
diesel::joinable!(member_needs -> household_members (member_id));
diesel::joinable!(product_event -> product_instance (product_instance_id));
diesel::joinable!(product_events -> product_instances (product_instance_id));
diesel::joinable!(product_events -> users (user_id));
diesel::joinable!(product_instance -> product (product_id));
diesel::joinable!(product_instances -> households (household_id));
diesel::joinable!(product_members -> household_members (member_id));
diesel::joinable!(product_members -> products (product_id));
diesel::joinable!(product_nutritions -> products (product_id));
diesel::joinable!(products -> households (household_id));
diesel::joinable!(sessions -> users (user_id));
diesel::joinable!(users -> households (household_id));

diesel::allow_tables_to_appear_in_same_query!(
    household_members,
    households,
    member_needs,
    product,
    product_event,
    product_events,
    product_instance,
    product_instances,
    product_members,
    product_nutritions,
    products,
    sessions,
    users,
);
