# Googol Translate
![grugtranslate_smaller](https://github.com/johanyim/googol_translate/assets/37012949/6e398e76-1323-4f89-a919-fe7c756e5312)

## What is Googol Translate?

This is a full stack web application which imitates the lesser-known competitor application, [Google Translate](https://translate.google.com/). You can use the API endpoint to generate your own translations. 

The translation results are **deterministic**, like a real translation dictionary. The same inputs will result in the same outputs[^1].

[^1]: Results can vary between different releases

Note: For purposes of operating with HTMX, the response is plain text, with formatted HTML hypermedia on errors.

## Try it yourself

#### Website: [`googoltranslate.com`](http://googoltranslate.com) 
#### API endpoint: [`api.googoltranslate.com/translate`](https://api.googoltranslate.com) 

## Example usage

`json` Payload
```json
{
    "text": "We are just an advanced breed of monkeys on a minor planet of a very average star. But we can understand the Universe. That makes us something very special.",
    "voice": "Caveman"
}
```

With `cURL`
```bash
foo@bar:~$ curl -X POST -d \
'{
    "text": "We are just an advanced breed of monkeys on a minor planet of a very average star. But we can understand the Universe. That makes us something very special.",
    "voice": "Caveman"
}' \
-X POST "https://api.googoltranslate.com/translator"
```

```
We advanced monkey type on small rock of normal fire ball. But we know big everything. That make us very special
```

## Funny results 


## Technologies used
#### Frontend: `HTMX`, `HTML`, `Sass`

#### Backend: `Rust`, `Cargo Lambda`, 

#### Deployment: `AWS`, `Lambda`, `API Gateway`, `EC2`, `Route53`, `ACM`, `GitHub Workflows`
