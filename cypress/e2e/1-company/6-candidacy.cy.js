/// <reference types="cypress" />

describe("candidacy", () => {
  beforeEach(() => {
    cy.visit("/");
    cy.wait(1000);
    cy.get("#companyRole").check();
    cy.get("#email").type("pchojka@weldr-conseil.fr");
    cy.get("#password").type("123456");
    cy.get(":nth-child(4) > .w-full").click();
  })

  it("check candidacies", () => {
    cy.wait(1500);
    cy.get("[href='/candidacies']").click();
    cy.wait(5000);
    cy.get(".space-y-1 > .w-full").first().click();
    cy.wait(5000);
    cy.get(".bg-green-600").click();
    cy.wait(2000);
    cy.get("[href='/candidacies']").click();
  })
})
