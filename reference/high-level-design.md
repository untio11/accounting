What should the architecture/API of this B look like?

Input data:
- cvs files with transactions
- user profile

Output result:
- Show a bunch of insights about the transaction to the user

How do we get from the input to the output?
1. Application starts
2. User is prompted to select/create profile
   - [profile selection]
   - [profile creation]
   - [profile editing]
3. User is prompted to select which .csv files to import.
   - [import selection]
4. User is presented a dashboard from which to analyze the data
   - [dashboard design]
     - [show basic profile information]
   - [analysis options]
     - [filtering: transactions between times]
     - [filtering: cash flow per node]
     - [summary: amount of transactions]
	  - [summary: transaction node frequencies]
     - [summary: cash flow per node]

## Analysis pipeline
First narrow down the set of transactions to operate on, then perform the analysis.
Good idea to put it on `Transactions` struct? Struct keeps ownership, provide iterators over subsets of transactions

## Profile stuff

## Application state
- (Profile not selected) | (Profile P selected)
- (Transactions not imported) | (Transactions available)
