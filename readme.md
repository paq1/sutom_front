# SUTOM FRONTEND (POC Dioxus web)

## dépendance du front

<p>
Il faut lancer le projet sutom api en local
</p>
<a href="https://github.com/paq1/sutom-api/tree/ajout-cors-all">Sutom api</a>
<p>
A la racine du projet, ajoutez un fichier .env et mettez y l'uri de votre mongodb local,
par exemple: 
</p>

```text
MONGO_URI=mongodb://localhost:27017
```
<p>
Enfin lancer l'api sutom api
</p>

```shell
cargo run
```

## premier run du front

<ul>
    <li>installation des tools</li>
</ul>

```shell
rustup update
```
```shell
cargo install dioxus-cli
```
```shell
rustup target add wasm32-unknown-unknown
```

<p>NB: soucis avec dioxus sur les variables d'env pour le moment, en 
attendant le fix de leur côté j'ai mis en place un scrip qui va recup
les variables d'env et les mettre dans un fichier de config
</p>

<ul>
    <li>
    execution du script permettant l'ecriture dans le fichier de config
    </li>
</ul>

```shell
cargo run --example setup_env
```
<p>
NB: la partie --bin ne fonctionne pas avec dioxus d'où le --example
</p>
<ul>
    <li>
    lancement de l'app
    </li>
</ul>

```shell
dioxus serve
```