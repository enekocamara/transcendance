import { Controller, Get } from '@nestjs/common';
import { DatabaseService} from './database.providers'

@Controller('spheniscidae')
export class SpheniscidaeController {
  constructor (private databaseService: DatabaseService){}
  
  @Get()
  getSpheniscidae(): string {
    this.databaseService.addPenguin();
    return 'Hello pingu!';
  }
}