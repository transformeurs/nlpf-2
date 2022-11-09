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

## Offer

```
CREATE (o:Offer {
    title : "Google offer",
    description : "Get the best job ever",
    created_at : date("2022-08-12"),
    skills : ["Beau", "Intelligent", "Puissant"],
    location : "The Moon",
    salary : 69420,
    job_duration : "1 year and 4 mounths",
    job_start : date("2023-10-04"),
}) RETURN c
```

Link :
```
CREATE (o)-[:POSTED]->(company)
```