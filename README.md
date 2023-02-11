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

### Setup ###

Install node with [nvm](https://github.com/nvm-sh/nvm).

    nvm install     # one-time install
    nvm use         # each time you enter the project directory

Install npm packages.

    npm install


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
