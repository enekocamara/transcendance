import { Module } from '@nestjs/common';
import { RegisterController } from './register.controller';
import { WinstonService } from 'server/winston.service';
import { DatabaseService } from 'server/database/database.providers';
import { LoginController } from './login.controller';

@Module({
    imports:[],
    controllers: [RegisterController, LoginController],
    providers: [DatabaseService],
})
export class UsersModule{
    constructor (private databaseService: DatabaseService,
                 private winston: WinstonService){}
}