set
    client_encoding = 'UTF8';

create type lifecycle as enum ('test', 'qa', 'stage', 'prod');

create type archival_format as enum ('zip', 'tgz');

create table namespace (
    id bigserial primary key not null,
    name text not null,
    description text default 'ML Namespace' :: text not null,
    created_at timestamptz not null default now(),
    last_modified timestamptz not null default now()
);

create index on namespace using btree(name);

create table bucket (
    id bigserial primary key not null,
    namespace bigint references namespace(id) on delete cascade on update cascade not null,
    name text not null,
    region text not null,
    role lifecycle not null,
    shard integer not null,
    created_at timestamptz not null default now(),
    last_modified timestamptz not null default now()
);

create index bucket_btree_name on bucket using btree(name);

create unique index bucket_name_idx on bucket (namespace, name, region);

create unique index bucket_role_shard on bucket (namespace, role, shard);

create table model (
    id bigserial primary key not null,
    namespace bigint references namespace(id) on delete cascade on update cascade not null,
    name text not null,
    created_at timestamptz not null default now(),
    last_modified timestamptz not null default now()
);

create index model_btree_name on model using btree(name);

create table model_version (
    id bigserial primary key not null,
    model_id bigint references model(id) on delete cascade on update cascade not null,
    version text not null
);

create unique index model_version_model_version_idx on model_version (model_id, version);

create table model_state (
    id bigserial primary key not null,
    version_id bigint references model_version(id) on delete cascade on update cascade not null,
    state lifecycle not null,
    last_modified timestamptz not null default now()
);

create unique index model_state_version_idx on model_state (version_id);

create table object_blob (
    id bigserial primary key not null,
    -- tracking only - we dont care about the actual bucket here for 'validation'
    bucket_id bigint references bucket(id) on delete cascade on update cascade not null,
    -- we also dont want the version id in here to that we can track & remove orphaned objects
    -- the derived key
    key text not null,
    -- the actual object version
    version_id varchar not null,
    -- size in bytes
    size bigint not null,
    -- validation & version checking metadata
    sha256 varchar(256) not null,
    archive archival_format not null,
    -- blob info
    created_at timestamptz not null default now() -- we do not allow modifications so there's nothing else to track
);

create unique index object_blob_version_idx on object_blob (version_id, key);

create table model_artifact (
    id bigserial primary key not null,
    -- we want the version id here to soft delete our artifacts
    version_id bigint references model_version(id) on delete cascade not null,
    blob bigint references object_blob(id) on delete cascade not null,
    -- the actual name of the artifact
    name text not null
);

create unique index model_artifact_version_idx on model_artifact (version_id, name);

create unique index model_artifact_blob_idx on model_artifact (blob);