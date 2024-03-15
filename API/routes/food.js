const express = require('express');
const router = express.Router();
const client = require('../db');

// Route pour récupérer tous les aliments
router.post('/', async (req, res) => {
    const { category, food_name, food_price } = req.body;
    try {
        const result = await client.query('INSERT INTO food(category, food_name, price) VALUES($1, $2, $3) RETURNING *', [category, food_name, food_price]);
        res.json(result.rows[0]);
    } catch (err) {
        console.error(err);
        res.status(500).send("Erreur lors de l'ajout de la nourriture");
    }
});

router.get('/:id', async (req, res) => {
    const { id } = req.params;
    try {
        const result = await client.query('SELECT * FROM food WHERE id = $1', [id]);
        if (result.rows.length > 0) {
            res.json(result.rows[0]);
        } else {
            res.status(404).send("Nourriture non trouvée");
        }
    } catch (err) {
        console.error(err);
        res.status(500).send("Erreur lors de la récupération de la nourriture");
    }
});

router.get('/', async (req, res) => {
    try {
        const result = await client.query('SELECT * FROM food');
        res.json(result.rows);
    } catch (err) {
        console.error(err);
        res.status(500).send("Erreur lors de la récupération de la nourriture");
    }
});

router.put('/:id', async (req, res) => {
    const { id } = req.params;
    const { category, food_name, food_price } = req.body;
    try {
        const result = await client.query('UPDATE food SET category = $1, food_name = $2, price = $3 WHERE id = $4 RETURNING *', [category, food_name, food_price, id]);
        if (result.rows.length > 0) {
            res.json(result.rows[0]);
        } else {
            res.status(404).send("Nourriture non trouvée");
        }
    } catch (err) {
        console.error(err);
        res.status(500).send("Erreur lors de la mise à jour de la nourriture");
    }
});

router.delete('/:id', async (req, res) => {
    const { id } = req.params;
    try {
        const result = await client.query('DELETE FROM food WHERE id = $1 RETURNING *', [id]);
        if (result.rows.length > 0) {
            res.json(result.rows[0]);
        } else {
            res.status(404).send("Nourriture non trouvée");
        }
    } catch (err) {
        console.error(err);
        res.status(500).send("Erreur lors de la suppression de la nourriture");
    }
});

module.exports = router;
