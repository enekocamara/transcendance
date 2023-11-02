import { Controller, Post, Body, Res} from '@nestjs/common';
import { Response } from 'express';
import { DatabaseService} from '../database/database.providers';
import {WinstonService} from '../winston.service';
import { CreateUserDto } from './createUser.dto';
import { TokenService } from 'server/authentification/token.service';

@Controller('login')
export class LoginController {
  constructor (private databaseService: DatabaseService, private winston: WinstonService, private token: TokenService){}
  
  @Post()
  async login(@Body() createUserDto: CreateUserDto, @Res() res: Response): Promise<void> {
    const result = await this.databaseService.loginClient(createUserDto);
    if (result == true){
      const payload = {
        "sub": createUserDto.username,
        "iss": process.env.SERVER_IDENTIFIER,
        "aud": "my-client",
        "iat":  Math.floor(Date.now() / 1000)
      }
      const token = this.token.genToken(payload);
      if (!token)
        res.status(500).json( {message: 'Login failed: failed to create token'});
      else
        res.status(200).json({ message: 'Login successful', token: token});
    } else {
      res.status(500).json({ message: 'Login failed: login or password missmatch'});
    }
  }
}