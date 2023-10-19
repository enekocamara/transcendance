import { Injectable, OnModuleDestroy } from '@nestjs/common';
import { Pool } from 'pg';
import { WinstonService} from './winston.service'

@Injectable()
export class DatabaseService implements OnModuleDestroy{
    private pool = new Pool({
        user: process.env.POSTGRES_USER,
        host: process.env.DATABASE_HOST,
        database: process.env.POSTGRES_DB,
        password: process.env.POSTGRES_PASSWORD,
        port: parseInt(process.env.DATABASE_PORT, 10),
    });

    constructor (private winston :WinstonService){}

    async getClient(){
        const client = await this.pool.connect();
        return client;
    }

    async addPenguin() {
        try {
            const client = await this.getClient();
            try {
                const query = 'INSERT INTO penguins(id, name) VALUES($1,$2)';
                const params = ['1', 'pinguino'];
                await client.query(query, params);
            } catch (error) {
                this.winston.error('Database error:', error);
                this.releaseClient(client);
                // or handle the error in a way that's appropriate for your application
            } finally {
                this.releaseClient(client);
            }
        } catch (error){
            this.winston.error('Database error: ', error);
        }
    }
    //INSERT INTO penguins(id, name) VALUES ('1', 'ecamara')

    releaseClient(client){
        client.release();
    }

    async onModuleDestroy() {
        await this.pool.end();
    }
}