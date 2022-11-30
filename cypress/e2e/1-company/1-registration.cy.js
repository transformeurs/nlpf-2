/// <reference types="cypress" />

describe('company', () => {
  beforeEach(() => {
    cy.visit("/");
  })

  it('create a new account', () => {
    cy.wait(3000);

    // Click on the "S'inscrire" button
    cy.get("[href='/signup']").click();

    cy.wait(1000);

    cy.get("#companyRole").check();
    cy.get("#username").type("Weldr Conseil");
    cy.get("#email").type("pchojka@weldr-conseil.fr");
    cy.get("#password").type("123456");
    cy.get("#description").type("L’architecture SI à la hauteur de vos ambitions. Pour répondre aux besoins actuels et futurs, Weldr Conseil accompagne ses clients dans la définition et la mise en place de stratégies et solutions innovantes adaptées à leur échelle.")
    cy.get("#fileinput").selectFile("cypress/e2e/1-company/logo_weldr.png");

    cy.wait(4000);

    // Click on the "S'inscrire" button
    cy.get(":nth-child(8) > .w-full").click();
  })
})
