import { Controller, Post, Body, Res} from '@nestjs/common';
import { Response } from 'express';
import { DatabaseService} from '../database/database.providers';
import {WinstonService} from '../winston.service';
import { CreateUserDto } from './createUser.dto';

@Controller('login')
export class LoginController {
  constructor (private databaseService: DatabaseService, private winston: WinstonService){}
  
  @Post()
  async login(@Body() createUserDto: CreateUserDto, @Res() res: Response): Promise<void> {
    const val = await this.databaseService.loginClient(createUserDto);
    if (val == true){
      res.status(200).json({ message: 'Login successful' });
    } else {
      res.status(500).json({ message: 'Login failed: login or password missmatch'})
    }
  }
}