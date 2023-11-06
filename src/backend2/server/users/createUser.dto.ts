//import {IsString, IsNotEmpty} from '@nestjs/class-validator';

export class CreateUserDto {
    //@IsString()
    //@IsNotEmpty()
    readonly username: string;
  
    //@IsString()
    //@IsNotEmpty()
    readonly password: string;
}