import { Controller, Post, Body, UseInterceptors,  Res} from '@nestjs/common';
import { Response } from 'express';
import { DatabaseService} from '../database/database.providers';
import {WinstonService} from '../winston.service';
import { CreateUserDto } from './createUser.dto';
import { RegistrationStatus } from '../database/registration-status.enum';

@Controller('register')
export class RegisterController {
  constructor (private databaseService: DatabaseService, private winston: WinstonService){}
  
  @Post()
  async register(@Body() createUserDto: CreateUserDto, @Res() res: Response): Promise<void> {
    this.winston.log("User: " + createUserDto.username + " wants to register.");
    const val = await this.databaseService.registerClient(createUserDto);
    if (val == RegistrationStatus.Success)
    {
      res.status(200).json({ message: 'Registration successful' });
    }else if (val == RegistrationStatus.ErrorDatabase){
      res.status(500).json({ message: 'Registration failed: database error' })
    }
    else {
      res.status(500).json({ message: 'Registration failed: already registered'})
    }
  }
}