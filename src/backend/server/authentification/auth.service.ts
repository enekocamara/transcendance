import * as argon2 from 'argon2';
import { Injectable } from '@nestjs/common';

@Injectable()
export class AuthService{
    async hashPassword(password: string): Promise<string> {
        return await argon2.hash(password);
    }

    async verifyPassword(storedHash: string, enteredPassword: string): Promise<boolean> {
        return await argon2.verify(storedHash, enteredPassword);
    }
}