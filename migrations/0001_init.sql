create table if not exists products (
    id integer primary key not null,
    title text not null,
    created_at integer not null,
    price integer not null default 0,
);
