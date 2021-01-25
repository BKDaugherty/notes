# Notes
I've become increasingly frustrated by my ability to record my notes in a searchable fashion.
This is a basic CRUD API for storing my notes in a way that I will like, and can programatically iterate on.

~~Goodbye Apple Notes. You served your purpose well.~~

On second thought, Apple notes does a lot of good stuff! What I can mostly use this for is keeping track of recommendations.
I find it difficult to keep a list of recommendations and sort through them. This will help me deal with that.

## Usage
```
# Run the application
`cargo run`
```


## TODO
- Implement Lists
- Async Storage?


## Setup Heroku DB
```
$ export DATABASE_URL=`heroku pg:credentials:url DATABASE -a typed-thoughts | grep "://" | xargs`
$ diesel migration run
```