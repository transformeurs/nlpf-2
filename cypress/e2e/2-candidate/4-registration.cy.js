/// <reference types="cypress" />

describe('candidate', () => {
  beforeEach(() => {
    cy.visit("/");
  })

  it('create a new account', () => {
    cy.wait(1000);

    // Click on the "S'inscrire" button
    cy.get("[href='/signup']").click();

    cy.wait(1000);

    cy.get("#username").type("Robin Duval");
    cy.get("#email").type("leyoda@gmail.com");
    cy.get("#password").type("123456");
    cy.get("#age").type("74");
    cy.get("#description").type("Coach Agile en reconversion");
    cy.get("#fileinput").selectFile("cypress/e2e/2-candidate/robin_photo.jpg");

    cy.wait(1000);

    // // Click on the "S'inscrire" button
    cy.get(":nth-child(8) > .w-full").click();
  })
})
