
FROM node:16-alpine

WORKDIR /app

COPY package.json ./
COPY package-lock.json ./

RUN npm install
RUN npm install react-scripts@5.0.1 -g

COPY . .

RUN npm run build

CMD ["npm", "start"]