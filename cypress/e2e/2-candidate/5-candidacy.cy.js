/// <reference types="cypress" />

describe("candidacy", () => {
  beforeEach(() => {
    cy.visit("/");
    cy.wait(1000);
    cy.get("#email").type("leyoda@gmail.com");
    cy.get("#password").type("123456");
    cy.get(":nth-child(4) > .w-full").click();
  })

  it("create a candidacy", () => {
    cy.wait(5000);
    cy.get(":nth-child(1) > .bg-white > .flex-col > .space-y-1 > .w-full").click();
    cy.wait(5000);
    cy.get(".w-full").click();
    cy.wait(2000);
    cy.get("#resume_url").selectFile("cypress/e2e/2-candidate/resume.pdf");
    cy.get("#cover_letter_url").selectFile("cypress/e2e/2-candidate/cover_letter.pdf");
    cy.get(".relative > .block").type("Je suis grave motivé, no noob no arnak");
    cy.get(":nth-child(1) > .flex > :nth-child(1) > input").check();
    cy.get(":nth-child(2) > .flex > :nth-child(1) > input").check();
    cy.wait(3000);
    cy.get(".w-64").click();
    cy.wait(2000);
    cy.get("[href='/candidacies']").click();
    cy.wait(5000);
    cy.get(".space-y-1 > .w-full").first().click();
  })
})
