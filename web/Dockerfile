FROM node:16-alpine


RUN mkdir -p /app
WORKDIR /app

# Copying source files
COPY . /app

# Installing dependencies
RUN npm install --legacy-peer-deps

# Building app
# RUN npm run build
EXPOSE 3000

# Running the app
CMD "npm" "run" "dev"
