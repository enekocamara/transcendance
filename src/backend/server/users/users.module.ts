import { Module } from '@nestjs/common';
import { RegisterController } from './register.controller';
import { WinstonService } from 'server/winston.service';
import { DatabaseService } from 'server/database/database.providers';

@Module({
    imports:[],
    controllers: [RegisterController],
    providers: [DatabaseService],
})
export class UsersModule{
    constructor (private databaseService: DatabaseService,
                 private winston: WinstonService){}
}