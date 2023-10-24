import { Injectable,OnModuleInit, OnModuleDestroy } from '@nestjs/common';
import { Pool, Client } from 'pg';
import { WinstonService} from './winston.service'
import retry from 'async-retry';


@Injectable()
export class DatabaseService implements OnModuleInit, OnModuleDestroy{
    private pool: Pool;

    constructor (private winston :WinstonService){
        this.pool = new Pool({
            user: process.env.POSTGRES_USER,
            host: process.env.POSTGRES_HOST,
            database: process.env.POSTGRES_DB,
            password: process.env.POSTGRES_PASSWORD,
            port: parseInt(process.env.DATABASE_PORT, 10),
        });
    }

    async getClient(): Promise<Client>{
        const client = await this.pool.connect();
        return client;
    }

    async addPenguin() {
        this.winston.log('PINGUINGGGGGGG');
        try {
            const client = await this.getClient();
            try {
                const query = "INSERT INTO public.users (username, password, image_index) VALUES($1, $2, $3) RETURNING *;";
                const params = ['test_user', 'hashed_password', 1];
                const result = await client.query(query, params);

                this.winston.log(result.rows[0]); // This will log the inserted row
                if (result.rowCount === 1) {
                    this.winston.log('Successfully added a penguin.');
                } else {
                    this.winston.log('Failed to add a penguin.');
                }
            } catch (error) {
                this.winston.error('Database query error:', error.message);
            } finally {
                this.releaseClient(client);
            }
        } catch (error){
            this.winston.error('Database connection error: ', error.message);
        }
    }
    //INSERT INTO penguins(id, name) VALUES ('1', 'ecamara')

    releaseClient(client){
        client.release();
    }

    async onModuleInit(): Promise<void> {
        for (let i = 0; i < 5; i++) { // number of attempts
            try {
              await this.pool.connect();
              this.winston.log('Connected to the database');
              break; // connection is successful, break out of the loop
            } catch (error) {
              this.winston.error('Could not connect to the database, retrying in 5 seconds', error);
              await new Promise(res => setTimeout(res, 5000)); // wait for 5 seconds before trying again
            }
        }
    }

    async onModuleDestroy() {
        await this.pool.end();
    }
}