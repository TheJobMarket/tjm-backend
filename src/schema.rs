// @generated automatically by Diesel CLI.

diesel::table! {
    companies (id) {
        id -> Varchar,
        date_added -> Nullable<Date>,
        name -> Varchar,
        website -> Nullable<Varchar>,
        logo_cid -> Nullable<Varchar>,
        description -> Nullable<Text>,
    }
}

diesel::table! {
    jobs (id) {
        id -> Varchar,
        date_posted -> Nullable<Date>,
        title -> Varchar,
        description -> Nullable<Text>,
        company_id -> Varchar,
        active -> Bool,
        application_url -> Nullable<Varchar>,
        pay_min -> Nullable<Int4>,
        pay_max -> Nullable<Int4>,
        city -> Nullable<Varchar>,
        country -> Nullable<Varchar>,
        languages -> Nullable<Varchar>,
        workplace -> Nullable<Varchar>,
        job_type -> Nullable<Varchar>,
    }
}

diesel::joinable!(jobs -> companies (company_id));

diesel::allow_tables_to_appear_in_same_query!(
    companies,
    jobs,
);
