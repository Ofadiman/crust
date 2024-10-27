create function column_update_guard() returns trigger as

$$
begin
    raise exception
        'trigger %: updating % column is prohibited',
        tg_name, tg_argv[0]
        using errcode = 'restrict_violation';
end;
$$ language plpgsql;

create function refresh_updated_at() returns trigger as
$$
begin
    new.updated_at = now();
    return new;
end
$$ language plpgsql;

create table users
(
    id         uuid        not null primary key default gen_random_uuid(),
    first_name text        not null,
    last_name  text        not null,
    email      text        not null,
    password   text        not null,
    created_at timestamptz not null             default now(),
    updated_at timestamptz not null             default now()
);

create trigger users_refresh_updated_at
    before update
    on users
    for each row
execute function refresh_updated_at();

create trigger users_immutable_created_at
    before update of created_at
    on users
    for each row
    when (old.created_at is distinct from new.created_at)
execute function column_update_guard('created_at');

