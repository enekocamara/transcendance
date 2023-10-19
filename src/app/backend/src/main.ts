import { NestFactory } from '@nestjs/core';
import { AppModule } from './app.module';


async function bootstrap() {
  const app = await NestFactory.create(AppModule);
  await app.listen(3000);
}

const fs = require('fs');
const util = require('util');

const logFile = fs.createWriteStream('error.log', { flags: 'a' }); // 'a' means appending (old data will be preserved)

// Redirect errors to the error.log file
console.error = (msg) => {
  logFile.write(util.format(msg) + '\n');
};
console.error('Starting the app...');

bootstrap();
