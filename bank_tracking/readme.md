# Bank tracking

Basically take a bank statement from the RBC online app and pass it through
this application and it spits out a monthly statement about how you spend
your money monthly. The goal is to have an interactive prompt for you to
quickly sort through all your expenses so that you actually look at the
individual expenses that you're making on the regular.

# TODO

- [x] Write interactive prompt to sort through all of the transactions
- [x] Split the transactions by month and store them
- [x] Remove duplicate transactions from newly entered statements
- [ ] Seperate the income from the rest of the expenses
- [ ] Calculate spending trends bi-weekly
    - [x] Total expenses
    - [x] Total expenses by category
    - [x] Percentage of expenses by each category of total expenses
    - [ ] Percentage of expenses by each category of total income
- [ ] Calculate accumulation of funds in additional accounts over a period of time
- [ ] Put all of these lovely statistics on a nice application front end that
      you can view in all of its glory
- [x] Have the sorting have shortcuts through the number pad
- [ ] Add runtime configurable tags for each expense through letters
