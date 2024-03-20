const express = require('express');
const router = express.Router();
const client = require('../db');

router.post('/', async (req, res) => {
    const { id_order, id_food, amount } = req.body;
    console.log(id_order, id_food, amount);
    try {
        const result = await client.query('INSERT INTO ordered_food(id, id_order, id_food, amount) VALUES(DEFAULT,$1, $2, $3) RETURNING *', [id_order, id_food, amount]);
        res.json(result.rows[0]);
    } catch (err) {
        console.error(err);
        res.status(500).send("Erreur lors de l'ajout de la commande de nourriture");
    }
});

// Lire tous les ordered_food.js
router.get('/', async (req, res) => {
    try {
        const result = await client.query('SELECT * FROM ordered_food');
        res.json(result.rows);
    } catch (err) {
        console.error(err);
        res.status(500).send("Erreur lors de la récupération des commandes de nourriture");
    }
});

// Lire un ordered_food.js par son ID

router.get('/:id', async (req, res) => {
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

// Mettre à jour un ordered_food.js par son ID
router.put('/:id', async (req, res) => {
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

// Supprimer un ordered_food.js par son ID
router.delete('/:id', async (req, res) => {
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

module.exports = router;