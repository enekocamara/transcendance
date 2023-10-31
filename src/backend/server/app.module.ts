import { Module } from '@nestjs/common';
import { AppController } from './app.controller';
import { AppService } from './app.service';
import { DatabaseService } from './database/database.providers';
import { WinstonService } from './winston.service'
import { AuthService } from './authentification/hash.service';
import { AuthModule } from './authentification/auth.module';
import { RegisterController } from './users/register.controller';
import { LoginController } from './users/login.controller';


@Module({
  imports: [AuthModule],
  controllers: [AppController, RegisterController, LoginController],
  providers: [AppService, DatabaseService, WinstonService, AuthService],
})
export class AppModule {
  constructor (private dataService: DatabaseService,
      private winstonService: WinstonService){
      this.winstonService.log('AppModule initialized.');
  }
}
