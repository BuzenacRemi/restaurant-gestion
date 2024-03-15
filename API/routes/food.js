const express = require('express');
const router = express.Router();
const client = require('../db');

// Route pour récupérer tous les foods
router.post('/', async (req, res) => {
    const { food_name } = req.body;
    try {
        const result = await client.query('INSERT INTO food(food_name) VALUES($1) RETURNING *', [food_name]);
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
    const { food_name } = req.body;
    try {
        const result = await client.query('UPDATE food SET food_name = $1 WHERE id = $2 RETURNING *', [food_name, id]);
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