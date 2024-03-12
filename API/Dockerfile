FROM node:latest

WORKDIR /app

COPY package*.json ./

COPY . .

EXPOSE 3000

CMD ["npm", "start"]
COPY wait-for-it.sh /wait-for-it.sh