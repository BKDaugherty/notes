# Notes
I've become increasingly frustrated by my ability to record my notes in a searchable fashion. This is a basic CRUD API for storing my notes in a way that I will like, and can programatically iterate on.

~~Goodbye Apple Notes. You served your purpose well.~~

On second thought, Apple notes does a lot of good stuff! What I've mostly used this for is keeping track of single items that I would forget otherwise. It's been most useful to track books, movies, video games, restaurants, recipes and other things I've wanted to remember. It also helps me keep track of who recommended something to me, so that I can have a conversation with them about it after I've visited a restaurant, or read the book.

## Structure

The application is largely built like a traditional entity system. Each `Note` or entity is given different `Tag`'s that sort it into a category. As an example, I might toss the `Productivity` and `Book` tags on the book "Deep Work". What makes this system especially useful though, is that the `Tag` enum takes advantage of Rust's enums, which are more like Algebraic types from Haskell or other functional programming languages than traditional enums.

```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Tag {
    // Medium Based
    Book,
    ...
    // Genre based
    Productivity,
    ...
    // Topic based
    ArtificialIntelligence,
    EffectiveAltruism,
    ...
    // Meta based
    RecommendedBy(String),
    RemindsMeOf(String),
    Origin(String),
}
```
One particularly useful example is that of recommendations. With the `RecommendedBy` `Tag`, I can associate someone with the `Note`, in a structured and unified way. I can also process these programatically! Which makes me much happier than my old system in which I'd put plain text into Apple Notes. I'm not sure if more complicated forms of Tags will arise, but I'm excited about playing around with the idea :)

Here are some screenshots from the [prototype web app](https://github.com/BKDaugherty/notes-web) I built for this which shows some of the tags in action to give you and idea of how I use it.

![example of some notes with tags in ui](docs/images/tags-example-0)
![example of some notes with tags in ui using search](docs/images/tags-example-1)
![example of some notes with tags in ui using a more complex search](docs/images/tags-example-2)

## Develop

### Usage
```
# Run the application
`cargo run`
```

### Documentation
```
# Generate and view documentation
`cargo doc --open`
```

### TODO
- Implement Lists
- Async Storage?

### Setup Heroku DB
```
$ export DATABASE_URL=`heroku pg:credentials:url DATABASE -a typed-thoughts | grep "://" | xargs`
$ diesel migration run
```