import { Module } from '@nestjs/common';
import { AppController } from './app.controller';
import { SpheniscidaeController } from './spheniscidae.controller';
import { AppService } from './app.service';
import { DatabaseService } from './database.providers';

@Module({
  imports: [],
  controllers: [AppController, SpheniscidaeController],
  providers: [AppService, DatabaseService],
})
export class AppModule {
  constructor (private dataService: DatabaseService){
  }
}
