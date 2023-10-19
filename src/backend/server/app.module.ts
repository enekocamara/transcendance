import { Module } from '@nestjs/common';
import { AppController } from './app.controller';
import { SpheniscidaeController } from './spheniscidae.controller';
import { AppService } from './app.service';
import { DatabaseService } from './database.providers';
import { WinstonService } from './winston.service'


@Module({
  imports: [],
  controllers: [AppController, SpheniscidaeController],
  providers: [AppService, DatabaseService, WinstonService],
})
export class AppModule {
  constructor (private dataService: DatabaseService,
      private winstonService: WinstonService){
      this.winstonService.log('AppModule initialized.');
  }
}
