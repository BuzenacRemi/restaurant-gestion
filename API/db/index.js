const { Client } = require('pg');
const fs = require('fs');
const path = require('path');

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
        // Exécuter le script SQL d'initialisation
        initializeDatabase();
    }
});

async function initializeDatabase() {
    try {
        // Construire le chemin d'accès complet au fichier SQL
        const sqlScriptPath = path.join(__dirname, 'restaurant.sql');
        // Charger le script SQL d'initialisation depuis le fichier
        const sqlScript = fs.readFileSync(sqlScriptPath, 'utf8');
        // Exécuter le script SQL
        await client.query(sqlScript);
        console.log('Base de données initialisée avec succès');
    } catch (err) {
        console.error('Erreur lors de l\'initialisation de la base de données', err);
    }
}
