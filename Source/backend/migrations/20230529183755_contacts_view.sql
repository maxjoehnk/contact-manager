create view "Contacts" as
select id, 1 as type, first_name, last_name, null as name
from "Persons"
union
select id, 2, null, null, name
from "Organizations";
