import { Controller, Post, UseInterceptors } from '@nestjs/common';
import { DatabaseService} from '../database.providers';
import {WinstonService} from '../winston.service';

@Controller('signup')
export class SignUpController {
  constructor (private databaseService: DatabaseService, private winston: WinstonService){}
  
  @Post()
  postSignUp(): string {
    return 'Signed up!';
  }
}