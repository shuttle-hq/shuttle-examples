POST http://localhost:8000/api/auth/register
{
    "username":"josh",
    "password":"1234"
}

HTTP 201

POST http://localhost:8000/api/auth/login
{
    "username":"josh",
    "password":"1234"
}

HTTP 200

[Captures]
session-id: cookie "token"

POST http://localhost:8000/api/chat/create
Cookie: token={{session-id}}

HTTP 200

[Captures]
conversation-id: jsonpath "$['conversation_id']"

POST http://localhost:8000/api/chat/conversations/{{conversation-id}}
Cookie: token={{session-id}}
{
    "prompt":"Tell me about shuttle.dev",
    "model":"gpt-4o-mini"
}

HTTP 200

[Captures]
body: body
