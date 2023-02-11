ARG nodeV=18.14.0

from node:${nodeV}-bullseye-slim as build

copy package.json package-lock.json ./
run npm install
copy tsconfig.json jest.config.ts .eslintrc.yaml .prettierrc.yaml ./
copy src src
copy test test
run npm run ci

from node:${nodeV}-bullseye-slim as release
copy package.json package-lock.json ./
copy public public
copy --from=build dist/ dist/
run npm install --omit=dev
entrypoint ["npm", "start"]
