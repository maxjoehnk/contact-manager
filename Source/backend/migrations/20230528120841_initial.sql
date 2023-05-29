create table "Events"
(
    event_id  uuid,
    domain_id uuid,
    version   int,
    event     jsonb
);

create unique index event_domain_version on "Events" (domain_id, version);

create table "Persons"
(
    id         uuid default gen_random_uuid() not null
        constraint "Persons_pk"
            primary key,
    first_name varchar,
    last_name  varchar
);


create table "Organizations"
(
    id   uuid default gen_random_uuid() not null
        primary key,
    name varchar                        not null
);

create table "DistributionLists"
(
    id   uuid    not null
        primary key,
    name varchar not null
);

create table "Person_DistributionList"
(
    person_id            uuid not null
        constraint person_distributionlist_persons_id_fk
            references "Persons" (id)
            on delete cascade,
    distribution_list_id uuid not null
        constraint person_distributionlist_distributionlists_id_fk
            references "DistributionLists" (id)
            on delete cascade
);

create unique index person_distributionlist_uindex
    on "Person_DistributionList" (person_id, distribution_list_id);

create table "Organization_DistributionList"
(
    organization_id            uuid not null
        constraint organization_distributionlist_organizations_id_fk
            references "Organizations" (id)
            on delete cascade,
    distribution_list_id uuid not null
        constraint organization_distributionlist_distributionlists_id_fk
            references "DistributionLists" (id)
            on delete cascade
);

create unique index organization_distributionlist_uindex
    on "Organization_DistributionList" (organization_id, distribution_list_id);

