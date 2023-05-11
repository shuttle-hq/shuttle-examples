SCHEDULE="*/2 * * * * *"
URL="https://example.com"

curl -v http://localhost:8000/crontab/set\
  -H "Content-Type: application/x-www-form-urlencoded"\
  -d "schedule=$SCHEDULE&url=$URL"
