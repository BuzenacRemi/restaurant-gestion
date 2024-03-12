const express = require('express');
const router = express.Router();
const client = require('../db');

// Route pour récupérer tous les restaurants

// Créer un nouveau restaurant
router.post('/restaurants', async (req, res) => {
    const { name } = req.body;
    try {
        const result = await client.query('INSERT INTO restaurant(name) VALUES($1) RETURNING *', [name]);
        res.json(result.rows[0]);
    } catch (err) {
        console.error(err);
        res.status(500).send("Erreur lors de l'ajout du restaurant");
    }
});

// Lire tous les restaurants
router.get('/restaurants', async (req, res) => {
    try {
        const result = await client.query('SELECT * FROM restaurant');
        res.json(result.rows);
    } catch (err) {
        console.error(err);
        res.status(500).send("Erreur lors de la récupération des restaurants");
    }
});

// Lire un restaurant par son ID

router.get('/restaurants/:id', async (req, res) => {
    const { id } = req.params;
    try {
        const result = await client.query('SELECT * FROM restaurant WHERE id = $1', [id]);
        if (result.rows.length > 0) {
            res.json(result.rows[0]);
        } else {
            res.status(404).send("Restaurant non trouvé");
        }
    } catch (err) {
        console.error(err);
        res.status(500).send("Erreur lors de la récupération du restaurant");
    }

});
// Mettre à jour un restaurant par son ID
router.put('/restaurants/:id', async (req, res) => {
    const { id } = req.params;
    const { name } = req.body;
    try {
        const result = await client.query('UPDATE restaurant SET name = $1 WHERE id = $2 RETURNING *', [name, id]);
        if (result.rows.length > 0) {
            res.json(result.rows[0]);
        } else {
            res.status(404).send("Restaurant non trouvé");
        }
    } catch (err) {
        console.error(err);
        res.status(500).send("Erreur lors de la mise à jour du restaurant");
    }
});

// Supprimer un restaurant par son ID
router.delete('/restaurants/:id', async (req, res) => {
    const { id } = req.params;
    try {
        const result = await client.query('DELETE FROM restaurant WHERE id = $1 RETURNING *', [id]);
        if (result.rows.length > 0) {
            res.json(result.rows[0]);
        } else {
            res.status(404).send("Restaurant non trouvé");
        }
    } catch (err) {
        console.error(err);
        res.status(500).send("Erreur lors de la suppression du restaurant");
    }
});

module.exports = router;