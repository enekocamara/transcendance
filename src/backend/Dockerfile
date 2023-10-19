# Use an official Node.js runtime as the base image
FROM node:alpine3.18

# Set the working directory in the Docker containerx
WORKDIR /usr/src/app

# Copy package.json and package-lock.json to the container
COPY ./package*.json ./

RUN npm install pg winston

# Copy the rest of the application to the container
COPY . .

# Expose port 3000, which is the default port that NestJS apps listen on
EXPOSE 3000

CMD ["npm", "run", "start:dev"]