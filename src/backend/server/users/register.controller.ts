import { Controller, Post, Body, UseInterceptors } from '@nestjs/common';
import { DatabaseService} from '../database/database.providers';
import {WinstonService} from '../winston.service';
import { CreateUserDto } from './createUser.dto';

@Controller('register')
export class RegisterController {
  constructor (private databaseService: DatabaseService, private winston: WinstonService){}
  
  @Post()
  register(@Body() createUserDto : CreateUserDto ):string {
    this.winston.log("Register petition recieved.")
    const val = this.databaseService.registerClient(createUserDto);
    if (!val)
      return 'Error 500'
    else
      return '200'
  }
}