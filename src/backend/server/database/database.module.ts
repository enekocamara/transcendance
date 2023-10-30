import { Module, NestModule} from '@nestjs/common';
import { DatabaseService } from './database.providers';
import { AuthModule } from 'server/authentification/auth.module';

@Module({
    imports: [AuthModule, ],
    providers: [DatabaseService],
})

export class DatabaseModule{}
