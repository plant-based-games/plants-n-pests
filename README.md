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

Make sure you have the wasm compilation target & Install the WASM Bindgen

    rustup target add wasm32-unknown-unknown
    cargo install wasm-bindgen-cli

### Build & Package the game for WASM ###
    
    cargo build --release --target wasm32-unknown-unknown
    wasm-bindgen --out-name plantsnpests \
    --out-dir public \
    --target web target/wasm32-unknown-unknown/release/client.wasm


### Build & run the game for your local environment ###

    cargo build
    cargo run


### Run rust tests ###

    cargo test


### Format rust code ###

    cargo fmt

