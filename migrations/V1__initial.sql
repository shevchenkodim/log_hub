/* user */
CREATE TABLE auth_user
(
    id serial primary key,
    username varchar(50) not null unique,
    password varchar(254) not null,
    last_login timestamptz not null
);


/* logger */
CREATE TABLE logger
(
    id serial primary key,
    level integer default 0,
    event_type integer not null,
    source varchar(50) not null,
    logger_name varchar(50) not null,
    user_id integer null default null,
    msg text not null,
    params json default '{}',
    created_at timestamptz not null
);

CREATE INDEX source_idx ON logger(source);
CREATE INDEX created_at_idx ON logger(created_at);
CREATE INDEX logger_name_idx ON logger(logger_name);


/* session */
CREATE TABLE session
(
    key varchar(40) primary key not null,
    payload text not null,
    expires timestamptz not null
);

CREATE INDEX key_idx ON session(key);
