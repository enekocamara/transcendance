import * as jwt from 'jsonwebtoken';
import { Injectable } from '@nestjs/common';


@Injectable()
export class TokenService{
     genToken(payload): string | null {
        return  jwt.sign(payload, process.env.TOKEN_SECRET_KEY, {expiresIn: process.env.TOKEN_EXPIRE_TIME});
    }

    verifyToken(token: string): boolean{
        try {
            jwt.verify(token, process.env.TOKEN_SECRET_KEY)
            return true;
        } catch (error){
            return false;
        }
    }
}