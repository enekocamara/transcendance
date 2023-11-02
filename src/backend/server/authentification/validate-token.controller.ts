import { Controller, Post, Headers, Res} from '@nestjs/common';
import { Response } from 'express';
import { TokenService } from 'server/authentification/token.service';

@Controller('validate-token')
export class ValidateTokenController{
    constructor (private token: TokenService){}
    @Post()
    async validateToken(@Headers('authorization') authorization: string, @Res() res: Response):Promise<void> {
        const token = authorization.split('Bearer ')[1];
        if (this.token.verifyToken(token) == true)
            res.status(200);
        else    
            res.status(500);
    }
}