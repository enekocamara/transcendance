# Use an official Node.js runtime as the base image
FROM node:20

# Set the working directory in the Docker container
WORKDIR /usr/src/app

# Copy package.json and package-lock.json to the container

COPY package*.json ./

RUN npm install

RUN npm install @nestjs/core @nestjs/cli @nestjs/common ts-node typescript

# Copy the rest of the application to the container
COPY . .

# COPY ./tools/package.json /usr/src/app/package.json

# Expose port 3000, which is the default port that NestJS apps listen on
EXPOSE 3000


#CMD ["npm", "run", "start:dev"]
CMD ["npm", "run", "start:dev"]
