SCHEDULE="*/2 * * * * *"
URL="example.com"

curl -X POST http://localhost:8000/set-schedule\
  -H "Content-Type: application/x-www-form-urlencoded"\
  -d "schedule=$SCHEDULE&url=$URL"
