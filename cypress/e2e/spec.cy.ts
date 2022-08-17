describe('Navigation', () => {
  it('should navigate to the index page', () => {
    // Start from the index page
    cy.visit('http://localhost:3000/')

    // check exists sign-in button
    cy.get('button#signin').contains('Log in');
    cy.get('button#signin').click();

    // cypress url() is one of command : https://docs.cypress.io/api/commands/url#Arguments
    // should 1st argument is chainer that according to BDD Assertion list(https://docs.cypress.io/guides/references/assertions#BDD-Assertions)
    cy.url().should('to.have.string', 'https://wallet.testnet.near.org');
  })
})
