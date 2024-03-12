const express = require('express');
const app = express();
const port = 6969;

app.use(express.json());

// Importer et utiliser les routes
app.use('/clients', require('./routes/clients'));
app.use('/food', require('./routes/food'));
app.use('/orders', require('./routes/orders'));
app.use('/restaurants', require('./routes/restaurants'));

app.listen(port, () => {
    console.log(`L'application écoute sur le port ${port}`);
});


// Gestion de la fermeture propre de l'application
process.on('SIGINT', () => {
    console.log("Fermeture du serveur et de la connexion à la base de données...");
    client.end(err => {
        if (err) {
            console.error('Erreur lors de la fermeture de la connexion à la base de données', err.stack);
        }
        process.exit();
    });
});

