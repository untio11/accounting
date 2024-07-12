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
     - [summary: amount of transactions]
	   - [summary: transaction node frequencies]
     - [summary: cash flow per node]

## Profile stuff

## Application state
- (Profile not selected) | (Profile P selected)
- (Transactions not imported) | (Transactions available)
