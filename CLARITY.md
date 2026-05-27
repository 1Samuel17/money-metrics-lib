# Project Clarity
---

## What am I trying to accomplish with this project (broadly)?
Broadly, I am trying to create a library that contains functionality found in the Quicken software.

## What am I trying to accomplish with this project (specifically)?
Specifically, I am trying to create a library with the following core functionalities found in the Quicken software:
- Add incomes
- Add obligations
- Calculate balance
- Forecast balance
- Generate report
- Integrate hypothetical scenarios

## What is motivating me to build this library?
The following factors are motivating me to build this library:
- I need a money management tool to keep track of cash flow for financial decisions.
- I liked the features of Quicken financial software, but do not want to keep paying a subscription for it.
- I do not need all the features of Quicken, and I do not need cloud sync features.
- I do not need individual transaction history
- I want to use this library in a CLI tool
- I need to become fluent in Rust programming.
- I need a project that demonstrates core Rust fluency to potenial employers

## How do I plan on building this library?
I plan on using the following plan to build this library:
- establish income sources
- establish budgets for core financial obligations
- develop calculation functionality
- develop forecasting functionality
- develop report generation functionality
- develop hypothetical functionality

## What core entity types do I need to begin building this library with the desired functionality?
I will need the following core types to begin with:
- IncomeSource (enum):
    - Employer
    - Property Rental
    - Person
- ObligationSource (enum):
    - Mortgage
    - HOA
    - Electricity
    - Gas
    - Water
    - Phone
    - Internet
    - Vehicle
    - StudentLoan
    - CreditCard
    - InstallmentPlan
    - Tithe
    - Groceries
- ScenarioType (enum):
    - Real
    - Hypothetical
- Income (struct): 
    - IncomeSource
    - Amount
    - Frequency
    - Note
- Obligation (stuct):
    - ObligationSource
    - Amount
    - Frequency
    - Note
- Scenario (struct):
    - ScenarioType (ScenarioType)
    - Incomes (vec)
    - Obligations (vec)
- Metrics (trait):
    - ratios
    - percentages
    - annualizations
    - projections
    - hypotheticals
- Frequency (enum):
    - Biweekly
    - Monthly
    - Random
        
## How will I do this while also balancing my time with my full-time job and family?
I have set up a 2hr countdown timer to work. Before beginning to work, I pray for guidance and wisdom to help me make the most of this time in order to keep the project moving forward towards completion with little or no stalls. When the time is up, I will commit my work, make notes of what is next to do on the next session and turn everything off.

## How will I maintain personal development accountability and integrity regarding AI usage?
I will pause Github Copilot for the 2hr work duration to receive no "ghost code" suggestions and only unpause for overcoming very difficult errors outside of my knowledge in order to keep development moving forward without delays.