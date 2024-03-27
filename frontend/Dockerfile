FROM node:latest

WORKDIR /sites/public-page

COPY . .

RUN npm install
RUN npm run build

CMD ["npm", "run", "dev"]
