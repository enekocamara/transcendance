import { Injectable, OnModuleDestroy } from '@nestjs/common';
import { Pool } from 'pg';

@Injectable()
export class DatabaseService implements OnModuleDestroy{
    private pool = new Pool({
        user: process.env.POSTGRES_USER,
        host: process.env.DATABASE_HOST,
        database: process.env.POSTGRES_DB,
        password: process.env.POSTGRES_PASSWORD,
        port: parseInt(process.env.DATABASE_PORT, 10),
    });

    async getClient(){
        const client = await this.pool.connect();
        return client;
    }

    async addPenguin() {
        const client = await this.getClient();
        try {
            const query = 'INSERT INTO penguins(id, name) VALUES($1,$2)';
            const params = ['1', 'pinguino'];
            await client.query(query, params);
        } catch (error) {
            console.error('Database error:', error);
            this.releaseClient(client);
            // or handle the error in a way that's appropriate for your application
        } finally {
            this.releaseClient(client);
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