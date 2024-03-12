const express = require('express');
const router = express.Router();
const client = require('../db');

// Créer un nouveau ordered_food
router.post('/ordered_food', async (req, res) => {
    const { id_food, amount } = req.body;
    try {
        const result = await client.query('INSERT INTO ordered_food(id_food, amount) VALUES($1, $2) RETURNING *', [id_food, amount]);
        res.json(result.rows[0]);
    } catch (err) {
        console.error(err);
        res.status(500).send("Erreur lors de l'ajout de la commande de nourriture");
    }
});

// Lire tous les ordered_food
router.get('/ordered_food', async (req, res) => {
    try {
        const result = await client.query('SELECT * FROM ordered_food');
        res.json(result.rows);
    } catch (err) {
        console.error(err);
        res.status(500).send("Erreur lors de la récupération des commandes de nourriture");
    }
});

// Lire un ordered_food par son ID

router.get('/ordered_food/:id', async (req, res) => {
    const { id } = req.params;
    try {
        const result = await client.query('SELECT * FROM ordered_food WHERE id = $1', [id]);
        if (result.rows.length > 0) {
            res.json(result.rows[0]);
        } else {
            res.status(404).send("Commande de nourriture non trouvée");
        }
    } catch (err) {
        console.error(err);
        res.status(500).send("Erreur lors de la récupération de la commande de nourriture");
    }
});

// Mettre à jour un ordered_food par son ID
router.put('/ordered_food/:id', async (req, res) => {
    const { id } = req.params;
    const { id_food, amount } = req.body;
    try {
        const result = await client.query('UPDATE ordered_food SET id_food = $1, amount = $2 WHERE id = $3 RETURNING *', [id_food, amount, id]);
        if (result.rows.length > 0) {
            res.json(result.rows[0]);
        } else {
            res.status(404).send("Commande de nourriture non trouvée");
        }
    } catch (err) {
        console.error(err);
        res.status(500).send("Erreur lors de la mise à jour de la commande de nourriture");
    }
});

// Supprimer un ordered_food par son ID
router.delete('/ordered_food/:id', async (req, res) => {
    const { id } = req.params;
    try {
        const result = await client.query('DELETE FROM ordered_food WHERE id = $1 RETURNING *', [id]);
        if (result.rows.length > 0) {
            res.json(result.rows[0]);
        } else {
            res.status(404).send("Commande de nourriture non trouvée");
        }
    } catch (err) {
        console.error(err);
        res.status(500).send("Erreur lors de la suppression de la commande de nourriture");
    }
});

// Créer une nouvelle commande
router.post('/orders', async (req, res) => {
    const { id_client, id_restaurant, id_ordered_food } = req.body;
    try {
        const result = await client.query('INSERT INTO "order"(id_client, id_restaurant, id_ordered_food) VALUES($1, $2, $3) RETURNING *', [id_client, id_restaurant, id_ordered_food]);
        res.json(result.rows[0]);
    } catch (err) {
        console.error(err);
        res.status(500).send("Erreur lors de l'ajout de la commande");
    }
});

// Lire toutes les commandes
router.get('/orders', async (req, res) => {
    try {
        const result = await client.query('SELECT * FROM "order"');
        res.json(result.rows);
    } catch (err) {
        console.error(err);
        res.status(500).send("Erreur lors de la récupération des commandes");
    }
});

// Lire une commande par son ID
router.get('/orders/:id', async (req, res) => {
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
router.put('/orders/:id', async (req, res) => {
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
router.delete('/orders/:id', async (req, res) => {
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