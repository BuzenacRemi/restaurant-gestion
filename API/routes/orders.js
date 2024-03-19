const express = require('express');
const router = express.Router();
const client = require('../db');


// Créer une nouvelle commande
router.post('/', async (req, res) => {
    const { user_id, id_ordered_food } = req.body;
    try {
        const result = await client.query('INSERT INTO "order"(id, id_client, id_ordered_food) VALUES(DEFAULT, $1, DEFAULT) RETURNING *', [user_id]);
        res.json(result.rows[0]);
    } catch (err) {
        console.error(err);
        res.status(500).send("Erreur lors de l'ajout de la commande");
    }
});

// Lire toutes les commandes
router.get('/', async (req, res) => {
    try {
        const result = await client.query('SELECT * FROM "order"');
        res.json(result.rows);
    } catch (err) {
        console.error(err);
        res.status(500).send("Erreur lors de la récupération des commandes");
    }
});

// Lire une commande par son ID
router.get('/:id', async (req, res) => {
    const { id } = req.params;
    try {
        const result = await client.query('SELECT * FROM "order" WHERE id = $1', [id]);
        if (result.rows.length > 0) {
            res.json(result.rows[0]);
        } else {
            res.status(404).send("Commande non trouvée");
        }
    } catch (err) {
        console.error(err);
        res.status(500).send("Erreur lors de la récupération de la commande");
    }
});

// Mettre à jour une commande par son ID
router.put('/:id', async (req, res) => {
    const { id } = req.params;
    const { id_client, id_restaurant, id_ordered_food } = req.body;
    try {
        const result = await client.query('UPDATE "order" SET id_client = $1, id_restaurant = $2, id_ordered_food = $3 WHERE id = $4 RETURNING *', [id_client, id_restaurant, id_ordered_food, id]);
        if (result.rows.length > 0) {
            res.json(result.rows[0]);
        } else {
            res.status(404).send("Commande non trouvée");
        }
    } catch (err) {
        console.error(err);
        res.status(500).send("Erreur lors de la mise à jour de la commande");
    }
});

// Supprimer une commande par son ID
router.delete('/:id', async (req, res) => {
    const { id } = req.params;
    try {
        const result = await client.query('DELETE FROM "order" WHERE id = $1 RETURNING *', [id]);
        if (result.rows.length > 0) {
            res.json(result.rows[0]);
        } else {
            res.status(404).send("Commande non trouvée");
        }
    } catch (err) {
        console.error(err);
        res.status(500).send("Erreur lors de la suppression de la commande");
    }
});

module.exports = router;