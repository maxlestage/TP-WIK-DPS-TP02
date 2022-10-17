# TP-WIK-DPS-TP01

## CONSIGNES :

- Mettre en place son environnement de développement et les bases du projet avec le moins de dépendances possibles.
- Développer une API qui retourne au format JSON les headers de la requête quand il y une requête HTTP GET sur /ping.
- Le serveur doit écouter sur un port configurable via la variable d'environnement : PING_LISTEN_PORT.

- Réponse vide avec code 404 si quoi que ça soit d'autre que GET /ping ou code 500 si erreur inconnue.
- Réalisation d'un README avec documentation sur le lancement du projet.

## Choix des Technos :

- Rust : `Comme langage de programmation.`
- Actix : `Web framework pour Rust.`
- Serde : `Outil de sérialization JSON.`

## Lancer le projet :

Commande à lancer dans le temrinal à la racine du projet :

```rs
cargo run // Cette commande permet de lancer le projet Rust sans configurer le port (par défaut 8080).
```

```rs
export PING_LISTEN_PORT=9000 && cargo run // Le port peut-être choisi au lancement de l'app, dans le cas présent on le définit sur 9000.
```

### Tester l'application :

- Depuis le Terminal avec la commande curl :

```txt
curl http://127.0.0.1:9000/ping
```

- \*Depuis un Navigateur Web via le lien suivant en local :

```txt
http://127.0.0.1:9000/ping
```

\*Retourne à l'utilisateur l'affichage du header sous la forme d'un JSON dans le body

Si l'utilisateur essaie de visiter tout autres routes que `/ping`, le serveur renverra une erreur une 404, celle-ci est visible dans la console du navigateur dans l'onglet Network ou Réseau.

LESTAGE Maxime - TP DevOps n°1 - Ynov 2022.
