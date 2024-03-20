# restaurant-gestion

## Prérequis

- Docker
- Docker-compose

## Installation

1. Clonez le dépôt :
```bash
git clone https://github.com/BuzenacRemi/restaurant-gestion.git
```

2. Naviguez vers le répertoire du projet :
```bash
cd restaurant-gestion
```
3. Construire l'image docker :
```bash
docker-compose build --up
```

## Utilisation
Une fois que les conteneurs Docker sont en cours d'exécution, vous pouvez accéder à l'application via votre navigateur web à l'adresse http://localhost:8000. 

## FAQ
Q: Comment arrêter les conteneurs Docker ?  

R: Vous pouvez arrêter les conteneurs Docker en exécutant la commande suivante dans le répertoire du projet :
```bash
docker-compose down
```

Q: Comment puis-je voir les logs de l'application ?

R: Vous pouvez voir les logs de l'application en exécutant la commande suivante dans le répertoire du projet :
```bash
docker-compose logs
```

Q: Comment puis-je mettre à jour l'application ?

R: Vous pouvez mettre à jour l'application en exécutant les commandes suivantes dans le répertoire du projet :
```bash
git pull
docker-compose down
docker-compose build --up
```

Q: Que faire si j'obtiens une erreur lors de l'exécution de docker-compose up -d ?

R: Assurez-vous que Docker est en cours d'exécution sur votre machine. Si l'erreur persiste, essayez de construire les images Docker individuellement en utilisant docker build.

## License

Ce projet est sous licence MIT - voir le fichier [LICENSE](LICENSE) pour plus de détails.
``` 