FROM node:lts
WORKDIR /app
RUN npm i -g jest
COPY package.json package-lock.json ./
RUN npm i
COPY ./*.js ./
CMD jest