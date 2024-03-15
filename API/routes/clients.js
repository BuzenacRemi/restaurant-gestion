const express = require('express');
const router = express.Router();
const client = require('../db');

// Créer un nouveau client
router.post('/', async (req, res) => {
    const { name, address } = req.body;
    try {
        const result = await client.query('INSERT INTO client(name, address) VALUES($1, $2) RETURNING *', [name, address]);
        res.json(result.rows[0]);
    } catch (err) {
        console.error(err);
        res.status(500).send("Erreur lors de l'ajout du client");
    }
});

// Lire tous les clients
router.get('/', async (req, res) => {
    try {
        const result = await client.query('SELECT * FROM client');
        res.json(result.rows);
    } catch (err) {
        console.error(err);
        res.status(500).send("Erreur lors de la récupération des clients");
    }
});

// Lire un client par son ID
router.get('/:id', async (req, res) => {
    const { id } = req.params;
    try {
        const result = await client.query('SELECT * FROM client WHERE id = $1', [id]);
        if (result.rows.length > 0) {
            res.json(result.rows[0]);
        } else {
            res.status(404).send("Client non trouvé");
        }
    } catch (err) {
        console.error(err);
        res.status(500).send("Erreur lors de la récupération du client");
    }
});

// Mettre à jour un client par son ID
router.put('/:id', async (req, res) => {
    const { id } = req.params;
    const { name, address } = req.body;
    try {
        const result = await client.query('UPDATE client SET name = $1, address = $2 WHERE id = $3 RETURNING *', [name, address, id]);
        if (result.rows.length > 0) {
            res.json(result.rows[0]);
        } else {
            res.status(404).send("Client non trouvé");
        }
    } catch (err) {
        console.error(err);
        res.status(500).send("Erreur lors de la mise à jour du client");
    }
});

// Supprimer un client par son ID
router.delete('/:id', async (req, res) => {
    const { id } = req.params;
    try {
        const result = await client.query('DELETE FROM client WHERE id = $1 RETURNING *', [id]);
        if (result.rows.length > 0) {
            res.json(result.rows[0]);
        } else {
            res.status(404).send("Client non trouvé");
        }
    } catch (err) {
        console.error(err);
        res.status(500).send("Erreur lors de la suppression du client");
    }
});

module.exports = router;
