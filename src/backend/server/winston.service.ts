// winston.service.ts
import { Injectable, LoggerService as NestLoggerService } from '@nestjs/common';
import * as winston from 'winston';

const myFormat = winston.format.printf(({ level, message, timestamp }) => {
  return `${timestamp} ${level}: ${message}`;
});

@Injectable()
export class WinstonService implements NestLoggerService {
  private readonly logger: winston.Logger;

  constructor() {
    this.logger = winston.createLogger({
      level: 'info',
      format: winston.format.combine(
        winston.format.timestamp({ format: 'YYYY-MM-DD HH:mm:ss' }),
        winston.format.errors({ stack: true }),
        winston.format.printf(({ level, message, timestamp, stack }) => {
          if (stack) {
            const stackLines = stack.split('\n');
            if (stackLines.length > 1) {
              const lineInfo = stackLines[1].trim();
              return `${timestamp} ${level}: ${message} (${lineInfo})`;
            }
          }
          return `${timestamp} ${level}: ${message}`;
        })
      ),
      transports: [
        new winston.transports.File({ filename: 'logs/error.log', level: 'error' }),
        new winston.transports.File({ filename: 'logs/combined.log' }),
      ],
    });
  }

  log(message: string) {
    this.logger.info(message);
  }

  //error(message: string, trace: string) {
  //  this.logger.error(message, { trace });
  //}

  error(message: string, error: Error) {
    this.logger.error(message, error);
  }

  warn(message: string) {
    this.logger.warn(message);
  }

  debug(message: string) {
    this.logger.debug(message);
  }

  verbose(message: string) {
    this.logger.verbose(message);
  }
}