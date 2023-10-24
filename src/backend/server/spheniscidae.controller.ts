import { Controller, Get, UseInterceptors } from '@nestjs/common';
import { DatabaseService} from './database.providers';
import {WinstonService} from './winston.service';

@Controller('spheniscidae')
export class SpheniscidaeController {
  constructor (private databaseService: DatabaseService, private winston: WinstonService){}
  
  @Get()
  getSpheniscidae(): string {
    this.databaseService.addPenguin();
    this.winston.log('New pinguin.');
    return 'Hello pingu!';
  }
}