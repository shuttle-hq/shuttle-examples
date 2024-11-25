up:
    shuttle run

fe:
    npm --prefix ./frontend run dev

build:
    npm --prefix ./frontend run build

deploy:
    build
    shuttle deploy

deploy-ad:
    build
    shuttle deploy --ad

test:
    hurl hurl/register.hurl --verbose
