FROM node:19-alpine
WORKDIR /project

RUN apk add npm

COPY ./project/ ./

RUN npm install --force --silent

EXPOSE 3000