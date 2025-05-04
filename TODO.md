# TODO - Faxmon Rust SDK (`faxmon-sdk-rust`)

Ce fichier liste les tâches pour créer le SDK Rust permettant aux agents/sondes d'interagir avec l'API Faxmon.

## Phase 1: Initialisation du Projet et Dépendances

* [ ] Créer un nouveau projet de librairie Rust (`cargo new --lib faxmon-sdk-rust`).
* [ ] Ajouter les dépendances nécessaires dans `Cargo.toml`:
    * [ ] `reqwest` (avec features `json`, `rustls-tls` ou `native-tls`).
    * [ ] `tokio` (avec features `macros`, `rt-multi-thread`).
    * [ ] `serde` (avec feature `derive`).
    * [ ] `serde_json`.
    * [ ] `url`.
    * [ ] `thiserror`.
    * [ ] `chrono` (optionnel, si besoin de manipuler des dates/heures dans le SDK).
* [ ] Mettre en place la structure de base (`src/lib.rs`, `client.rs`, `models.rs`, `error.rs`).

## Phase 2: Modèles de Données (`src/models.rs`)

* [ ] Définir l'enum `EventStatus` avec `#[serde(rename_all = "lowercase")]`.
* [ ] Définir la struct `EventStatusUpdatePayload` avec `#[derive(Serialize)]` et `Option<T>` pour les champs optionnels.
* [ ] Définir la struct `EventConsolidatedState` avec `#[derive(Deserialize)]`.
* [ ] Définir la struct `ErrorResponse` avec `#[derive(Deserialize)]`.
* [ ] Exporter publiquement les types nécessaires.

## Phase 3: Gestion des Erreurs (`src/error.rs`)

* [ ] Définir l'enum `Error` public avec `#[derive(Debug, thiserror::Error)]`.
* [ ] Ajouter des variantes pour `ReqwestError(#[from] reqwest::Error)`, `UrlParseError(#[from] url::ParseError)`, `JsonError(#[from] serde_json::Error)`.
* [ ] Ajouter une variante `ApiError { status: u16, message: String }` pour les erreurs retournées par l'API Faxmon.
* [ ] Implémenter `std::fmt::Display` pour l'enum `Error`.

## Phase 4: Client API (`src/client.rs`)

* [ ] Définir la struct publique `Client`.
* [ ] Implémenter la fonction publique `Client::new(base_url_str: &str, probe_token: String) -> Result<Self, crate::error::Error>`.
* [ ] Implémenter la méthode publique `async fn update_event_status(&self, event_id: &str, payload: crate::models::EventStatusUpdatePayload) -> Result<crate::models::EventConsolidatedState, crate::error::Error>`:
    * [ ] Construire l'URL avec `self.base_url.join(...)`.
    * [ ] Créer et envoyer la requête `POST` avec `self.http_client`.
        * [ ] Ajouter le header `Authorization: Bearer ...`.
        * [ ] Ajouter le header `Content-Type: application/json`.
        * [ ] Sérialiser le `payload` en JSON pour le corps.
    * [ ] Gérer la réponse :
        * [ ] Vérifier le statut HTTP.
        * [ ] Si succès, désérialiser le corps en `EventConsolidatedState`.
        * [ ] Si erreur, tenter de désérialiser en `ErrorResponse` et retourner `Error::ApiError`.
        * [ ] Encapsuler les autres erreurs (réseau, JSON) dans `crate::error::Error`.

## Phase 5: Exposition et Finalisation (`src/lib.rs`)

* [ ] Déclarer les modules `client`, `models`, `error`.
* [ ] Exporter les éléments publics nécessaires (`pub use ...`).
* [ ] Ajouter la documentation du crate (`//! ...`).

## Phase 6: Documentation & Exemples

* [ ] Ajouter la documentation Rustdoc (`///`) aux structs, enums, fonctions publiques.
* [ ] Rédiger le `README.md` du crate :
    * [ ] Description du SDK.
    * [ ] Instructions d'installation/utilisation (`Cargo.toml`).
    * [ ] Exemple d'instanciation du `Client`.
    * [ ] Exemple d'appel à `update_event_status`.
    * [ ] Mention de la gestion des erreurs (`Result`).
* [ ] Créer un exemple exécutable dans `examples/send_update.rs`.

## Phase 7: Tests (Optionnel mais Recommandé)

* [ ] Mettre en place des tests d'intégration (nécessite un serveur Faxmon de test ou un mock type `wiremock-rs` ou `mockito`).
    * [ ] Tester un appel réussi à `update_event_status`.
    * [ ] Tester différents cas d'erreur (401, 403, 404, 400, 500).
    * [ ] Tester la gestion des erreurs réseau.