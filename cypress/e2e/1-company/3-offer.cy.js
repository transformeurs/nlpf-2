/// <reference types="cypress" />

describe("offer", () => {
  beforeEach(() => {
    cy.visit("/");
    cy.wait(1000);
    cy.get("#companyRole").check();
    cy.get("#email").type("pchojka@weldr-conseil.fr");
    cy.get("#password").type("123456");
    cy.get(":nth-child(4) > .w-full").click();
  })

  it("create a new offer", () => {
    cy.wait(2000);
    cy.get(".relative > a > .border").click();
    cy.wait(2000);
    
    cy.get(".space-y-6 > :nth-child(1) > .block").type("Architecte Data");
    cy.get(":nth-child(2) > .block").type("Verticaux : Retail, Média, Industrie, Marketing\nExpertises : Problématiques Data, Processus de développement logiciel, Intégration applicative");
    cy.get(":nth-child(3) > .block").type("Programmation, Data, Architecture");
    cy.get(":nth-child(4) > .block").type("Paris");
    cy.get(":nth-child(5) > .block").type("48 000 € brut annuel");
    cy.get(":nth-child(6) > .block").type("CDI");
    cy.get(":nth-child(7) > .block").type("Février 2023");
    cy.get(":nth-child(9) > .block").select(1);
    cy.get(".w-64").click();
    cy.wait(2000);
    cy.get("[href='/offers']").click();
    cy.wait(5000);

    cy.get(":nth-child(1) > .bg-white > .flex-col > .space-y-1 > .w-full").click();
    cy.wait(5000);
    cy.scrollTo("bottom");
  })
})
