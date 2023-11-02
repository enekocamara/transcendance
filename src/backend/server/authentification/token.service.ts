import * as jwt from 'jsonwebtoken';
import { Injectable } from '@nestjs/common';


@Injectable()
export class TokenService{
     genToken(payload): string | null {
        return  jwt.sign(payload, process.env.TOKEN_SECRET_KEY, {expiresIn: process.env.TOKEN_EXPIRE_TIME});
    }
}