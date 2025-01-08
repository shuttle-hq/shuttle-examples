# URL Shortener

A URL shortener that you can use from your terminal - built with Shuttle, rocket and postgres with opendal.

## How to use it

POST a URL to like so:

```bash
curl -d 'https://shuttle.dev/' http://localhost:8000/
```

You will get the shortened URL back (something like this `http://localhost:8000/0fvAo2`).
Visiting it will redirect you to the original URL.
