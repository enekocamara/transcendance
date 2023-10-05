# Use an official Node.js runtime as the base image
FROM node:19

# Set the working directory in the Docker container
RUN pwd
WORKDIR /usr/src/app

# Copy package.json and package-lock.json to the container
COPY ./tools/package.json /usr/src/app/package.json
# Install Node.js dependencies in the container
#RUN npm install && npm install -g nodemon

RUN npm install @nestjs/core @nestjs/common reflect-metadata rxjs
RUN npm install --save-dev nodemon

# If you're running in a development mode, you might also install the global CLI for NestJS (not typical for production images)
# RUN npm install -g @nestjs/cli

# Copy the rest of the application to the container
COPY . .

# Expose port 3000, which is the default port that NestJS apps listen on
EXPOSE 3000

# Start the application
    #for mounted files, no need to restart the container
CMD ["tail", "-f", "/dev/null"]
CMD ["npm", "run", "start:dev"]
#CMD ["npm", "run", "start"]

