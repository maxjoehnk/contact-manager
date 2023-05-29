GET http://localhost:3000/api/v1/contacts

HTTP 200
[Asserts]
jsonpath "$" count == 0

POST http://localhost:3000/api/v1/contacts/persons
{
    "firstName": "Max",
    "lastName": "Mustermann",
    "email": [{
        "label": "Home",
        "email": "max.mustermann@example.org"
    }]
}

HTTP 201

[Captures]
person_id: jsonpath "$"

GET http://localhost:3000/api/v1/contacts/persons

HTTP 200
[Asserts]
jsonpath "$" count == 1
jsonpath "$[0].id" == "{{person_id}}"

POST http://localhost:3000/api/v1/contacts/organizations
{
    "name": "Example Org",
    "email": [{
        "label": "Work",
        "email": "info@example.org"
    }]
}

HTTP 201

[Captures]
organization_id: jsonpath "$"

GET http://localhost:3000/api/v1/contacts/organizations

HTTP 200
[Asserts]
jsonpath "$" count == 1
jsonpath "$[0].id" == "{{organization_id}}"

GET http://localhost:3000/api/v1/contacts

HTTP 200
[Asserts]
jsonpath "$" count == 2

DELETE http://localhost:3000/api/v1/contacts/persons/{{person_id}}

HTTP 204

DELETE http://localhost:3000/api/v1/contacts/organizations/{{organization_id}}

HTTP 204

GET http://localhost:3000/api/v1/contacts

HTTP 200
[Asserts]
jsonpath "$" count == 0
