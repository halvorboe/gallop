
# Gallop

Analytics server based on 'tantivy.rs'

Made for my personal blog.

## Goal

Store a dictinary like structure for every event that comes in. Index it into tantivy. Log to file for backup. Can replay from file if necessary. 

## Libraries

https://github.com/graphql-rust/juniper
https://github.com/tantivy-search/tantivy

## API

Based on GraphQL.

## Structure of the project

Indexer should be seperate from the API node. 