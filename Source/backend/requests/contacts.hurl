GET http://localhost:3000/api/v1/contacts

HTTP 200
[Asserts]
jsonpath "$" count == 0

POST http://localhost:3000/api/v1/contacts/persons
{
    "first_name": "Max",
    "last_name": "Mustermann",
    "email": [{
        "label": "Home",
        "email": "max.mustermann@example.org"
    }]
}

HTTP 201

POST http://localhost:3000/api/v1/contacts/organizations
{
    "name": "Example Org",
    "email": [{
        "label": "Work",
        "email": "info@example.org"
    }]
}

HTTP 201

GET http://localhost:3000/api/v1/contacts

HTTP 200
[Asserts]
jsonpath "$" count == 2
