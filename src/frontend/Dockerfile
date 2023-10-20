# Use an official Node.js runtime as the base image
FROM node:alpine3.18

# Set the working directory in the Docker containerx
WORKDIR /usr/src/app

# Copy package.json and package-lock.json to the container
COPY ./package*.json ./

RUN npm install

# Copy the rest of the application to the container
COPY . .

#CMD ["npm", "run", "dev", "--", "--host", "0.0.0.0"]
CMD /bin/sh start.sh