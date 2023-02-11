Plants & Pests
==============

Plant-based CCG.
Home-grown card game goodness.


Docker
------

    sh docker-build.sh
    sh docker-run.sh


Developing
----------

## Server-side Setup ##

Install node with [nvm](https://github.com/nvm-sh/nvm).

    nvm install     # one-time install
    nvm use         # each time you enter the project directory

Install npm packages.

    npm install


### Configuration ###

Define a secret for cookie signing.

    cp example.env .env
    pwgen -sy 48 >> .env

Edit .env file to set cookie_secret


### Build ###

    npx tsc


### Run ###

    npm start


### Run during dev ###

While developing, you can skip building by using tsnode.

    npm run start-dev


### Format, lint, build, test ###

    npm run all


### Test coverage ###

    # Run the tests with `npm run all` or
    npm test
    # This will generate the test coverage report
    # Then open the test coverage report
    firefox coverage/lcov-report/index.html


### Generate documentation ###

    npm run doc
    firefox docs/index.html &


### Update dependencies ###

    npm run ncu


## Client-side Setup ##

Install Rust and its dev tools via Rustup

    curl https://sh.rustup.rs -sSf | sh


### Build & Package the game ###
    
    cargo build


### Run the game ###

    cargo run
