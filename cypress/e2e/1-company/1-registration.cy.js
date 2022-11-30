/// <reference types="cypress" />

describe('company', () => {
  beforeEach(() => {
    cy.visit("/");
  })

  it('create a new account', () => {
    cy.wait(1000);

    // Click on the "S'inscrire" button
    cy.get("[href='/signup']").click();

    cy.wait(1000);

    cy.get("#companyRole").check();
    cy.get("#username").type("Weldr Conseil");
    cy.get("#email").type("pchojka@weldr-conseil.fr");
    cy.get("#password").type("123456");
    cy.get("#description").type("L’architecture SI à la hauteur de vos ambitions. Pour répondre aux besoins actuels et futurs, Weldr Conseil accompagne ses clients dans la définition et la mise en place de stratégies et solutions innovantes adaptées à leur échelle.")
    cy.get("#fileinput").selectFile("cypress/e2e/1-company/logo_weldr.png");

    cy.wait(1000);

    // Click on the "S'inscrire" button
    cy.get(":nth-child(8) > .w-full").click();
  })

  // it('displays two todo items by default', () => {
  //   cy.get('.todo-list li').should('have.length', 2)

  //   cy.get('.todo-list li').first().should('have.text', 'Pay electric bill')
  //   cy.get('.todo-list li').last().should('have.text', 'Walk the dog')
  // })

  // it('can add new todo items', () => {
  //   const newItem = 'Feed the cat'

  //   cy.get('[data-test=new-todo]').type(`${newItem}{enter}`)

  //   cy.get('.todo-list li')
  //     .should('have.length', 3)
  //     .last()
  //     .should('have.text', newItem)
  // })

  // it('can check off an item as completed', () => {
  //   cy.contains('Pay electric bill')
  //     .parent()
  //     .find('input[type=checkbox]')
  //     .check()

  //   cy.contains('Pay electric bill')
  //     .parents('li')
  //     .should('have.class', 'completed')
  // })

  // context('with a checked task', () => {
  //   beforeEach(() => {
  //     cy.contains('Pay electric bill')
  //       .parent()
  //       .find('input[type=checkbox]')
  //       .check()
  //   })

  //   it('can filter for uncompleted tasks', () => {
  //     cy.contains('Active').click()

  //     cy.get('.todo-list li')
  //       .should('have.length', 1)
  //       .first()
  //       .should('have.text', 'Walk the dog')

  //     cy.contains('Pay electric bill').should('not.exist')
  //   })

  //   it('can filter for completed tasks', () => {
  //     cy.contains('Completed').click()

  //     cy.get('.todo-list li')
  //       .should('have.length', 1)
  //       .first()
  //       .should('have.text', 'Pay electric bill')

  //     cy.contains('Walk the dog').should('not.exist')
  //   })

  //   it('can delete all completed tasks', () => {
  //     cy.contains('Clear completed').click()

  //     cy.get('.todo-list li')
  //       .should('have.length', 1)
  //       .should('not.have.text', 'Pay electric bill')

  //     cy.contains('Clear completed').should('not.exist')
  //   })
  // })
})
