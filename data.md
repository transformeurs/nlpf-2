## Users

```
CREATE (c:Candidate {
    name: "Tom Marquaille",
    email: "tom@tom.fr",
    password: "toto",
    age: 12,
    photoUrl: "...",
    description: "dev"
}) RETURN c
```

## Companies

```
CREATE (c:Company {
      name: "EPITA",
      email: "..",
      password: "toto",
      description: "",
      photoUrl: "toto"
}) RETURN c
```

## Offers

```
CREATE (o:Offer {
    title: "Stage",
    description: "Stage de 6 mois",
    photoUrl: "toto",
    salary: 1000,
    location: "Paris",
    company: "EPITA"
}) RETURN o
```


## Create relationship between offer and candidate

```
MATCH (c:Candidate)
WITH c
MATCH (o:Offer)
WHERE c.name = "Tom Marquaille" AND o.title = "Stage"
CREATE (c)-[:CANDIDATE_TO {
    candidate_id: 1,
    offer_id: 1,
    created_at: "2019-01-01",
    status: "pending",
    cover_letter_url: "toto",
    resume_url: "tata",
    custom_field: "titi"
}]->(o)
```