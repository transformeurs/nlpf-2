/// <reference types="cypress" />

describe("questionnaire", () => {
  beforeEach(() => {
    cy.visit("/");
    cy.wait(1000);
    cy.get("#companyRole").check();
    cy.get("#email").type("pchojka@weldr-conseil.fr");
    cy.get("#password").type("123456");
    cy.get(":nth-child(4) > .w-full").click();
  })

  it("create a new questionnaire", () => {
    cy.wait(2000);
    cy.get("[href='/questionnaires']").click();
    cy.wait(2000);
    cy.get(".relative > a > .border").click(); // Click on the "Créer un questionnaire" button
    cy.wait(2000);
    cy.get("#name").type("Questionnaire architecte");
    cy.get(".flex > .mt-5").click(); // Click on the "Ajouter une question" button
    cy.get(".flex > #question-0").type("Quel type d'intégration de solutions considérons-nous comme primordial ?");
    cy.get(".w-64").click(); // Click on the "Ajouter une option" button
    cy.get("#answer-0-0").type("Driven-driven");
    cy.get(".w-64").click(); // Click on the "Ajouter une option" button
    cy.get("#answer-0-1").type("Data-driven");
    cy.get(".w-64").click(); // Click on the "Ajouter une option" button
    cy.get("#answer-0-2").type("Money-driven");
    cy.get("#validity-0-1").check();

    cy.get(".flex > .mt-5").click(); // Click on the "Ajouter une question" button
    cy.get(".flex > #question-1").type("En quelle année a été fondé Weldr Conseil ?");
    cy.get(":nth-child(2) > .flex > .w-64").click(); // Click on the "Ajouter une option" button
    cy.get("#answer-1-0").type("2010");
    cy.get(":nth-child(2) > .flex > .w-64").click(); // Click on the "Ajouter une option" button
    cy.get("#answer-1-1").type("2020");
    cy.get(":nth-child(2) > .flex > .w-64").click(); // Click on the "Ajouter une option" button
    cy.get("#answer-1-2").type("2022");
    cy.get(":nth-child(2) > .flex > .w-64").click(); // Click on the "Ajouter une option" button
    cy.get("#answer-1-3").type("2027");
    cy.get("#validity-1-2").check();
    cy.wait(6000);
    cy.get(".space-y-12 > :nth-child(4) > .w-full").click();
    cy.wait(2000);
    cy.get(":nth-child(1) > .mt-4 > summary").click();
  })
})
