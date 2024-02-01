set
    client_encoding = 'UTF8';

create type lifecycle as enum ('test', 'qa', 'stage', 'prod');

-- storing models or test data
create type archive_compression as enum (
    'uncompressed',
    'zip',
    'gzip',
    'tar',
    'tzg',
    'snappy',
    'lz4',
    'zstd'
);

-- when we want to store test data for reproducibility
create type archive_format as enum (
    -- data
    'json',
    'jsonl',
    'arrow',
    'parquet',
    'msgpack',
    'xml',
    'csv',
    'xls',
    -- misc train / artifacts
    'jpeg',
    'png',
    'wav',
    'mov',
    'mp4',
    'pdf',
    'html',
    'txt',
    'md'
);

create table namespace (
    id bigserial primary key not null,
    name text not null,
    description text default 'ML Namespace' :: text not null,
    created_at timestamptz not null default now(),
    last_modified timestamptz not null default now()
);

comment on table namespace is 'a namespace';

create index on namespace using btree(name);

create table bucket (
    id bigserial primary key not null,
    namespace bigint references namespace(id) on delete cascade on update cascade not null,
    name text not null,
    region text not null,
    role lifecycle not null,
    -- shard integer not null,
    created_at timestamptz not null default now(),
    last_modified timestamptz not null default now()
);

comment on table bucket is 'a bucket';

comment on column bucket.role is 'the lifecycle of the bucket';

create index bucket_btree_name on bucket using btree(name);

create unique index bucket_name_idx on bucket (namespace, name, region);

-- create unique index bucket_role_shard on bucket (namespace, role, shard);
create table model (
    id bigserial primary key not null,
    namespace bigint references namespace(id) on delete cascade on update cascade not null,
    name text not null,
    created_at timestamptz not null default now(),
    last_modified timestamptz not null default now()
);

comment on table model is 'a model';

create index model_btree_name on model using btree(name);

create table model_version (
    id bigserial primary key not null,
    model_id bigint references model(id) on delete cascade on update cascade not null,
    version text not null
);

comment on table model_version is 'the version of a model';

create unique index model_version_model_version_idx on model_version (model_id, version);

create table model_state (
    id bigserial primary key not null,
    version_id bigint references model_version(id) on delete cascade on update cascade not null,
    state lifecycle not null,
    last_modified timestamptz not null default now()
);

comment on table model_state is 'the lifecycle of a model version';

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
    -- null implies raw archives
    encode archive_compression,
    format archive_format,
    -- blob info
    created_at timestamptz not null default now() -- we do not allow modifications so there's nothing else to track
);

comment on table object_blob is 'a single object blob (meta)';

create type upload_ticket_status as enum (
    'pending',
    'uploading',
    'erred',
    'expired',
    'cancelled',
    'completed'
);

create table upload_tickets (
    id bigserial primary key not null,
    status upload_ticket_status not null,
    created_at timestamptz not null default now(),
    last_modified timestamptz not null default now()
);

create unique index object_blob_version_idx on object_blob (version_id, key);

create table model_artifact (
    id bigserial primary key not null,
    -- we want the version id here to soft delete our artifacts
    version_id bigint references model_version(id) on delete cascade not null,
    blob bigint references object_blob(id) on delete cascade not null,
    extra jsonb,
    name text not null
);

comment on table model_artifact is 'a single artifact associated with a single model version';

comment on column model_artifact.extra is 'we may want to store some metadata about the artifact
this is optional and may be null
this allows e.g.
    { "model_encoding": "burn" }
    { "model_encoding": "safetensors", "model_type": "xgboost" }
';

comment on column model_artifact.name is 'the actual name of the artifact';

comment on column model_artifact.version_id is 'the version id of the artifact';

create unique index model_artifact_version_idx on model_artifact (version_id, name);

create unique index model_artifact_blob_idx on model_artifact (blob);

-- we want to be able to associate multiple experiments with a single model version
create table experiment (
    id bigserial primary key not null,
    version_id bigint references model_version(id) on delete cascade on update cascade not null,
    name text not null,
    created_at timestamptz not null default now()
);

comment on table experiment is 'a single experiment associated with a single model version';

-- we want experiments to be uniquely identificable per model version
create unique index experiment_name_idx on experiment (version_id, name);

create table experiment_artifact (
    id bigserial primary key not null,
    experiment_id bigint references experiment(id) on delete cascade on update cascade not null,
    -- the data blob is the actual artifact
    -- we want the version id here to soft delete our artifacts
    version_id bigint references model_version(id) on delete cascade not null,
    -- we want the data blob to be unique per experiment
    blob bigint references object_blob(id) on delete cascade not null,
    -- the actual name of the artifact
    name text not null
);

comment on table experiment_artifact is 'a single artifact associated with a single experiment';

create unique index experiment_artifact_name_idx on experiment_artifact (experiment_id, name);

create unique index experiment_artifact_blob_idx on experiment_artifact (blob);

create table namespace_tag (
    id bigint primary key not null,
    namespace_id bigint references namespace(id) on delete cascade not null,
    tag text not null,
    color text not null,
    created_at timestamptz not null default now()
);

create table model_tag (
    id bigint primary key not null,
    model_id bigint references model(id) on delete cascade not null,
    tag bigint references namespace_tag(id) on delete cascade not null,
    created_at timestamptz not null default now()
);

create unique index model_tags_model_tag_idx on model_tag (model_id, tag);

create table model_version_tag (
    id bigint primary key not null,
    version_id bigint references model_version(id) on delete cascade not null,
    tag bigint references namespace_tag(id) on delete cascade not null,
    created_at timestamptz not null default now()
);

create unique index model_version_tag_version_tag_idx on model_version_tag (version_id, tag);

create table experiment_tag (
    id bigint primary key not null,
    experiment_id bigint references experiment(id) on delete cascade not null,
    tag bigint references namespace_tag(id) on delete cascade not null,
    created_at timestamptz not null default now()
);

create unique index experiment_tags_experiment_tag_idx on experiment_tag (experiment_id, tag);