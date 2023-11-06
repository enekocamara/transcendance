import {Module} from '@nestjs/common'
import { HashService } from './hash.service'
import { TokenService } from './token.service'
import { ValidateTokenController } from './validate-token.controller'

@Module({
    providers: [HashService, TokenService, ValidateTokenController],
    exports: [HashService, TokenService, ValidateTokenController],
})
export class AuthModule{}