import {Module} from '@nestjs/common'
import { HashService } from './hash.service'
import { TokenService } from './token.service'

@Module({
    providers: [HashService, TokenService],
    exports: [HashService, TokenService],
})
export class AuthModule{}