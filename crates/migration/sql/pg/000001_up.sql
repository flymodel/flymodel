set
    client_encoding = 'UTF8';

create type lifecycle as enum ('test', 'qa', 'stage', 'prod');

create sequence namespace_id_seq START WITH 1 INCREMENT BY 1 NO MAXVALUE NO MINVALUE CACHE 1;

create table namespace (
    id integer primary key default nextval('namespace_id_seq' :: regclass) not null,
    name text not null,
    description text default 'ML Namespace'::text not null,
    created_at timestamp with time zone default now(),
    last_modified timestamp with time zone default now()
);

create index on namespace using btree(name);

create sequence bucket_id_seq START WITH 1 INCREMENT BY 1 NO MAXVALUE NO MINVALUE CACHE 1;

create table bucket (
    id integer primary key default nextval('bucket_id_seq' :: regclass) not null,
    namespace integer references namespace(id) on delete cascade on update cascade not null,
    name text not null,
    region text not null,
    role lifecycle not null,
    shard integer not null,
    created_at timestamp with time zone default now(),
    last_modified timestamp with time zone default now()
);


create index bucket_btree_name on bucket using btree(name);

create unique index bucket_name_idx on bucket (namespace, name, region);

create unique index bucket_role_shard on bucket (namespace, role, shard);

create sequence model_id_seq START WITH 1 INCREMENT BY 1 NO MAXVALUE NO MINVALUE CACHE 1;

create table model (
    id integer primary key default nextval('model_id_seq' :: regclass) not null,
    namespace integer references namespace(id) on delete cascade on update cascade not null,
    name text not null,
    created_at timestamp with time zone default now(),
    last_modified timestamp with time zone default now()
);

create index model_btree_name on model using btree(name);

create sequence model_version_seq START WITH 1 INCREMENT BY 1 NO MAXVALUE NO MINVALUE CACHE 1;

create table model_version (
    id integer primary key default nextval('model_version_seq' :: regclass) not null,
    model_id integer references model(id) on delete cascade on update cascade not null,
    version text not null
);

create unique index model_version_model_version_idx on model_version (model_id, version);

create sequence model_states_seq START WITH 1 INCREMENT BY 1 NO MAXVALUE NO MINVALUE CACHE 1;

create table model_states (
    id integer primary key default nextval('model_states_seq' :: regclass) not null,
    version_id integer references model_version(id) on delete cascade on update cascade not null,
    state lifecycle not null,
    last_modified timestamp with time zone default now()
);

create sequence model_artifacts_seq START WITH 1 INCREMENT BY 1 NO MAXVALUE NO MINVALUE CACHE 1;

create table model_artifacts (
    id integer primary key default nextval('model_artifacts_seq' :: regclass) not null,
    version_id integer references model_version(id) on delete cascade on update cascade not null,
    bucket_id integer references bucket(id) on delete cascade on update cascade not null,
    key text not null,
    created_at timestamp with time zone default now()
);
