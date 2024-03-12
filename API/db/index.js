const { Client } = require('pg');

const client = new Client({
    host: 'postgres_db',
    user: 'admin',
    port: 5432,
    password: 'Passw0rd',
    database: 'resto_database',
});

client.connect(err => {
    if (err) {
        console.error('Erreur de connexion à la base de données', err.stack);
    } else {
        console.log('Connecté à la base de données');
    }
});

module.exports = client;
