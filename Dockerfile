ARG nodeV=21.7.2

FROM node:${nodeV}-bullseye-slim as build
COPY package.json package-lock.json ./
RUN npm install
COPY tsconfig.json jest.config.ts .eslintrc.yaml .prettierrc.yaml ./
COPY src src
COPY test test
RUN npm run ci

FROM node:${nodeV}-bullseye-slim as release
COPY package.json package-lock.json ./
COPY public public
COPY --from=build dist/ dist/
RUN npm install --omit=dev
ENTRYPOINT ["npm", "start"]
